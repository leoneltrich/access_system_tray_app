import { writable, get } from 'svelte/store';
import { fetch } from '@tauri-apps/plugin-http';
import { db } from './app-db';
import { serverUrl } from './settings';

const KEY_SAVED_SERVERS = 'saved_servers';
const KEY_JWT = 'auth_token';

// --- TYPES ---

interface AccessStatusResponse {
    server: string;
    ip: string;
    is_active: boolean;
    expiration: number | null;
    time_remaining: string | null;
}

export interface ServerCard {
    id: string;
    status: 'idle' | 'access-granted' | 'offline' | 'error'; // Added 'error' for clearer UI states
    timeRemaining?: string | null;
}

// --- STATE ---
export const servers = writable<ServerCard[]>([]);
// serverError can be removed if you handle errors locally in the UI,
// but we keep it if you use it for global alerts.
export const serverError = writable<string>("");

// --- HELPERS ---

/**
 * Centralized error thrower based on HTTP status
 */
async function handleResponseError(response: Response) {
    if (response.ok) return; // Should not be called if OK

    // You can also parse the body if your backend sends JSON error details:
    // const body = await response.json().catch(() => null);

    switch (response.status) {
        case 401: throw new Error("ERR_AUTH"); // Unauthorized
        case 403: throw new Error("ERR_FORBIDDEN"); // Access Denied
        case 404: throw new Error("ERR_NOT_FOUND"); // Server Deleted/Missing
        case 500: throw new Error("ERR_SERVER"); // Internal Error
        case 502:
        case 503: throw new Error("ERR_OFFLINE"); // Gateway/Service unavailable
        default: throw new Error(`ERR_UNKNOWN:${response.status}`);
    }
}

// --- ACTIONS ---

export async function loadServers() {
    try {
        const stored = await db.get<ServerCard[]>(KEY_SAVED_SERVERS);
        if (stored) {
            const safeList = stored.map(s => ({
                ...s,
                status: 'idle' as const, // Always start neutral
                timeRemaining: null      // Clear stale timers
            }));

            servers.set(safeList);
        }
    } catch (err) {
        console.error("Failed to load servers:", err);
    }
}

/**
 * Checks a SINGLE server's status and updates the store.
 * NOW HANDLES OFFLINE/STALE STATES CORRECTLY.
 */
export async function checkServerStatus(serverName: string) {
    const token = await db.get<string>(KEY_JWT);
    // If no token, we can't really check status, but we shouldn't show 'access-granted'
    if (!token) {
        updateServerStatus(serverName, 'idle');
        return;
    }

    const baseUrl = get(serverUrl);
    // Safety check: if no URL, can't check
    if (!baseUrl) {
        updateServerStatus(serverName, 'offline');
        return;
    }

    const cleanUrl = baseUrl.replace(/\/$/, "");

    try {
        const response = await fetch(`${cleanUrl}/users/access/${serverName}/status`, {
            method: 'GET',
            headers: {
                'Authorization': `Bearer ${token}`,
                'Content-Type': 'application/json'
            }
        });

        if (response.status === 401) {
            // Token expired. We effectively have no access.
            // You might want to trigger a logout flow here eventually.
            console.warn("Token expired during polling");
            updateServerStatus(serverName, 'idle');
            return;
        }

        if (!response.ok) {
            // If the server returns 404/500, we consider the connection "alive" but the server specific status "error"
            // For simplicity, we fallback to 'idle' or 'error' so the user sees they aren't connected.
            updateServerStatus(serverName, 'idle'); // Or 'error' if you want a red badge
            return;
        }

        const data: AccessStatusResponse = await response.json();

        // SUCCESS: Update with real data
        servers.update(list => list.map(s => {
            if (s.id === serverName) {
                return {
                    ...s,
                    status: data.is_active ? 'access-granted' : 'idle',
                    timeRemaining: data.time_remaining
                };
            }
            return s;
        }));

    } catch (err) {
        console.error(`Failed to check status for ${serverName}:`, err);
        // CRITICAL FIX: If fetch throws (Network Error), MARK AS OFFLINE.
        // This ensures the UI updates to show the backend is down,
        // removing the stale "Active" green dot.
        updateServerStatus(serverName, 'offline');
    }
}

/**
 * Helper to check ALL servers at once
 */
export async function syncAllStatuses() {
    const currentList = get(servers);
    await Promise.all(currentList.map(s => checkServerStatus(s.id)));
}

export async function addServer(serverName: string) {
    // Reset any global error
    serverError.set("");

    const currentList = get(servers);
    if (currentList.some(s => s.id === serverName)) throw new Error("ERR_DUPLICATE");

    const token = await db.get<string>(KEY_JWT);
    if (!token) throw new Error("ERR_AUTH");

    const baseUrl = get(serverUrl);
    const cleanUrl = baseUrl.replace(/\/$/, "");

    try {
        const response = await fetch(`${cleanUrl}/users/servers/${serverName}/exists`, {
            method: 'GET',
            headers: { 'Authorization': `Bearer ${token}`, 'Content-Type': 'application/json' }
        });

        await handleResponseError(response); // Will throw specific codes if not OK

        const data = await response.json();
        if (!data.exists) throw new Error("ERR_NOT_FOUND");

        const newServer: ServerCard = { id: serverName, status: 'idle' };
        servers.update(list => {
            const updated = [...list, newServer];
            saveToDisk(updated);
            return updated;
        });
    } catch (err: any) {
        // If it's already one of our codes, rethrow. If it's a fetch error, map to network.
        const msg = err.message || "";
        if (msg.startsWith("ERR_")) throw err;
        throw new Error("ERR_NETWORK");
    }
}

export async function requestAccess(serverName: string) {
    const token = await db.get<string>(KEY_JWT);
    if (!token) throw new Error("ERR_AUTH");

    const baseUrl = get(serverUrl);
    const cleanUrl = baseUrl.replace(/\/$/, "");

    try {
        const response = await fetch(`${cleanUrl}/users/access`, {
            method: 'POST',
            headers: { 'Authorization': `Bearer ${token}`, 'Content-Type': 'application/json' },
            body: JSON.stringify({ server_id: serverName })
        });

        // --- THE FIX ---
        // Instead of generic "Request Failed", we check codes
        await handleResponseError(response);

        // Optimistic update
        updateServerStatus(serverName, 'access-granted');

        // Immediately fetch accurate time remaining to sync up
        await checkServerStatus(serverName);

    } catch (err: any) {
        // Map native fetch errors (like "Failed to fetch") to our code
        const msg = err.message || "";
        if (msg.startsWith("ERR_")) throw err; // Already formatted
        throw new Error("ERR_NETWORK"); // Network/Connection refused
    }
}

export async function removeServer(serverName: string) {
    servers.update(list => {
        const updated = list.filter(s => s.id !== serverName);
        saveToDisk(updated);
        return updated;
    });
}

// --- INTERNAL HELPERS ---

function updateServerStatus(id: string, status: ServerCard['status'], timeRemaining?: string | null) {
    servers.update(list => {
        const updated = list.map(s =>
            s.id === id ? { ...s, status, timeRemaining: timeRemaining ?? s.timeRemaining } : s
        );
        // We don't save to disk on status updates to save IO, only on add/remove
        return updated;
    });
}

async function saveToDisk(list: ServerCard[]) {
    await db.set(KEY_SAVED_SERVERS, list);
    await db.save();
}
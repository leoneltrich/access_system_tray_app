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
    status: 'idle' | 'access-granted' | 'offline';
    timeRemaining?: string | null; // NEW: Holds "2h 30m" etc.
}

// --- STATE ---
export const servers = writable<ServerCard[]>([]);
export const serverError = writable<string>("");

// --- ACTIONS ---

export async function loadServers() {
    try {
        const stored = await db.get<ServerCard[]>(KEY_SAVED_SERVERS);
        if (stored) {
            servers.set(stored);
        }
    } catch (err) {
        console.error("Failed to load servers:", err);
    }
}

/**
 * Checks a SINGLE server's status and updates the store.
 * Silent: Doesn't throw errors to the UI, just logs them.
 */
export async function checkServerStatus(serverName: string) {
    const token = await db.get<string>(KEY_JWT);
    if (!token) return;

    const baseUrl = get(serverUrl);
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
            console.warn("Token expired during polling");
            return;
        }

        if (!response.ok) {
            // If 404 or other error, we might assume 'offline' or just keep current state
            // For now, let's strictly handle connectivity issues vs logical issues
            return;
        }

        const data: AccessStatusResponse = await response.json();

        // Update the store based on real data
        servers.update(list => {
            const updated = list.map(s => {
                if (s.id === serverName) {
                    return {
                        ...s,
                        // If active -> granted, else -> idle
                        status: data.is_active ? ('access-granted' as const) : ('idle' as const),
                        timeRemaining: data.time_remaining
                    };
                }
                return s;
            });
            // We don't save to disk on every poll to avoid IO thrashing,
            // but you could if persistence of exact seconds matters.
            return updated;
        });

    } catch (err) {
        console.error(`Failed to check status for ${serverName}`, err);
    }
}

/**
 * Helper to check ALL servers at once
 */
export async function syncAllStatuses() {
    const currentList = get(servers);
    // Execute all checks in parallel
    await Promise.all(currentList.map(s => checkServerStatus(s.id)));
}

// ... (Rest of your existing addServer, requestAccess, removeServer functions stay the same) ...

export async function addServer(serverName: string) {
    // ... (Keep existing code) ...
    // Note: Copied from previous step, ensure this function exists here
    serverError.set("");
    const currentList = get(servers);
    if (currentList.some(s => s.id === serverName)) throw new Error("Server already added.");
    const token = await db.get<string>(KEY_JWT);
    if (!token) throw new Error("You must be logged in.");
    const baseUrl = get(serverUrl);
    const cleanUrl = baseUrl.replace(/\/$/, "");

    try {
        const response = await fetch(`${cleanUrl}/users/servers/${serverName}/exists`, {
            method: 'GET',
            headers: { 'Authorization': `Bearer ${token}`, 'Content-Type': 'application/json' }
        });
        if (!response.ok) throw new Error("Server check failed.");
        const data = await response.json();
        if (!data.exists) throw new Error("Server does not exist.");

        const newServer: ServerCard = { id: serverName, status: 'idle' };
        servers.update(list => {
            const updated = [...list, newServer];
            saveToDisk(updated);
            return updated;
        });
    } catch (err: any) {
        throw new Error(err.message);
    }
}

export async function requestAccess(serverName: string) {
    // ... (Keep existing code) ...
    const token = await db.get<string>(KEY_JWT);
    if (!token) throw new Error("You must be logged in.");
    const baseUrl = get(serverUrl);
    const cleanUrl = baseUrl.replace(/\/$/, "");

    try {
        const response = await fetch(`${cleanUrl}/users/access`, {
            method: 'POST',
            headers: { 'Authorization': `Bearer ${token}`, 'Content-Type': 'application/json' },
            body: JSON.stringify({ server_id: serverName })
        });
        if (!response.ok) throw new Error("Request failed.");

        // Optimistic update
        servers.update(list => {
            const updated = list.map(s =>
                s.id === serverName ? { ...s, status: 'access-granted' as const } : s
            );
            saveToDisk(updated);
            return updated;
        });

        // Immediately fetch accurate time remaining
        await checkServerStatus(serverName);

    } catch (err: any) {
        throw err;
    }
}

export async function removeServer(serverName: string) {
    servers.update(list => {
        const updated = list.filter(s => s.id !== serverName);
        saveToDisk(updated);
        return updated;
    });
}

async function saveToDisk(list: ServerCard[]) {
    await db.set(KEY_SAVED_SERVERS, list);
    await db.save();
}
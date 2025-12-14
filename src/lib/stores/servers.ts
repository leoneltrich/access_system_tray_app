import { writable, get } from 'svelte/store';
import { fetch } from '@tauri-apps/plugin-http';
import { db } from './app-db';
import { serverUrl } from './settings';

const KEY_SAVED_SERVERS = 'saved_servers';
const KEY_JWT = 'auth_token';

export interface ServerCard {
    id: string;
    status: 'idle' | 'access-granted' | 'offline';
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

export async function addServer(serverName: string) {
    serverError.set("");

    // 1. Local Duplicate Check
    const currentList = get(servers);
    if (currentList.some(s => s.id === serverName)) {
        throw new Error("Server already added.");
    }

    // 2. Prepare API
    const token = await db.get<string>(KEY_JWT);
    if (!token) throw new Error("You must be logged in.");

    const baseUrl = get(serverUrl);
    const cleanUrl = baseUrl.replace(/\/$/, "");

    // 3. Check Existence
    try {
        const response = await fetch(`${cleanUrl}/users/servers/${serverName}/exists`, {
            method: 'GET',
            headers: {
                'Authorization': `Bearer ${token}`,
                'Content-Type': 'application/json'
            }
        });

        if (response.status === 401) throw new Error("Unauthorized. Please log in again.");
        if (!response.ok) throw new Error(`API Error: ${response.status}`);

        const data = await response.json();
        if (!data.exists) throw new Error("Server does not exist.");

        // 4. Success - Add to list
        const newServer: ServerCard = { id: serverName, status: 'idle' };

        servers.update(list => {
            const updated = [...list, newServer];
            saveToDisk(updated);
            return updated;
        });

    } catch (err: any) {
        console.error("Add Server Failed:", err);
        throw new Error(err.message || "Failed to connect to server.");
    }
}

/**
 * NEW: Requests access for a specific server IP/ID
 */
export async function requestAccess(serverName: string) {
    const token = await db.get<string>(KEY_JWT);
    if (!token) throw new Error("You must be logged in.");

    const baseUrl = get(serverUrl);
    const cleanUrl = baseUrl.replace(/\/$/, "");

    try {
        const response = await fetch(`${cleanUrl}/users/access`, {
            method: 'POST',
            headers: {
                'Authorization': `Bearer ${token}`,
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ server_id: serverName })
        });

        if (response.status === 401) throw new Error("Session expired.");
        if (!response.ok) {
            const errData = await response.json().catch(() => ({}));
            throw new Error(errData.message || `Error ${response.status}`);
        }

        // Success: Update status to 'access-granted'
        servers.update(list => {
            const updated = list.map(s =>
                s.id === serverName
                    // Add 'as const' so TS knows this is the specific literal type
                    ? { ...s, status: 'access-granted' as const }
                    : s
            );
            saveToDisk(updated);
            return updated;
        });

    } catch (err: any) {
        console.error("Access Request Failed:", err);
        throw err; // Re-throw so UI can show error
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
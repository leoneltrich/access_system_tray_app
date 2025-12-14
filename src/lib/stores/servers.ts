import { writable, get } from 'svelte/store';
import { fetch } from '@tauri-apps/plugin-http';
import { db } from './app-db';
import { serverUrl } from './settings';

const KEY_SAVED_SERVERS = 'saved_servers';
const KEY_JWT = 'auth_token';

export interface ServerCard {
    id: string;      // The servername (e.g., "media-server-01")
    status: 'idle' | 'access-granted' | 'offline'; // Local status
}

// --- STATE ---
export const servers = writable<ServerCard[]>([]);
export const serverError = writable<string>("");

// --- ACTIONS ---

/**
 * Load servers from disk on app start
 */
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
 * Validates existence via API, then adds to list.
 */
export async function addServer(serverName: string) {
    serverError.set("");

    // 1. Basic duplicate check (Local)
    const currentList = get(servers);
    if (currentList.some(s => s.id === serverName)) {
        throw new Error("Server already added.");
    }

    // 2. Get Dependencies (Token & URL)
    const token = await db.get<string>(KEY_JWT);
    if (!token) throw new Error("You must be logged in to add servers.");

    const baseUrl = get(serverUrl);
    const cleanUrl = baseUrl.replace(/\/$/, "");

    // 3. Check API
    try {
        const response = await fetch(`${cleanUrl}/users/servers/${serverName}/exists`, {
            method: 'GET',
            headers: {
                'Authorization': `Bearer ${token}`,
                'Content-Type': 'application/json'
            }
        });

        if (response.status === 401 || response.status === 403) {
            throw new Error("Unauthorized. Check your login.");
        }

        if (!response.ok) {
            throw new Error(`API Error: ${response.status}`);
        }

        const data = await response.json(); // returns { exists: boolean }

        if (!data.exists) {
            throw new Error("Server does not exist.");
        }

        // 4. Success - Update State & Disk
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
 * Remove a server from the list
 */
export async function removeServer(serverName: string) {
    servers.update(list => {
        const updated = list.filter(s => s.id !== serverName);
        saveToDisk(updated);
        return updated;
    });
}

/**
 * Helper to save changes to settings.json
 */
async function saveToDisk(list: ServerCard[]) {
    await db.set(KEY_SAVED_SERVERS, list);
    await db.save();
}
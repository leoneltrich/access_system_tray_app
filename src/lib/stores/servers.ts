import { writable, get } from 'svelte/store';
import { db } from './app-db';
import { api } from '$lib/services/api';

const KEY_SAVED_SERVERS = 'saved_servers';

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
    status: 'idle' | 'access-granted' | 'offline' | 'error';
    timeRemaining?: string | null;
}

// --- STATE ---
export const servers = writable<ServerCard[]>([]);

// --- ACTIONS ---

export async function loadServers() {
    try {
        const stored = await db.get<ServerCard[]>(KEY_SAVED_SERVERS);
        if (stored) {
            const safeList = stored.map(s => ({
                ...s,
                status: 'idle' as const,
                timeRemaining: null
            }));
            servers.set(safeList);
        }
    } catch (err) {
        console.error("Failed to load servers:", err);
    }
}

export async function checkServerStatus(serverName: string) {
    try {
        const data = await api.get<AccessStatusResponse>(`/users/access/${serverName}/status`);

        updateServerState(serverName, {
            status: data.is_active ? 'access-granted' : 'idle',
            timeRemaining: data.time_remaining
        });

    } catch (err: any) {
        const msg = err.message || "";

        if (msg === "ERR_AUTH") {
            console.warn("Token expired during polling");
            updateServerState(serverName, { status: 'idle' });
        } else if (msg === "ERR_NETWORK" || msg === "ERR_OFFLINE") {
            updateServerState(serverName, { status: 'offline' });
        } else {
            updateServerState(serverName, { status: 'idle' });
        }
    }
}

export async function syncAllStatuses() {
    const currentList = get(servers);
    await Promise.all(currentList.map(s => checkServerStatus(s.id)));
}

export async function addServer(serverName: string) {
    const currentList = get(servers);
    if (currentList.some(s => s.id === serverName)) throw new Error("ERR_DUPLICATE");

    const data = await api.get<{ exists: boolean }>(`/users/servers/${serverName}/exists`);

    if (!data.exists) throw new Error("ERR_NOT_FOUND");

    const newServer: ServerCard = { id: serverName, status: 'idle' };

    servers.update(list => {
        const updated = [...list, newServer];
        saveToDisk(updated);
        return updated;
    });
}

export async function requestAccess(serverName: string) {

    await api.post('/users/access', { server_id: serverName });

    updateServerState(serverName, { status: 'access-granted' });

    await checkServerStatus(serverName);
}

export async function removeServer(serverName: string) {
    servers.update(list => {
        const updated = list.filter(s => s.id !== serverName);
        saveToDisk(updated);
        return updated;
    });
}

// --- INTERNAL HELPERS ---

function updateServerState(id: string, updates: Partial<ServerCard>) {
    servers.update(list => {
        return list.map(s =>
            s.id === id ? { ...s, ...updates } : s
        );
    });
}

async function saveToDisk(list: ServerCard[]) {
    await db.set(KEY_SAVED_SERVERS, list);
    await db.save();
}
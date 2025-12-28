import { get } from 'svelte/store';
import { db } from '$lib/stores/app-db';
import { api } from '$lib/services/api';
import { servers, type ServerCard } from '$lib/stores/servers';

const KEY_SAVED_SERVERS = 'saved_servers';

// Private API Response Type
interface AccessStatusResponse {
    server: string;
    ip: string;
    is_active: boolean;
    expiration: number | null;
    time_remaining: string | null;
}

export const ServerService = {

    /**
     * Loads saved servers from disk into the store
     */
    async load() {
        try {
            const stored = await db.get<ServerCard[]>(KEY_SAVED_SERVERS);
            if (stored) {
                // Reset status on load so we don't show stale "online" states
                const safeList = stored.map(s => ({
                    ...s,
                    status: 'idle' as const,
                    timeRemaining: null
                }));
                servers.set(safeList);
            }
        } catch (err) {
            console.error("[ServerService] Load failed:", err);
        }
    },

    /**
     * Checks the access status for a single server
     */
    async checkStatus(serverName: string) {
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
    },

    /**
     * Iterates through all servers in the store and updates their status
     */
    async syncAll() {
        const currentList = get(servers);
        await Promise.all(currentList.map(s => this.checkStatus(s.id)));
    },

    /**
     * Verifies a server exists and adds it to the list
     */
    async add(serverName: string) {
        const currentList = get(servers);

        if (currentList.some(s => s.id === serverName)) {
            throw new Error("ERR_DUPLICATE");
        }

        const data = await api.get<{ exists: boolean }>(`/users/servers/${serverName}/exists`);
        if (!data.exists) throw new Error("ERR_NOT_FOUND");

        const newServer: ServerCard = { id: serverName, status: 'idle' };

        servers.update(list => {
            const updated = [...list, newServer];
            saveToDisk(updated);
            return updated;
        });
    },

    /**
     * Requests access to a server
     */
    async requestAccess(serverName: string) {
        await api.post('/users/access', { server_id: serverName });

        updateServerState(serverName, { status: 'access-granted' });

        await this.checkStatus(serverName);
    },

    /**
     * Removes a server from the list
     */
    async remove(serverName: string) {
        servers.update(list => {
            const updated = list.filter(s => s.id !== serverName);
            saveToDisk(updated);
            return updated;
        });
    }
};


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
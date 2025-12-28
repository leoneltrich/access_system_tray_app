import { writable } from 'svelte/store';

export interface ServerCard {
    id: string;
    status: 'idle' | 'access-granted' | 'offline' | 'error';
    timeRemaining?: string | null;
}

export const servers = writable<ServerCard[]>([]);
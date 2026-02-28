import { writable } from 'svelte/store';

export interface Session {
    access_token: string | null;
    refresh_token: string | null;
    username: string | null;
}

export const isAuthenticated = writable<boolean>(false);
export const authLoading = writable<boolean>(false);
export const authError = writable<string>("");
export const authSession = writable<Session | null>(null);
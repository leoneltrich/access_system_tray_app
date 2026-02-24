import { writable } from 'svelte/store';

export interface Session {
    access_token: string;
    refresh_token: string;
    username: string;
}

export const isAuthenticated = writable<boolean>(false);
export const authLoading = writable<boolean>(false);
export const authError = writable<string>("");
export const authSession = writable<Session | null>(null);
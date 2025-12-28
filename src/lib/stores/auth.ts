import { writable } from 'svelte/store';

// --- STATE ---
export const isAuthenticated = writable<boolean>(false);
export const authLoading = writable<boolean>(false);
export const authError = writable<string>("");
export const authToken = writable<string | null>(null);
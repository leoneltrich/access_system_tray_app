import { writable } from 'svelte/store';

export const DEFAULT_URL = 'https://api.myapp.com';

// --- STATE ---
export const serverUrl = writable<string>(DEFAULT_URL);
export const isSettingsLoaded = writable<boolean>(false);

export const autoStartEnabled = writable<boolean>(false);
export const extensionsEnabled = writable<boolean>(false);
import { writable } from 'svelte/store';

export const DEFAULT_URL = 'https://api.myapp.com';
export const serverUrl = writable<string>(DEFAULT_URL);
export const isSettingsLoaded = writable<boolean>(false);
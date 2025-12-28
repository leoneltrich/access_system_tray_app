import { writable } from 'svelte/store';

// We re-export these from config so the UI has a single import point for "Settings Data"
export { serverUrl, isSettingsLoaded } from '$lib/stores/config';

// This store is specific to the settings module, so we define it here
export const autoStartEnabled = writable<boolean>(false);
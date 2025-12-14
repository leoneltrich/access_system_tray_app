import { LazyStore } from '@tauri-apps/plugin-store';

// A single shared instance for the whole app
export const db = new LazyStore('settings.json');
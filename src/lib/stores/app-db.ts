import { LazyStore } from '@tauri-apps/plugin-store';

export const db = new LazyStore('settings.json');
import { writable } from 'svelte/store';
import { isEnabled, enable, disable } from '@tauri-apps/plugin-autostart';
import { db } from './app-db';
import { api } from '$lib/services/api';
import { serverUrl, isSettingsLoaded, DEFAULT_URL } from '$lib/stores/config';

const KEY_SERVER_URL = 'server_url';

export const autoStartEnabled = writable<boolean>(false);

export { serverUrl, isSettingsLoaded };

export async function loadSettings() {
    try {
        const [urlVal, autoStartVal] = await Promise.all([
            db.get<string>(KEY_SERVER_URL),
            isEnabled().catch(() => false)
        ]);

        serverUrl.set(urlVal || DEFAULT_URL);
        autoStartEnabled.set(autoStartVal);

    } catch (err) {
        console.error("Failed to load settings:", err);
    } finally {
        isSettingsLoaded.set(true);
    }
}

export async function toggleAppStart(): Promise<boolean> {
    const current = await isEnabled();

    if (current) {
        await disable();
        autoStartEnabled.set(false);
        return false;
    } else {
        await enable();
        autoStartEnabled.set(true);
        return true;
    }
}

export async function updateServerUrl(candidateUrl: string) {
    // 1. Input Validation
    if (!candidateUrl.startsWith('http')) {
        throw new Error("URL must start with http:// or https://");
    }

    // 2. Health Check
    try {
        await api.checkConnection(candidateUrl);
    } catch (err: any) {
        if (err.name === 'AbortError') throw new Error("Connection timed out");
        throw new Error("Could not reach server health endpoint");
    }

    // 3. Save to Disk
    try {
        const cleanUrl = candidateUrl.replace(/\/$/, "");
        serverUrl.set(cleanUrl);
        await db.set(KEY_SERVER_URL, cleanUrl);
        await db.save();
    } catch (err) {
        console.error("FS Save failed:", err);
        throw new Error("Failed to save configuration to disk");
    }
}
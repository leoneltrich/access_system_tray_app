import { isEnabled, enable, disable } from '@tauri-apps/plugin-autostart';
import { db } from '$lib/stores/app-db';
import { api } from '$lib/services/api';

import { serverUrl, isSettingsLoaded, DEFAULT_URL } from '$lib/stores/config';
import { autoStartEnabled } from '$lib/stores/settings';

const KEY_SERVER_URL = 'server_url';

export const SettingsService = {

    /**
     * Loads all settings from Disk and OS
     * Updates: serverUrl, autoStartEnabled, isSettingsLoaded
     */
    async load() {
        try {
            const [urlVal, autoStartVal] = await Promise.all([
                db.get<string>(KEY_SERVER_URL),
                isEnabled().catch(() => false) // Fail safe if plugin missing
            ]);

            serverUrl.set(urlVal || DEFAULT_URL);
            autoStartEnabled.set(autoStartVal);

        } catch (err) {
            console.error("[SettingsService] Load failed:", err);
        } finally {
            isSettingsLoaded.set(true);
        }
    },

    /**
     * Toggles the OS Autostart behavior
     * Updates: autoStartEnabled
     */
    async toggleAutoStart(): Promise<boolean> {
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
    },

    /**
     * Validates, Checks Connection, and Saves the new URL
     * Updates: serverUrl, DB
     */
    async updateServerUrl(candidateUrl: string) {
        // 1. Input Validation
        if (!candidateUrl.startsWith('http')) {
            throw new Error("URL must start with http:// or https://");
        }

        // 2. Health Check (Network)
        try {
            await api.checkConnection(candidateUrl);
        } catch (err: any) {
            const msg = err.name === 'AbortError' ? "Connection timed out" : "Could not reach server health endpoint";
            throw new Error(msg);
        }

        // 3. Persist (Disk & State)
        try {
            const cleanUrl = candidateUrl.replace(/\/$/, "");

            serverUrl.set(cleanUrl);

            await db.set(KEY_SERVER_URL, cleanUrl);
            await db.save();
        } catch (err) {
            console.error("[SettingsService] Save failed:", err);
            throw new Error("Failed to save configuration to disk");
        }
    }
};
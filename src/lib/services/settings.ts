import { isEnabled, enable, disable } from '@tauri-apps/plugin-autostart';
import { invoke } from '@tauri-apps/api/core';
import { db } from '$lib/stores/app-db';
import { api } from '$lib/services/api';

import {
    serverUrl,
    isSettingsLoaded,
    autoStartEnabled,
    DEFAULT_URL, extensionsEnabled
} from '$lib/stores/settings';

const KEY_SERVER_URL = 'server_url';
const KEY_EXTENSIONS_ENABLED = 'extensions_enabled';

export const SettingsService = {

    /**
     * Loads all settings from Disk and OS
     * Updates: serverUrl, autoStartEnabled, isSettingsLoaded
     */
    async load() {
        try {
            const [urlVal, autoStartVal, extensionsVal] = await Promise.all([
                db.get<string>(KEY_SERVER_URL),
                isEnabled().catch(() => false),
                db.get<boolean>(KEY_EXTENSIONS_ENABLED)
            ]);

            serverUrl.set(urlVal || DEFAULT_URL);
            autoStartEnabled.set(!!autoStartVal);
            extensionsEnabled.set(!!extensionsVal);

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
            await invoke('fix_autostart_path');
            autoStartEnabled.set(true);
            return true;
        }
    },

    /**
     * Toggles Extensions
     * Updates: extensionsEnabled
     */
    async toggleExtensions(): Promise<boolean> {
        try {
            let current = false;
            const unsubscribe = extensionsEnabled.subscribe(v => current = v);
            unsubscribe();

            const newValue = !current;

            await db.set(KEY_EXTENSIONS_ENABLED, newValue);
            await db.save();

            extensionsEnabled.set(newValue);
            return newValue;
        } catch (err) {
            console.error("[SettingsService] Toggle failed:", err);
            return false;
        }
    },

    /**
     * Validates, Checks Connection, and Saves the new URL
     * Updates: serverUrl, DB
     */
    async updateServerUrl(candidateUrl: string) {
        // Input Validation
        if (!candidateUrl.startsWith('http')) {
            throw new Error("URL must start with http:// or https://");
        }

        // Health Check (Network)
        try {
            await api.checkConnection(candidateUrl);
        } catch (err: any) {
            const msg = err.name === 'AbortError' ? "Connection timed out" : "Could not reach server health endpoint";
            throw new Error(msg);
        }

        // Persist (Disk & State)
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
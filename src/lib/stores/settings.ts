// src/lib/stores/settings.ts
import { writable, get } from 'svelte/store';
import { fetch } from '@tauri-apps/plugin-http';
import { db } from './app-db'; // <--- We now use the shared DB instance

const KEY_SERVER_URL = 'server_url';
const DEFAULT_URL = 'https://api.myapp.com';

// --- STATE ---
export const serverUrl = writable<string>(DEFAULT_URL);
export const isLoading = writable<boolean>(true);
export const saveStatus = writable<'idle' | 'checking' | 'saving' | 'success' | 'error' | 'invalid'>('idle');
export const statusMessage = writable<string>("");

// --- ACTIONS ---

export async function loadSettings() {
    isLoading.set(true);
    try {
        // CHANGED: storePlugin -> db
        const val = await db.get<string>(KEY_SERVER_URL);
        if (val) {
            serverUrl.set(val);
        } else {
            serverUrl.set(DEFAULT_URL);
        }
    } catch (err) {
        console.error("Failed to load settings:", err);
    } finally {
        isLoading.set(false);
    }
}

export async function saveSettings() {
    const currentUrl = get(serverUrl);

    // 1. Input Validation
    if (!currentUrl.startsWith('http')) {
        saveStatus.set('invalid');
        statusMessage.set("URL must start with http:// or https://");
        setTimeout(() => saveStatus.set('idle'), 3000);
        return;
    }

    // 2. Health Check
    saveStatus.set('checking');
    statusMessage.set("");

    try {
        const cleanUrl = currentUrl.replace(/\/$/, "");
        const healthEndpoint = `${cleanUrl}/health`;

        console.log(`Checking health at: ${healthEndpoint}`);

        const controller = new AbortController();
        const timeoutId = setTimeout(() => controller.abort(), 5000);

        const response = await fetch(healthEndpoint, {
            method: 'GET',
            signal: controller.signal
        });

        clearTimeout(timeoutId);

        if (!response.ok) {
            throw new Error(`Server returned ${response.status}`);
        }

        // 3. Save to Disk
        saveStatus.set('saving');

        // CHANGED: storePlugin -> db
        await db.set(KEY_SERVER_URL, currentUrl);
        await db.save(); // Force write to disk

        saveStatus.set('success');
        setTimeout(() => saveStatus.set('idle'), 2000);

    } catch (err: any) {
        console.error("Validation failed:", err);

        saveStatus.set('invalid');

        if (err.name === 'AbortError') {
            statusMessage.set("Connection timed out");
        } else {
            statusMessage.set("Could not reach server health endpoint");
        }

        setTimeout(() => {
            saveStatus.set('idle');
            statusMessage.set("");
        }, 3000);
    }
}
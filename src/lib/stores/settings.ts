// src/lib/stores/settings.ts
import { writable, get } from 'svelte/store';
import { LazyStore } from '@tauri-apps/plugin-store';
import { fetch } from '@tauri-apps/plugin-http'; // Optional: Use Tauri's HTTP plugin if needed, or standard fetch

const STORE_FILENAME = 'settings.json';
const KEY_SERVER_URL = 'server_url';
const DEFAULT_URL = 'https://api.myapp.com';

// --- STATE ---
export const serverUrl = writable<string>(DEFAULT_URL);
export const isLoading = writable<boolean>(true);

// Expanded status types to include "checking" and "invalid"
export const saveStatus = writable<'idle' | 'checking' | 'saving' | 'success' | 'error' | 'invalid'>('idle');

// This message will hold the specific error for the UI (e.g., "404 Not Found" or "Network Error")
export const statusMessage = writable<string>("");

const storePlugin = new LazyStore(STORE_FILENAME);

// --- ACTIONS ---

export async function loadSettings() {
    isLoading.set(true);
    try {
        const val = await storePlugin.get<string>(KEY_SERVER_URL);
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
    statusMessage.set(""); // Clear previous messages

    try {
        // Construct the health endpoint. Remove trailing slash if present to avoid double slashes.
        const cleanUrl = currentUrl.replace(/\/$/, "");
        const healthEndpoint = `${cleanUrl}/health`;

        console.log(`Checking health at: ${healthEndpoint}`);

        // Set a timeout so the app doesn't hang forever on a bad IP
        const controller = new AbortController();
        const timeoutId = setTimeout(() => controller.abort(), 5000); // 5 second timeout

        const response = await fetch(healthEndpoint, {
            method: 'GET',
            signal: controller.signal
        });

        clearTimeout(timeoutId);

        if (!response.ok) {
            throw new Error(`Server returned ${response.status}`);
        }

        // 3. Save to Disk (Only reached if response.ok is true)
        saveStatus.set('saving');

        await storePlugin.set(KEY_SERVER_URL, currentUrl);
        await storePlugin.save();

        saveStatus.set('success');
        setTimeout(() => saveStatus.set('idle'), 2000);

    } catch (err: any) {
        console.error("Validation failed:", err);

        // Revert the store variable to the LAST saved value?
        // Usually, we keep the input as-is so the user can fix the typo,
        // but we DO NOT save it to disk.

        saveStatus.set('invalid'); // Using 'invalid' to indicate verification failure

        if (err.name === 'AbortError') {
            statusMessage.set("Connection timed out");
        } else {
            statusMessage.set("Could not reach server health endpoint");
        }

        // Reset to idle after 3 seconds so user can try again
        setTimeout(() => {
            saveStatus.set('idle');
            statusMessage.set("");
        }, 3000);
    }
}
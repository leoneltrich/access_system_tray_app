import { writable, get } from 'svelte/store';
import { fetch } from '@tauri-apps/plugin-http';
import { db } from './app-db';
import { serverUrl } from './settings';

const KEY_JWT = 'auth_token';

// --- TYPES ---
interface LoginResponse {
    token: string;
}

// --- STATE ---
export const isAuthenticated = writable<boolean>(false);
export const authLoading = writable<boolean>(false);
export const authError = writable<string>("");

// Internal timer reference for auto-logout
let logoutTimer: any = null;

// --- HELPERS ---

/**
 * Decodes the JWT payload (Base64Url) to get the expiration time.
 * We do this manually to avoid adding heavy NPM dependencies.
 */
function getJwtExpiration(token: string): number | null {
    try {
        const payloadBase64 = token.split('.')[1];
        if (!payloadBase64) return null;

        // Fix Base64Url to Base64
        const base64 = payloadBase64.replace(/-/g, '+').replace(/_/g, '/');
        const jsonPayload = decodeURIComponent(atob(base64).split('').map(c => {
            return '%' + ('00' + c.charCodeAt(0).toString(16)).slice(-2);
        }).join(''));

        const parsed = JSON.parse(jsonPayload);
        return parsed.exp || null; // 'exp' is standard JWT expiry (Unix timestamp)
    } catch (e) {
        console.error("Failed to parse JWT:", e);
        return null;
    }
}


/**
 * Initialize Auth: Load token from disk and check if valid.
 */
export async function initAuth() {
    try {
        const token = await db.get<string>(KEY_JWT);

        if (token) {
            const exp = getJwtExpiration(token);
            const now = Math.floor(Date.now() / 1000);

            if (exp && exp > now) {
                // Token is valid
                isAuthenticated.set(true);
                scheduleAutoLogout(exp);
                console.log("Session restored. Expires in:", exp - now, "seconds");
            } else {
                // Token expired while app was closed
                console.warn("Stored token is expired.");
                await logout();
            }
        }
    } catch (err) {
        console.error("Auth Init Error:", err);
    }
}

/**
 * Login Function
 */
export async function login(username: string, pass: string) {
    authLoading.set(true);
    authError.set("");

    try {
        const baseUrl = get(serverUrl);
        // Remove trailing slash if present
        const cleanUrl = baseUrl.replace(/\/$/, "");

        const response = await fetch(`${cleanUrl}/login`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ username, password: pass })
        });

        if (response.status === 401) {
            throw new Error("Invalid username or password.");
        }

        if (!response.ok) {
            throw new Error(`Server Error (${response.status})`);
        }

        const data = await response.json() as LoginResponse;

        if (data.token) {
            // 1. Save to Store
            await db.set(KEY_JWT, data.token);
            await db.save();

            // 2. Update State
            isAuthenticated.set(true);

            // 3. Schedule Logout
            const exp = getJwtExpiration(data.token);
            if (exp) scheduleAutoLogout(exp);
        }

    } catch (err: any) {
        console.error("Login failed:", err);
        authError.set(err.message || "Connection to backend failed");
    } finally {
        authLoading.set(false);
    }
}

/**
 * Logout Function
 */
export async function logout() {
    if (logoutTimer) clearTimeout(logoutTimer);

    // Clear from state
    isAuthenticated.set(false);

    // Clear from disk
    await db.set(KEY_JWT, null);
    await db.save();
}

/**
 * Helper to set a timer that fires when the token expires
 */
function scheduleAutoLogout(expTimestamp: number) {
    if (logoutTimer) clearTimeout(logoutTimer);

    const now = Math.floor(Date.now() / 1000);
    const timeLeft = (expTimestamp - now) * 1000; // Convert to ms

    if (timeLeft > 0) {
        // Log out exactly when token expires
        logoutTimer = setTimeout(() => {
            console.log("Token expired. Logging out...");
            logout();
            authError.set("Session expired. Please log in again.");
        }, timeLeft);
    } else {
        logout();
    }
}
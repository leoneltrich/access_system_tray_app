import { db } from '$lib/stores/app-db';
import { api } from '$lib/services/api';
import { isAuthenticated, authLoading, authError, authToken } from '$lib/stores/auth';
import {mapBackendError} from "$lib/utils";

const KEY_JWT = 'auth_token';

// Internal timer reference for auto-logout
let logoutTimer: ReturnType<typeof setTimeout> | null = null;

interface LoginResponse {
    token: string;
}

export const AuthService = {
    /**
     * Initialize Auth: Load token from disk and check if valid.
     * Should be called in your root layout onMount.
     */
    async init() {
        try {
            const token = await db.get<string>(KEY_JWT);

            if (token) {
                const exp = getJwtExpiration(token);
                const now = Math.floor(Date.now() / 1000);

                if (exp && exp > now) {
                    // Update State
                    authToken.set(token);
                    isAuthenticated.set(true);

                    scheduleAutoLogout(exp);
                    console.log("[AuthService] Session restored.");
                } else {
                    console.warn("[AuthService] Token expired.");
                    await this.logout();
                }
            }
        } catch (err) {
            console.error("[AuthService] Init Error:", err);
        }
    },

    /**
     * Login Function
     */
    async login(username: string, pass: string) {
        authLoading.set(true);
        authError.set("");

        try {
            const data = await api.post<LoginResponse>('/login', {
                username,
                password: pass
            });

            if (data.token) {
                await db.set(KEY_JWT, data.token);
                await db.save();

                authToken.set(data.token);
                isAuthenticated.set(true);

                const exp = getJwtExpiration(data.token);
                if (exp) scheduleAutoLogout(exp);
            }

        } catch (err: any) {
            console.error("[AuthService] Login failed:", err);

            if (err.message === 'ERR_AUTH') {
                authError.set("Incorrect username or password.");
            } else {
                authError.set(mapBackendError(err));
            }

        } finally {
            authLoading.set(false);
        }
    },

    /**
     * Logout Function
     */
    async logout() {
        if (logoutTimer) clearTimeout(logoutTimer);

        isAuthenticated.set(false);
        authToken.set(null);

        await db.set(KEY_JWT, null);
        await db.save();
    },
};

function scheduleAutoLogout(expTimestamp: number) {
    if (logoutTimer) clearTimeout(logoutTimer);

    const now = Math.floor(Date.now() / 1000);
    const timeLeft = (expTimestamp - now) * 1000; // Convert to ms

    if (timeLeft > 0) {
        logoutTimer = setTimeout(() => {
            console.log("[AuthService] Token expired. Logging out...");
            AuthService.logout();
            authError.set("Session expired. Please log in again.");
        }, timeLeft);
    } else {
        AuthService.logout();
    }
}

function getJwtExpiration(token: string): number | null {
    try {
        const payloadBase64 = token.split('.')[1];
        if (!payloadBase64) return null;

        const base64 = payloadBase64.replace(/-/g, '+').replace(/_/g, '/');
        const jsonPayload = decodeURIComponent(atob(base64).split('').map(c => {
            return '%' + ('00' + c.charCodeAt(0).toString(16)).slice(-2);
        }).join(''));

        const parsed = JSON.parse(jsonPayload);
        return parsed.exp || null;
    } catch (e) {
        console.error("Failed to parse JWT:", e);
        return null;
    }
}
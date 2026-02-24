import {db} from '$lib/stores/app-db';
import {api} from '$lib/services/api';
import {isAuthenticated, authLoading, authError, authSession, type Session} from '$lib/stores/auth';
import {mapBackendError} from "$lib/utils";
import {get} from "svelte/store";

const KEY_SESSION = 'user_session';
let isRefreshing = false;

let logoutTimer: ReturnType<typeof setTimeout> | null = null;

interface LoginResponse {
    access_token: string;
    refresh_token: string;
}

export const AuthService = {
    /**
     * Initialize Auth: Load token from disk and check if valid.
     * Should be called in your root layout onMount.
     */
    async init() {
        if (logoutTimer) clearTimeout(logoutTimer);

        try {
            const session = await db.get<Session>(KEY_SESSION);

            if (session?.access_token && session.refresh_token && session.username) {
                const exp = getJwtExpiration(session.access_token);
                const now = Math.floor(Date.now() / 1000);

                if (exp && exp > now) {
                    authSession.set(session);
                    isAuthenticated.set(true);

                    await scheduleAutoRefresh(exp);
                    console.log("[AuthService] Session restored.");
                } else {
                    console.log("[AuthService] Access token expired. Attempting to refresh session...");
                    await this.refreshToken(session.username, session.refresh_token);
                }
            }
        } catch (err) {
            console.error("[AuthService] Init Error:", err);
            await this.logout();
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

            if (data.access_token && data.refresh_token) {
                await this.updateCredentials(data, username);

                const exp = getJwtExpiration(data.access_token);
                if (exp) await scheduleAutoRefresh(exp);
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

    async updateCredentials(data: LoginResponse, username: string) {
        const session: Session = {
            access_token: data.access_token,
            refresh_token: data.refresh_token,
            username: username
        }

        await db.set(KEY_SESSION, session);
        await db.save();

        authSession.set(session);
        isAuthenticated.set(true);
    },

    async refreshToken(username: string, refresh_token: string) {
        if (logoutTimer) clearTimeout(logoutTimer);
        if (isRefreshing) return false;
        isRefreshing = true;

        try {
            const data = await api.post<LoginResponse>('/token/refresh', {
                username,
                refresh_token
            });

            await this.updateCredentials(data, username);

            const exp = getJwtExpiration(data.access_token);
            if (exp) await scheduleAutoRefresh(exp);

            return true;
        } catch (err: any) {
            console.error("[AuthService] Refresh failed:", err);
            await this.logout();
        } finally {
            isRefreshing = false;
        }
    },

    /**
     * Logout Function
     */
    async logout() {

        const session = get(authSession);

        try {
            if (session?.refresh_token) {
                await api.post('/logout', {refresh_token: session.refresh_token});
            }
            console.log("[AuthService] Server-side session invalidated.");
        } catch (err: any) {
            console.error("[AuthService] Server-side logout failed:", err.message);
        } finally {
            if (logoutTimer) clearTimeout(logoutTimer);

            isAuthenticated.set(false);
            authSession.set(null);

            await db.set(KEY_SESSION, null);
            await db.save();
        }
    },
};

async function scheduleAutoRefresh(expTimestamp: number) {
    if (logoutTimer) clearTimeout(logoutTimer);

    const now = Math.floor(Date.now() / 1000);
    const leadTime = 30;
    const timeLeft = (expTimestamp - now - leadTime) * 1000;

    if (timeLeft > 0) {
        logoutTimer = setTimeout(async () => {
            console.log("[AuthService] Token almost expired. Refreshing...");
            const authData = await db.get<Session>(KEY_SESSION);
            if (authData?.refresh_token) {
                await AuthService.refreshToken(authData.username, authData.refresh_token)
            } else {
                void AuthService.logout();
            }
        }, timeLeft);
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
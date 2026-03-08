import {db} from '$lib/stores/app-db';
import {api} from '$lib/services/api';
import {isAuthenticated, authLoading, authError, authSession, type Session} from '$lib/stores/auth';
import {mapBackendError} from "$lib/utils";
import {get} from "svelte/store";
import {invoke} from "@tauri-apps/api/core";
import {listen} from "@tauri-apps/api/event";

const KEY_USERNAME = 'username';
let isRefreshing = false;

interface LoginResponse {
    access_token: string;
    refresh_token: string;
}

interface TokenPayload {
    access: string;
    refresh: string;
}

export const AuthService = {
    /**
     * Initialize Auth: Load token from disk and check if valid.
     * Starts the listener for background token refreshes.
     */
    async init() {
        await this.setupRefreshListener();

        try {
            const session = await getSession();
            if (!session || !session.access_token || !session.refresh_token || !session.username) {
                return;
            }

            const exp = getJwtExpiration(session.access_token);
            const now = Math.floor(Date.now() / 1000);

            if (exp && exp > now) {
                authSession.set(session);
                isAuthenticated.set(true);
            } else {
                await refreshToken(session.username, session.refresh_token);
            }
        } catch (err: any) {
            console.error("[AuthService] Init Error:", err);
            // Only logout on explicit auth failure. Network errors should not log the user out.
            if (err.message === 'ERR_AUTH' || err.message === 'ERR_FORBIDDEN') {
                await this.logout();
            }
        }
    },

    async setupRefreshListener() {
        await listen<TokenPayload>("tokens-refreshed", (event) => {
            const current = get(authSession);
            if (!current) return;

            authSession.set({
                ...current,
                access_token: event.payload.access,
                refresh_token: event.payload.refresh
            });
            console.log("[AuthService] Session synced with background refresh.");
        });
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
                await updateCredentials(data, username);
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
        const session = get(authSession);

        try {
            if (session?.refresh_token) {
                await api.post('/logout', {refresh_token: session.refresh_token});
            }
        } catch (err: any) {
            console.error("[AuthService] Server-side logout failed:", err.message);
        } finally {
            isAuthenticated.set(false);
            authSession.set(null);

            await db.set(KEY_USERNAME, null);
            await db.save();

            await invoke('purge_tokens');
        }
    },
}

function getJwtExpiration(token: string | null): number | null {
    if (!token) return null;
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
        return null;
    }
}

async function getSession() {
    try {
        const username = await db.get<string>(KEY_USERNAME);
        if (!username) return null;

        const tokens = await invoke<TokenPayload>('get_tokens');

        const session: Session = {
            access_token: tokens.access,
            refresh_token: tokens.refresh,
            username: username
        }
        return session;
    } catch (err) {
        return null
    }
}

async function updateCredentials(data: LoginResponse, username: string | null) {
    if (!username) return;

    const session: Session = {
        access_token: data.access_token,
        refresh_token: data.refresh_token,
        username: username
    }

    await db.set(KEY_USERNAME, session.username);
    await db.save();

    await invoke('save_tokens', {
        accessToken: data.access_token,
        refreshToken: data.refresh_token
    })

    authSession.set(session);
    isAuthenticated.set(true);
}

async function refreshToken(username: string | null, refresh_token: string | null) {
    if (!username || !refresh_token || isRefreshing) return false;
    isRefreshing = true;

    try {
        const data = await api.post<LoginResponse>('/token/refresh', {
            username,
            refresh_token
        });

        await updateCredentials(data, username);
        return true;
    } catch (err: any) {
        console.error("[AuthService] Refresh failed:", err);
        // Only logout on explicit auth failure. Network errors should not log the user out.
        if (err.message === 'ERR_AUTH' || err.message === 'ERR_FORBIDDEN') {
            await AuthService.logout();
        }
        return false;
    } finally {
        isRefreshing = false;
    }
}
import { get } from 'svelte/store';
import { fetch } from '@tauri-apps/plugin-http';
import { db } from '$lib/stores/app-db';
import { serverUrl } from '$lib/stores/settings';

const KEY_JWT = 'auth_token';

// ---------------------------------------------------------
// CONFIGURATION
// Change this to '/v1' when the backend updates.
// Currently empty for unversioned API.
// ---------------------------------------------------------
const API_PREFIX = '';

class ApiService {
    /**
     * Helper to construct headers and full URL dynamically
     */
    private async prepareRequest(endpoint: string) {
        const token = await db.get<string>(KEY_JWT);
        const baseUrl = get(serverUrl);

        if (!baseUrl) {
            throw new Error("ERR_OFFLINE");
        }

        // Clean trailing slash from base URL to prevent double slashes
        // e.g. "https://api.com/" -> "https://api.com"
        const cleanBase = baseUrl.replace(/\/$/, "");

        // Construct full URL: Base + Version Prefix + Endpoint
        // Result: "https://api.com" + "" + "/users/access"
        // Future: "https://api.com" + "/v1" + "/users/access"
        const url = `${cleanBase}${API_PREFIX}${endpoint}`;

        const headers = {
            'Authorization': token ? `Bearer ${token}` : '',
            'Content-Type': 'application/json'
        };

        return { url, headers };
    }

    private handleResponse(response: Response) {
        if (response.ok) return response;

        switch (response.status) {
            case 401: throw new Error("ERR_AUTH");
            case 403: throw new Error("ERR_FORBIDDEN");
            case 404: throw new Error("ERR_NOT_FOUND");
            case 500: throw new Error("ERR_SERVER");
            case 502:
            case 503: throw new Error("ERR_OFFLINE");
            default: throw new Error(`ERR_UNKNOWN:${response.status}`);
        }
    }

    async get<T>(endpoint: string): Promise<T> {
        const { url, headers } = await this.prepareRequest(endpoint);

        try {
            const response = await fetch(url, { method: 'GET', headers });
            this.handleResponse(response);
            return await response.json() as T;
        } catch (err: any) {
            if (err.message && err.message.startsWith('ERR_')) throw err;
            throw new Error("ERR_NETWORK");
        }
    }

    async post<T>(endpoint: string, body: object): Promise<T> {
        const { url, headers } = await this.prepareRequest(endpoint);

        try {
            const response = await fetch(url, {
                method: 'POST',
                headers,
                body: JSON.stringify(body)
            });
            this.handleResponse(response);
            return await response.json().catch(() => ({})) as T;
        } catch (err: any) {
            if (err.message && err.message.startsWith('ERR_')) throw err;
            throw new Error("ERR_NETWORK");
        }
    }
}

export const api = new ApiService();
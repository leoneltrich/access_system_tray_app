<script lang="ts">
    import { addServer } from '$lib/stores/servers';
    import { goto } from '$app/navigation';
    import { mapBackendError } from '$lib/utils'; // <--- 1. Import the mapper
    import { AlertCircle } from 'lucide-svelte';

    let serverId = "";
    let isLoading = false;
    let errorMsg = "";

    async function handleSubmit() {
        if (!serverId.trim()) return;

        isLoading = true;
        errorMsg = "";

        try {
            await addServer(serverId.trim());
            goto('/');
        } catch (err: any) {
            // <--- 2. Use the mapper to convert "ERR_AUTH" to "Please Log In"
            errorMsg = mapBackendError(err);
        } finally {
            isLoading = false;
        }
    }
</script>

<div class="view-content">
    <div class="view-header">
        <h2 class="view-title">Add Server</h2>
    </div>

    <div class="view-body">
        <div class="content-wrapper">
            <p class="subtitle">Enter the Server ID to verify and add it to your dashboard.</p>

            {#if errorMsg}
                <div class="error-banner">
                    <AlertCircle size={16} />
                    <span>{errorMsg}</span>
                </div>
            {/if}

            <div class="input-group">
                <input
                        type="text"
                        placeholder="e.g. media-server-01"
                        bind:value={serverId}
                        disabled={isLoading}
                        spellcheck="false"
                        on:keydown={(e) => e.key === 'Enter' && handleSubmit()}
                        autoFocus
                />
            </div>
        </div>
    </div>

    <div class="footer">
        <button class="action-btn primary" on:click={handleSubmit} disabled={isLoading}>
            {#if isLoading}Verifying...{:else}Add Server{/if}
        </button>
    </div>
</div>

<style>
    /* --- LAYOUT --- */
    .view-content {
        display: flex;
        flex-direction: column;
        height: 100%;
    }

    .view-header {
        height: 2rem;
        display: flex;
        align-items: center;
        margin-bottom: 2rem;
        flex-shrink: 0;
    }

    .view-title {
        margin: 0;
        font-size: 1.25rem;
        font-weight: 600;
    }

    .view-body {
        flex: 1;
        display: flex;
        flex-direction: column;
    }

    .content-wrapper {
        display: flex;
        flex-direction: column;
        gap: 1.5rem;
    }

    .footer {
        margin-top: auto;
        padding-top: 1rem;
        flex-shrink: 0;
    }

    /* --- TEXT & INPUTS --- */
    .subtitle {
        margin: 0;
        color: #888;
        font-size: 0.9rem;
        line-height: 1.4;
    }

    .input-group {
        display: flex;
        flex-direction: column;
    }

    input {
        background: #1a1a1a;
        border: 1px solid #333;
        color: white;
        padding: 12px;
        border-radius: 8px;
        font-size: 0.95rem;
        outline: none;
        transition: all 0.2s;
    }
    input:focus { border-color: #555; background: #222; }
    input:disabled { opacity: 0.5; }

    .error-banner {
        background: rgba(239, 68, 68, 0.1);
        border: 1px solid rgba(239, 68, 68, 0.2);
        color: #ef4444;
        padding: 10px;
        border-radius: 8px;
        font-size: 0.85rem;
        display: flex;
        align-items: center;
        gap: 8px;
    }

    /* --- BUTTON --- */
    .action-btn {
        width: 100%;
        padding: 10px;
        border-radius: 8px;
        font-weight: 600;
        font-size: 0.9rem;
        cursor: pointer;
        border: none;
        transition: opacity 0.2s;
    }
    .action-btn:hover { opacity: 0.9; }
    .action-btn:disabled { opacity: 0.7; cursor: wait; }
    .action-btn.primary { background: white; color: black; }
</style>
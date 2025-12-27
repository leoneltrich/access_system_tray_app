<script lang="ts">
    import { goto } from '$app/navigation';
    import { addServer } from '$lib/stores/servers';
    import { mapBackendError } from '$lib/utils';

    // -- STATE --
    let serverId = $state("");
    let isLoading = $state(false);
    let errorMsg = $state("");

    async function handleSubmit(event?: Event) {
        // Prevent default if called via form submit
        if (event) event.preventDefault();

        const trimmedId = serverId.trim();
        if (!trimmedId) return;

        isLoading = true;
        errorMsg = "";

        try {
            await addServer(trimmedId);
            await goto('/');
        } catch (err: any) {
            console.error("Add server failed:", err);
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

    <form class="view-body" onsubmit={handleSubmit}>
        <div class="content-wrapper">
            <p class="subtitle">Enter the Server ID to verify and add it to your dashboard.</p>

            <div class="input-group">
                <label for="server-id-input" class="sr-only">Server ID</label>

                <input
                        id="server-id-input"
                        type="text"
                        placeholder="e.g. media-server-01"
                        bind:value={serverId}
                        disabled={isLoading}
                        spellcheck="false"
                        autocomplete="off"
                        autofocus
                        class:input-error={!!errorMsg}
                />

                {#if errorMsg}
                    <p class="error-msg" role="alert">{errorMsg}</p>
                {/if}
            </div>
        </div>

        <div class="footer">
            <button
                    type="submit"
                    class="action-btn primary"
                    disabled={isLoading || !serverId.trim()}
            >
                {#if isLoading}Verifying...{:else}Add Server{/if}
            </button>
        </div>
    </form>
</div>

<style>
    /* --- LAYOUT --- */
    .view-content { display: flex; flex-direction: column; height: 100%; }
    .view-header { height: 2rem; display: flex; align-items: center; margin-bottom: 2rem; flex-shrink: 0; }
    .view-title { margin: 0; font-size: 1.25rem; font-weight: 600; }

    /* Form acts as the body flex container now */
    .view-body { flex: 1; display: flex; flex-direction: column; }

    .content-wrapper { display: flex; flex-direction: column; gap: 1.5rem; }
    .footer { margin-top: auto; padding-top: 1rem; flex-shrink: 0; }

    /* --- TEXT & INPUTS --- */
    .subtitle { margin: 0; color: #888; font-size: 0.9rem; line-height: 1.4; }

    .input-group { display: flex; flex-direction: column; gap: 0.5rem; }

    /* Screen Reader Only Class - Standard pattern to hide labels visually but keep them accessible */
    .sr-only {
        position: absolute;
        width: 1px;
        height: 1px;
        padding: 0;
        margin: -1px;
        overflow: hidden;
        clip: rect(0, 0, 0, 0);
        white-space: nowrap;
        border-width: 0;
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
        width: 100%;
        box-sizing: border-box;
    }
    input:focus { border-color: #555; background: #222; }
    input:disabled { opacity: 0.5; }

    input.input-error { border-color: #ef4444; color: #ef4444; }

    .error-msg {
        color: #ef4444;
        font-size: 0.8rem;
        margin: 0 0 0 2px;
        animation: fadeIn 0.3s ease;
    }

    /* --- BUTTON --- */
    .action-btn { width: 100%; padding: 10px; border-radius: 8px; font-weight: 600; font-size: 0.9rem; cursor: pointer; border: none; transition: opacity 0.2s; }
    .action-btn:hover { opacity: 0.9; }
    .action-btn:disabled { opacity: 0.7; cursor: not-allowed; }
    .action-btn.primary { background: white; color: black; }

    @keyframes fadeIn { from { opacity: 0; transform: translateY(-5px); } to { opacity: 1; transform: translateY(0); } }
</style>
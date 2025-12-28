<script lang="ts">
    import { goto } from '$app/navigation';
    import { addServer } from '$lib/stores/servers';
    import { mapBackendError } from '$lib/utils';
    import FormInput from '$lib/components/ui/FormInput.svelte';

    let serverId = $state("");
    let isLoading = $state(false);
    let errorMsg = $state("");

    async function handleSubmit(event?: Event) {
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

            <FormInput
                    id="server-id"
                    label="Server ID"
                    hideLabel={true}
                    placeholder="e.g. media-server-01"
                    bind:value={serverId}
                    error={errorMsg}
                    disabled={isLoading}
                    autofocus
                    autocomplete="off"
            />
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
    .view-content { display: flex; flex-direction: column; height: 100%; }
    .view-header { height: 2rem; display: flex; align-items: center; margin-bottom: 2rem; flex-shrink: 0; }
    .view-title { margin: 0; font-size: 1.25rem; font-weight: 600; }
    .view-body { flex: 1; display: flex; flex-direction: column; }
    .content-wrapper { display: flex; flex-direction: column; gap: 1.5rem; }
    .footer { margin-top: auto; padding-top: 1rem; flex-shrink: 0; }
    .subtitle { margin: 0; color: #888; font-size: 0.9rem; line-height: 1.4; }

    .action-btn { width: 100%; padding: 10px; border-radius: 8px; font-weight: 600; font-size: 0.9rem; cursor: pointer; border: none; transition: opacity 0.2s; }
    .action-btn:hover { opacity: 0.9; }
    .action-btn:disabled { opacity: 0.7; cursor: not-allowed; }
    .action-btn.primary { background: white; color: black; }
</style>
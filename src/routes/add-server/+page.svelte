<script lang="ts">
    import { goto } from '$app/navigation';
    import { ServerService } from '$lib/services/servers';
    import { mapBackendError } from '$lib/utils';
    import FormInput from '$lib/components/ui/FormInput.svelte';
    import PageView from "$lib/components/ui/PageView.svelte";

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
            await ServerService.add(trimmedId);
            await goto('/');
        } catch (err: any) {
            console.error("Add server failed:", err);
            errorMsg = mapBackendError(err);
        } finally {
            isLoading = false;
        }
    }
</script>

<PageView title="Add Server">
    <form id="add-server-form" class="view-body" onsubmit={handleSubmit}>
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
    </form>

    {#snippet footer()}
        <div class="footer">
            <button
                    type="submit"
                    form="add-server-form"
                    class="primary-action-btn disabled-btn"
                    disabled={isLoading || !serverId.trim()}
            >
                {#if isLoading}Verifying...{:else}Add Server{/if}
            </button>
        </div>
    {/snippet}
</PageView>

<style>
    .content-wrapper { display: flex; flex-direction: column; gap: 1.5rem; }
    .subtitle { margin: 0; color: #888; font-size: 0.9rem; line-height: 1.4; }
</style>
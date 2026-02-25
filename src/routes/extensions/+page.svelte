<script lang="ts">
    import { onMount } from 'svelte';
    import PageView from '$lib/components/ui/PageView.svelte';
    import { Puzzle, CircleCheckBig, CircleX } from 'lucide-svelte';
    import { ExtensionService, type Extension } from '$lib/services/extensions';
    import ExtensionCard from '$lib/components/extensions/ExtensionCard.svelte';

    let isLoading = $state(false);
    let uploadError = $state<string | null>(null);
    let uploadSuccess = $state<string | null>(null);
    
    let extensions = $state<Extension[]>([]);
    let activeExtensions = $state<Set<string>>(new Set());

    async function refreshExtensions() {
        try {
            extensions = await ExtensionService.list();
        } catch (err) {
            console.error("Failed to load extensions:", err);
        }
    }

    onMount(() => {
        refreshExtensions();
    });

    async function handleAddExtension() {
        isLoading = true;
        uploadError = null;
        uploadSuccess = null;

        try {
            const uploadedFileName = await ExtensionService.add();
            uploadSuccess = `Extension '${uploadedFileName}' uploaded successfully!`;
            await refreshExtensions(); // Refresh the list after upload
        } catch (err: any) {
            console.error("Error adding extension:", err);
            uploadError = err.message || "An unknown error occurred during upload.";
        } finally {
            isLoading = false;
        }
    }

    function handleRun(id: string) {
        console.log("Running extension:", id);
        // Simulate running for now
        if (activeExtensions.has(id)) {
            activeExtensions.delete(id);
        } else {
            activeExtensions.add(id);
        }
        activeExtensions = new Set(activeExtensions);
    }

    function handleDelete(id: string) {
        console.log("Deleting extension:", id);
        // TODO: Implement deletion logic
    }
</script>

<PageView title="Extensions">
    <div class="extension-view-body">
        {#if uploadSuccess}
            <div class="upload-feedback success">
                <CircleCheckBig size={20}/>
                <span>{uploadSuccess}</span>
            </div>
        {/if}
        {#if uploadError}
            <div class="upload-feedback error">
                <CircleX size={20}/>
                <span>{uploadError}</span>
            </div>
        {/if}

        {#if extensions.length === 0}
            <div class="empty-state">
                <p>No extensions installed.</p>
                <span class="hint">Click the button below to add one.</span>
            </div>
        {:else}
            <div class="extension-grid">
                {#each extensions as extension (extension.id)}
                    <ExtensionCard 
                        {extension} 
                        isRunning={activeExtensions.has(extension.id)}
                        onrun={handleRun}
                        ondelete={handleDelete}
                    />
                {/each}
            </div>
        {/if}
    </div>

    {#snippet footer()}
        <button
            class="primary-action-btn"
            onclick={handleAddExtension}
            disabled={isLoading}
        >
            {#if isLoading}
                <Puzzle size={18}/>
                <span>Adding Extension...</span>
            {:else}
                <Puzzle size={18}/>
                <span>Add Extension</span>
            {/if}
        </button>
    {/snippet}
</PageView>

<style>
    .extension-view-body {
        flex: 1;
        display: flex;
        flex-direction: column;
        gap: 1.25rem;
    }

    .extension-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
        gap: 12px;
        overflow-y: auto;
        padding-bottom: 2rem;
        scrollbar-width: none;
    }

    .extension-grid::-webkit-scrollbar {
        display: none;
    }

    .empty-state {
        flex: 1;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        color: #666;
        text-align: center;
    }

    .hint {
        font-size: 0.8rem;
        color: #444;
    }

    .upload-feedback {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        padding: 0.75rem;
        border-radius: 8px;
        font-size: 0.9rem;
    }

    .upload-feedback.success {
        background-color: rgba(46, 204, 113, 0.1);
        color: #2ecc71;
        border: 1px solid rgba(46, 204, 113, 0.2);
    }

    .upload-feedback.error {
        background-color: rgba(231, 76, 60, 0.1);
        color: #e74c3c;
        border: 1px solid rgba(231, 76, 60, 0.2);
    }
</style>
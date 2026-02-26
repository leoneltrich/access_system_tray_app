<script lang="ts">
    import { onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/core';
    import PageView from '$lib/components/ui/PageView.svelte';
    import { Puzzle, CircleCheckBig, CircleX } from 'lucide-svelte';
    import { ExtensionService, type Extension } from '$lib/services/extensions';
    import ExtensionCard from '$lib/components/extensions/ExtensionCard.svelte';

    let isLoading = $state(false);
    let uploadError = $state<string | null>(null);
    let uploadSuccess = $state<string | null>(null);
    
    let extensions = $state<Extension[]>([]);
    let triggeredExtensions = $state<Set<string>>(new Set());

    async function refreshExtensions() {
        try {
            extensions = await ExtensionService.list();
        } catch (err) {
            console.error("Failed to load extensions:", err);
        }
    }

    onMount(() => {
        refreshExtensions();
        
        const interval = setInterval(() => {
            refreshExtensions();
        }, 1000);

        return () => {
            clearInterval(interval);
        };
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

    async function handleRun(id: string) {
        const ext = extensions.find(e => e.id === id);
        if (!ext) return;

        try {
            if (ext.isRunning) {
                await ExtensionService.stop(id);
            } else {
                await invoke('set_dialog_status', { isOpen: true });
                try {
                    await ExtensionService.run(id);
                    
                    triggeredExtensions.add(id);
                    triggeredExtensions = new Set(triggeredExtensions);
                    setTimeout(() => {
                        triggeredExtensions.delete(id);
                        triggeredExtensions = new Set(triggeredExtensions);
                    }, 1000);

                } finally {
                    await invoke('set_dialog_status', { isOpen: false });
                }
            }
            await refreshExtensions();
        } catch (err: any) {
            uploadError = `Action failed: ${err}`;
        }
    }

    async function handleDelete(id: string) {
        try {
            await ExtensionService.delete(id);
            await refreshExtensions();
        } catch (err: any) {
            uploadError = `Delete failed: ${err}`;
        }
    }
</script>

<PageView title="Extensions">
    <div class="extension-view-body">
        {#if uploadSuccess}
            <div class="upload-feedback success">
                <CircleCheckBig size={15}/>
                <span>{uploadSuccess}</span>
            </div>
        {/if}
        {#if uploadError}
            <div class="upload-feedback error">
                <CircleX size={15}/>
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
                        isRunning={extension.isRunning}
                        justTriggered={triggeredExtensions.has(extension.id)}
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
        height: 100%;
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
        align-items: flex-start;
        gap: 0.75rem;
        padding: 1rem;
        border-radius: 10px;
        font-size: 0.75rem;
        line-height: 1.2;
        animation: fadeInDown 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    }

    @keyframes fadeInDown {
        from {
            opacity: 0;
            transform: translateY(-8px);
        }
        to {
            opacity: 1;
            transform: translateY(0);
        }
    }

    .upload-feedback :global(svg) {
        flex-shrink: 0;
        margin-top: 1px;
    }

    .upload-feedback.success {
        background-color: rgba(16, 185, 129, 0.08);
        color: #10b981;
        border: 1px solid rgba(16, 185, 129, 0.15);
    }

    .upload-feedback.error {
        background-color: rgba(239, 68, 68, 0.08);
        color: #ef4444;
        border: 1px solid rgba(239, 68, 68, 0.15);
    }
</style>
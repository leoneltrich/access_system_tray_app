<script lang="ts">
    import PageView from '$lib/components/ui/PageView.svelte';
    import {Puzzle, CircleX, CircleCheckBig} from 'lucide-svelte'; // Added CheckCircle and XCircle for feedback
    import { ExtensionService } from '$lib/services/extensions'; // Import the new service

    let isLoading = $state(false);
    let uploadError = $state<string | null>(null);
    let uploadSuccess = $state<string | null>(null);

    async function handleAddExtension() {
        isLoading = true;
        uploadError = null;
        uploadSuccess = null;

        try {
            const uploadedFileName = await ExtensionService.add();
            uploadSuccess = `Extension '${uploadedFileName}' uploaded successfully!`;
            // You'll want to refresh your list of extensions here later
        } catch (err: any) {
            console.error("Error adding extension:", err);
            uploadError = err.message || "An unknown error occurred during upload.";
        } finally {
            isLoading = false;
        }
    }
</script>

<PageView title="Extensions">
    <div class="extension-view-body"> <!-- Use a dedicated class for this page's body -->
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

        <div class="empty-state">
            <p>No extensions installed.</p>
            <span class="hint">Click the button below to add one.</span>
        </div>
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
        flex: 1; /* Make it take available space */
        display: flex;
        flex-direction: column;
        gap: 1rem; /* Space between feedback and empty state */
    }
    .empty-state {
        flex: 1; /* Make empty state take available space within its parent */
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
        background-color: rgba(46, 204, 113, 0.2); /* Green background */
        color: #2ecc71; /* Green text */
        border: 1px solid #2ecc71;
    }

    .upload-feedback.error {
        background-color: rgba(231, 76, 60, 0.2); /* Red background */
        color: #e74c3c; /* Red text */
        border: 1px solid #e74c3c;
    }
</style>
<script lang="ts">
    import PageView from "$lib/components/ui/PageView.svelte";
    import {Plus} from 'lucide-svelte';
</script>

<PageView title="Extensions">
    <div class="empty-state">
        <p>No extensions installed.</p>
        <span class="hint">Click the button below to add one.</span>
    </div>

    {#snippet footer()}
        <button class="primary-action-btn">
            <Plus size={18}/>
            <span>Add Extension</span>
        </button>
    {/snippet}
</PageView>

<style>
    .empty-state {
        height: 100%;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        color: #666;
    }

    .hint {
        font-size: 0.8rem;
        color: #444;
    }
</style>
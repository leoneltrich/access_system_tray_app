<script>
    let {title, children, footer} = $props();
</script>

<div class="view-content">
    <div class="view-header">
        <h2 class="view-title">{title}</h2>
    </div>

    <div class="view-body">
        {@render children()}
    </div>

    {#if footer}
        <div class="view-footer">
            {@render footer()}
        </div>
    {/if}
</div>

<style>
    .view-content {
        display: flex;
        flex-direction: column;
        height: 100%;
        position: relative;
    }

    .view-header {
        height: 2rem;
        display: flex;
        align-items: center;
        margin-bottom: 1.5rem;
        flex-shrink: 0;
    }

    .view-title {
        margin: 0;
        font-size: 1.25rem;
        font-weight: 600;
    }

    .view-body {
        flex: 1;
        overflow-y: auto;
        scrollbar-width: none;
        display: flex;
        flex-direction: column;
    }

    .view-body::-webkit-scrollbar {
        display: none;
    }

    .view-footer {
        margin-top: 1rem;
        flex-shrink: 0;
    }
</style>
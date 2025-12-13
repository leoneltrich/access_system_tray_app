<script lang="ts">
    import { Settings, X } from 'lucide-svelte';
    import { page } from '$app/stores';
    import { goto } from '$app/navigation';

    $: isSettingsPage = $page.url.pathname === '/settings';

    function toggleView() {
        if (isSettingsPage) {
            goto('/');
        } else {
            goto('/settings');
        }
    }
</script>

<div class="window-wrapper">
    <div class="app-container">

        <button
                class="nav-btn"
                on:click={toggleView}
                aria-label={isSettingsPage ? "Close Settings" : "Open Settings"}
        >
            {#if isSettingsPage}
                <X size={18} />
            {:else}
                <Settings size={18} />
            {/if}
        </button>

        <slot />

    </div>
</div>

<style>
    :global(html), :global(body) {
        margin: 0 !important;
        padding: 0 !important;
        background: transparent !important;
        width: 100%;
        height: 100%;
        overflow: hidden;
        font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
        user-select: none; /* Prevents highlighting text by accident */
    }

    .window-wrapper {
        width: 100vw;
        height: 100vh;

        /* 1. FIXED PADDING: Applies equal gap to Top and Bottom */
        padding: 12px;

        display: flex;
        flex-direction: column;
        box-sizing: border-box;
        background: transparent !important;
    }

    .app-container {
        position: relative;
        flex: 1;
        width: 100%;
        background-color: #111111;
        color: #ffffff;
        border-radius: 16px;
        overflow: hidden;

        /* 2. CONSISTENT INTERNAL PADDING */
        padding: 1.5rem;

        box-sizing: border-box;
        box-shadow: inset 0 0 0 1px rgba(255, 255, 255, 0.05);
    }

    .nav-btn {
        position: absolute;
        /* 3. ALIGNMENT: Matches the padding of app-container (1.5rem) */
        /* We subtract a little to center the icon vertically with the text */
        top: 1.25rem;
        right: 1.25rem;

        background: transparent;
        border: none;
        color: #666666;
        cursor: pointer;
        padding: 6px;
        border-radius: 6px;
        display: flex;
        transition: all 0.2s ease;
        z-index: 50;
    }

    .nav-btn:hover {
        color: #ffffff;
        background-color: rgba(255, 255, 255, 0.1);
    }
</style>
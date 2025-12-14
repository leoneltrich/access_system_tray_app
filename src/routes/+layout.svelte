<script lang="ts">
    import { Settings, X, User } from 'lucide-svelte';
    import { page } from '$app/stores';
    import { goto } from '$app/navigation';

    // Determine if we are on a "sub-page" (anything other than home)
    $: isSubPage = $page.url.pathname !== '/';

    function goHome() {
        goto('/');
    }

    function goSettings() {
        goto('/settings');
    }

    function goProfile() {
        goto('/profile');
    }
</script>

<div class="window-wrapper">
    <div class="app-container">

        <div class="nav-actions">
            {#if isSubPage}
                <button class="nav-btn" on:click={goHome} aria-label="Close">
                    <X size={18} />
                </button>
            {:else}
                <button class="nav-btn" on:click={goProfile} aria-label="Profile">
                    <User size={18} />
                </button>
                <button class="nav-btn" on:click={goSettings} aria-label="Settings">
                    <Settings size={18} />
                </button>
            {/if}
        </div>

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
        user-select: none;
    }

    .window-wrapper {
        width: 100vw;
        height: 100vh;
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
        padding: 1.5rem;
        box-sizing: border-box;
        box-shadow: inset 0 0 0 1px rgba(255, 255, 255, 0.05);
    }

    /* Container for top-right buttons */
    .nav-actions {
        position: absolute;
        top: 1.25rem;
        right: 1.25rem;
        display: flex;
        gap: 0.5rem; /* Space between buttons */
        z-index: 50;
    }

    .nav-btn {
        background: transparent;
        border: none;
        color: #666666;
        cursor: pointer;
        padding: 6px;
        border-radius: 6px;
        display: flex;
        transition: all 0.2s ease;
    }

    .nav-btn:hover {
        color: #ffffff;
        background-color: rgba(255, 255, 255, 0.1);
    }
</style>
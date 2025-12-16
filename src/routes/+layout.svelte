<script lang="ts">
    import { onMount } from 'svelte';
    import { Settings, X, User } from 'lucide-svelte';
    import { page } from '$app/stores';
    import { goto } from '$app/navigation';
    // Import the OS checker
    import { type } from '@tauri-apps/plugin-os';

    $: isSubPage = $page.url.pathname !== '/';

    // Default to false to prevent flash of transparency on Windows
    let isMac = false;

    onMount(async () => {
        // Detect OS. If 'macos', we enable the floating transparent look.
        const osType = await type();
        if (osType === 'macos') {
            isMac = true;
        }
    });

    function goHome() { goto('/'); }
    function goSettings() { goto('/settings'); }
    function goProfile() { goto('/profile'); }
</script>

<div class="window-wrapper" class:platform-mac={isMac}>

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

        <div class="content-slot">
            <slot />
        </div>

    </div>
</div>

<style>
    /* --- GLOBAL DEFAULTS (Windows Safe) --- */
    :global(html), :global(body) {
        margin: 0;
        padding: 0;
        width: 100%;
        height: 100%;
        overflow: hidden;
        font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
        user-select: none;

        /* Default: Opaque Dark (Prevents glitches on Windows) */
        background-color: #111111;
    }

    /* --- LAYOUT WRAPPER --- */
    .window-wrapper {
        width: 100vw;
        height: 100vh;
        display: flex;
        flex-direction: column;
        box-sizing: border-box;
        padding: 0;
        /* Ensure the wrapper itself doesn't have a background unless specified */
        background-color: transparent;
    }

    /* --- APP CONTAINER (The actual Card) --- */
    .app-container {
        flex: 1;
        display: flex;
        flex-direction: column;
        position: relative;
        background-color: #111111; /* The card keeps the dark color */
        color: #ffffff;
        box-sizing: border-box;
        overflow: hidden;
        border-radius: 0;
    }

    .content-slot {
        flex: 1;
        overflow: hidden;
        padding: 1.5rem;
    }

    /* ========================================= */
    /* MAC-SPECIFIC OVERRIDES (THE FIX)          */
    /* ========================================= */

    /* FIX: We target HTML *and* BODY.
       If the body contains the .platform-mac class, make everything behind it transparent.
    */
    :global(html:has(.platform-mac)),
    :global(body:has(.platform-mac)) {
        background: transparent !important; /* Double safety */
    }

    /* Add the floating padding back for Mac */
    .window-wrapper.platform-mac {
        padding: 12px;
    }

    /* Round the corners of the card */
    .window-wrapper.platform-mac .app-container {
        border-radius: 16px;
        /* Optional: Add a border to separate it from the wallpaper better */
        border: none !important;
        box-shadow: none !important;
    }

    /* ... Keep your button styles below ... */
    .nav-actions {
        position: absolute;
        top: 1.25rem;
        right: 1.25rem;
        display: flex;
        gap: 0.5rem;
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
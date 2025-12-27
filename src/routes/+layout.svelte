<script lang="ts">
    import { onMount } from 'svelte';
    import { Settings, X, User } from 'lucide-svelte';
    import { goto } from '$app/navigation';
    // FIX 1: Use new state-based page object instead of store
    import { page } from '$app/state';
    import { type } from '@tauri-apps/plugin-os';
    import { initAuth } from '$lib/stores/auth';

    // FIX 2: Accept 'children' prop (replaces <slot>)
    let { children } = $props();

    let isMac = $state(false);

    // FIX 3: Derived state using new 'page' object (no '$' needed)
    let isSubPage = $derived(page.url.pathname !== '/');

    onMount(async () => {
        // FIX 4: Added await
        await initAuth();

        try {
            // FIX 5: Removed await (Tauri v2 type() is synchronous)
            const osType = type();
            if (osType === 'macos') {
                isMac = true;
            }
        } catch (e) {
            console.warn("Could not detect OS, defaulting to standard view", e);
        }
    });

    function goHome() { goto('/'); }
    function goSettings() { goto('/settings'); }
    function goProfile() { goto('/login'); }
</script>

<div class="window-wrapper" class:platform-mac={isMac}>

    <div class="app-container">

        <div class="nav-actions">
            {#if isSubPage}
                <button class="nav-btn" onclick={goHome} aria-label="Close">
                    <X size={18} />
                </button>
            {:else}
                <button class="nav-btn" onclick={goProfile} aria-label="Profile">
                    <User size={18} />
                </button>
                <button class="nav-btn" onclick={goSettings} aria-label="Settings">
                    <Settings size={18} />
                </button>
            {/if}
        </div>

        <div class="content-slot">
            {@render children()}
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
        background-color: transparent;
    }

    /* --- APP CONTAINER --- */
    .app-container {
        flex: 1;
        display: flex;
        flex-direction: column;
        position: relative;
        background-color: #111111;
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
    /* MAC-SPECIFIC OVERRIDES                    */
    /* ========================================= */

    :global(html:has(.platform-mac)),
    :global(body:has(.platform-mac)) {
        background: transparent !important;
    }

    .window-wrapper.platform-mac {
        padding: 12px;
    }

    .window-wrapper.platform-mac .app-container {
        border-radius: 16px;
        border: none !important;
        box-shadow: none !important;
    }

    /* --- NAVIGATION --- */
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
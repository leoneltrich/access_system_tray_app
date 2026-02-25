<script lang="ts">
    import {onMount} from 'svelte';
    import {Settings, X, User, LayoutGrid} from 'lucide-svelte';
    import {goto} from '$app/navigation';
    import {page} from '$app/state';
    import {type} from '@tauri-apps/plugin-os';
    import {AuthService} from '$lib/services/auth';
    import {listen, type UnlistenFn} from "@tauri-apps/api/event";

    let {children} = $props();

    let isMac = $state(false);

    let isSubPage = $derived(page.url.pathname !== '/' && (page.url.pathname as string) !== '');

    onMount(() => {
        let unlisten: UnlistenFn;

        const setup = async () => {
            await AuthService.init();

            unlisten = await listen('tauri://focus', async () => {
                console.log("App gained focus, re-validating session.");
                await AuthService.init();
            })
        }

        setup();

        try {
            const osType = type();
            if (osType === 'macos') {
                isMac = true;
            }
        } catch (e) {
            console.warn("Could not detect OS, defaulting to standard view", e);
        }

        return () => {
            if (unlisten) unlisten()
        }
    });

    function goHome() {
        goto('/');
    }

    function goSettings() {
        goto('/settings');
    }

    function goProfile() {
        goto('/login');
    }

    function goExtensions() {
        goto('/extensions');
    }
</script>

<div class="window-wrapper" class:platform-mac={isMac}>

    <div class="app-container">
        <div class="nav-actions">
            {#if isSubPage}
                <button class="nav-btn" onclick={goHome} aria-label="Close">
                    <X size={18}/>
                </button>
            {:else}
                <button class="nav-btn" onclick={goProfile} aria-label="Profile">
                    <User size={18}/>
                </button>
                <button class="nav-btn" onclick={goSettings} aria-label="Settings">
                    <Settings size={18}/>
                </button>
                <button class="nav-btn" onclick={goExtensions} aria-label="Extensions">
                    <LayoutGrid size={18}/>
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

    .window-wrapper {
        width: 100vw;
        height: 100vh;
        display: flex;
        flex-direction: column;
        box-sizing: border-box;
        padding: 0;
        background-color: transparent;
    }

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

    :global(.primary-action-btn) {
        width: 100%;
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 8px;
        background: #ffffff;
        color: #000000;
        border: none;
        padding: 10px;
        border-radius: 8px;
        font-weight: 600;
        font-size: 0.9rem;
        cursor: pointer;
        transition: all 0.2s ease;
    }

    :global(.primary-action-btn:hover) {
        background: #f0f0f0;
    }

    :global(.secondary-action-btn) {
        background: #222;
        border: 1px solid #333;
        color: #ddd;
    }

    :global(.disabled-btn:disabled) {
        opacity: 0.7;
        cursor: not-allowed;
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
</style>
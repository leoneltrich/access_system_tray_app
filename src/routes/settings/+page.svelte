<script lang="ts">
    import { onMount } from 'svelte';
    import { isEnabled, enable, disable } from '@tauri-apps/plugin-autostart';
    import {
        serverUrl,
        isLoading,
        saveStatus,
        statusMessage,
        loadSettings,
        saveSettings
    } from '$lib/stores/settings';

    // -- AUTOSTART STATE --
    let autoStartActive = false;
    // Start as false so we don't show a skeleton if we don't want to.
    // OR keep it true if you prefer a split-second skeleton over a "pop-in".
    // I recommend keeping it true for correctness, but it will be near-instant now.
    let autoStartProcessing = true;

    onMount(async () => {
        // 1. Load Server Settings
        loadSettings();

        // 2. Check Autostart Status
        try {
            // REMOVED THE ARTIFICIAL DELAY HERE
            autoStartActive = await isEnabled();
        } catch (error) {
            console.warn('Autostart check failed:', error);
        } finally {
            autoStartProcessing = false;
        }
    });

    // -- TOGGLE LOGIC --
    async function toggleAutoStart() {
        if (autoStartProcessing) return;

        // Optimistic UI: Flip the toggle immediately before waiting for Rust
        const previousState = autoStartActive;
        autoStartActive = !autoStartActive;
        autoStartProcessing = true;

        try {
            if (previousState) {
                await disable();
            } else {
                await enable();
            }
        } catch (error) {
            // Revert if it failed
            console.error('Failed to toggle autostart:', error);
            autoStartActive = previousState;
        } finally {
            autoStartProcessing = false;
        }
    }
</script>

<div class="view-content">
    <div class="view-header">
        <h2 class="view-title">Configuration</h2>
    </div>

    <div class="view-body">

        <div class="section-group">
            <div class="option-row">
                <div class="option-text">
                    <span class="label-text">Run on Startup</span>
                    <span class="subtitle">Launch automatically when you log in</span>
                </div>

                <div class="toggle-wrapper">
                    {#if autoStartProcessing}
                        <div class="skeleton-toggle"></div>
                    {:else}
                        <button
                                class="toggle"
                                class:active={autoStartActive}
                                on:click={toggleAutoStart}
                                aria-label="Toggle Run on Startup"
                        >
                            <div class="thumb"></div>
                        </button>
                    {/if}
                </div>
            </div>
        </div>

        <hr class="divider" />

        <div class="form-group">
            <label for="server-url">Server URL</label>

            {#if $isLoading}
                <div class="skeleton-input"></div>
            {:else}
                <input
                        id="server-url"
                        type="text"
                        bind:value={$serverUrl}
                        spellcheck="false"
                        placeholder="https://..."
                        class:input-error={$saveStatus === 'invalid' || $saveStatus === 'error'}
                />
            {/if}

            {#if $statusMessage}
                <p class="error-msg">{$statusMessage}</p>
            {/if}
        </div>

        <div class="footer">
            <button
                    class="save-btn"
                    class:success={$saveStatus === 'success'}
                    class:error={$saveStatus === 'invalid' || $saveStatus === 'error'}
                    class:checking={$saveStatus === 'checking'}
                    disabled={$isLoading || $saveStatus === 'saving' || $saveStatus === 'checking'}
                    on:click={saveSettings}
            >
                {#if $saveStatus === 'checking'}
                    Verifying...
                {:else if $saveStatus === 'saving'}
                    Saving...
                {:else if $saveStatus === 'success'}
                    Saved!
                {:else if $saveStatus === 'invalid'}
                    Verification Failed
                {:else if $saveStatus === 'error'}
                    Error
                {:else}
                    Save Changes
                {/if}
            </button>
        </div>
    </div>
</div>

<style>
    /* --- LAYOUT --- */
    .view-content {
        display: flex;
        flex-direction: column;
        height: 100%;
        color: white;
    }

    .view-header {
        height: 2rem;
        display: flex;
        align-items: center;
        margin-bottom: 1.5rem;
    }

    .view-title {
        margin: 0;
        font-size: 1.25rem;
        font-weight: 600;
        line-height: 1;
    }

    .view-body {
        flex: 1;
        display: flex;
        flex-direction: column;
        gap: 1.5rem;
    }

    /* --- TOGGLE ROW --- */
    .option-row {
        display: flex;
        justify-content: space-between;
        align-items: center;
        min-height: 30px; /* Ensure consistent height */
    }

    .option-text {
        display: flex;
        flex-direction: column;
        gap: 4px;
    }

    .label-text {
        font-size: 0.9rem;
        font-weight: 500;
    }

    .subtitle {
        font-size: 0.75rem;
        color: #888;
    }

    .divider {
        border: none;
        height: 1px;
        background: rgba(255, 255, 255, 0.1);
        margin: 0;
    }

    /*Wrapper to keep layout stable during loading state*/
    .toggle-wrapper {
        width: 44px;
        height: 24px;
        display: flex;
        align-items: center;
        justify-content: flex-end;
    }

    /* --- TOGGLE SWITCH (FIXED) --- */
    .toggle {
        box-sizing: border-box;
        width: 44px;
        height: 24px;
        background: #333; /* Inactive track color */
        border-radius: 99px;
        border: none;
        position: relative;
        cursor: pointer;
        transition: background 0.3s ease, transform 0.1s ease;
        padding: 0;
        flex-shrink: 0; /* Prevent squishing */
    }

    /* Click feedback */
    .toggle:active { transform: scale(0.95); }

    /* Active track color (White to match save button) */
    .toggle.active { background: #ffffff; }

    .thumb {
        width: 20px;
        height: 20px;
        background: white; /* Inactive thumb color */
        border-radius: 50%;
        position: absolute;
        top: 2px;  /* Exact 2px gap from top */
        left: 2px; /* Exact 2px gap from left */
        transition: transform 0.3s cubic-bezier(0.2, 0.8, 0.2, 1), background 0.3s;
        box-shadow: 0 1px 3px rgba(0,0,0,0.3); /* Slightly nicer shadow */
    }

    /* When active */
    .toggle.active .thumb {
        /* Math for perfect symmetry:
           Container Width (44) - Thumb Width (20) - Left Gap (2) - Right Gap (2) = 20px move
        */
        transform: translateX(20px);
        background: #000000; /* Active thumb color (black contrasting white track) */
    }

    .skeleton-toggle {
        width: 44px;
        height: 24px;
        background: rgba(255,255,255,0.1);
        border-radius: 99px;
        animation: pulse 1.5s infinite;
    }

    /* --- FORM --- */
    .form-group {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    label {
        font-size: 0.8rem;
        color: #888;
        margin-left: 2px;
    }

    input {
        background: #1a1a1a;
        border: 1px solid #333;
        color: white;
        padding: 10px;
        border-radius: 8px;
        font-size: 0.9rem;
        outline: none;
        width: 100%;
        box-sizing: border-box;
        transition: all 0.2s;
    }

    input:focus {
        border-color: #555;
    }

    input.input-error {
        border-color: #ef4444;
        color: #ef4444;
    }

    .error-msg {
        color: #ef4444;
        font-size: 0.8rem;
        margin: 0 0 0 2px;
        animation: fadeIn 0.3s ease;
    }

    .skeleton-input {
        height: 40px;
        background: rgba(255,255,255,0.05);
        border-radius: 8px;
        animation: pulse 1.5s infinite;
    }

    @keyframes pulse {
        0% { opacity: 0.5; }
        50% { opacity: 0.8; }
        100% { opacity: 0.5; }
    }

    @keyframes fadeIn {
        from { opacity: 0; transform: translateY(-5px); }
        to { opacity: 1; transform: translateY(0); }
    }

    /* --- FOOTER / BUTTONS --- */
    .footer {
        margin-top: auto;
    }

    .save-btn {
        width: 100%;
        background: #ffffff;
        color: #000000;
        border: none;
        padding: 10px;
        border-radius: 8px;
        font-weight: 600;
        font-size: 0.9rem;
        cursor: pointer;
        transition: all 0.3s ease;
    }

    .save-btn:hover { opacity: 0.9; }
    .save-btn:disabled { opacity: 0.7; cursor: wait; }

    .save-btn.checking { background-color: #f59e0b; color: white; }
    .save-btn.success { background-color: #10b981; color: white; }
    .save-btn.error { background-color: #ef4444; color: white; }
</style>
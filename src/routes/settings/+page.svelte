<script lang="ts">
    import { onMount } from 'svelte';
    import {
        serverUrl,
        isLoading,
        saveStatus,
        statusMessage, // Import the new message store
        loadSettings,
        saveSettings
    } from '$lib/stores/settings';

    onMount(() => {
        loadSettings();
    });
</script>

<div class="view-content">
    <div class="view-header">
        <h2 class="view-title">Configuration</h2>
    </div>

    <div class="view-body">
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
    }

    .view-header {
        height: 2rem;
        display: flex;
        align-items: center;
        margin-bottom: 2rem;
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

    /* Red border on error */
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

    /* --- BUTTON & STATES --- */
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

    /* State: Checking (Yellow/Orange) */
    .save-btn.checking {
        background-color: #f59e0b; /* Amber */
        color: white;
    }

    /* State: Success (Green) */
    .save-btn.success {
        background-color: #10b981; /* Emerald Green */
        color: white;
    }

    /* State: Invalid/Error (Red) */
    .save-btn.error {
        background-color: #ef4444; /* Red */
        color: white;
    }
</style>
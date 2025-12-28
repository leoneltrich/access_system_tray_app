<script lang="ts">
    import { onMount } from 'svelte';
    import { serverUrl, isSettingsLoaded, autoStartEnabled } from '$lib/stores/settings';
    import { SettingsService } from '$lib/services/settings';

    import Toggle from '$lib/components/ui/Toggle.svelte';
    import FormInput from '$lib/components/ui/FormInput.svelte';

    let inputUrl = $state($serverUrl);

    let autoStartProcessing = $state(false);
    let isSaving = $state(false);

    let saveStatus = $state<'idle' | 'checking' | 'success' | 'error' | 'invalid'>('idle');
    let statusMessage = $state("");

    onMount(async () => {
        await SettingsService.load();
        inputUrl = $serverUrl;
    });

    async function handleToggleAutoStart() {
        if (autoStartProcessing) return;
        autoStartProcessing = true;

        try {
            await SettingsService.toggleAutoStart();
        } catch (error) {
            console.error('Failed to toggle autostart', error);
        } finally {
            autoStartProcessing = false;
        }
    }

    async function handleSave(event?: Event) {
        if (event) event.preventDefault();
        if (isSaving) return;

        isSaving = true;
        saveStatus = 'checking';
        statusMessage = "";

        try {
            await SettingsService.updateServerUrl(inputUrl);
            saveStatus = 'success';
            statusMessage = "Settings saved successfully";
            setTimeout(() => { saveStatus = 'idle'; statusMessage = ""; }, 3000);
        } catch (err: any) {
            saveStatus = err.message.includes("http") ? 'invalid' : 'error';
            statusMessage = err.message;
        } finally {
            isSaving = false;
        }
    }
</script>

<div class="view-content">
    <div class="view-header"><h2 class="view-title">Configuration</h2></div>

    <div class="view-body">
        <div class="section-group">
            <div class="option-row">
                <div class="option-text">
                    <span class="label-text">Run on Startup</span>
                    <span class="subtitle">Launch automatically when you log in</span>
                </div>
                <Toggle
                        checked={$autoStartEnabled}
                        isLoading={autoStartProcessing}
                        onToggle={handleToggleAutoStart}
                        ariaLabel="Run on Startup"
                />
            </div>
        </div>

        <hr class="divider" />

        <form class="settings-form" onsubmit={handleSave}>
            {#if !$isSettingsLoaded}
                <div class="skeleton-input"></div>
            {:else}
                <FormInput
                        id="server-url"
                        label="Server URL"
                        placeholder="https://..."
                        bind:value={inputUrl}
                        autocomplete="url"
                        error={saveStatus !== 'success' ? statusMessage : undefined}
                />
                {#if saveStatus === 'success'}
                    <p class="success-msg">{statusMessage}</p>
                {/if}
            {/if}

            <div class="footer">
                <button
                        type="submit"
                        class="save-btn"
                        class:success={saveStatus === 'success'}
                        class:error={saveStatus === 'invalid' || saveStatus === 'error'}
                        class:checking={saveStatus === 'checking'}
                        disabled={isSaving || !$isSettingsLoaded}
                >
                    {#if saveStatus === 'checking'} Verifying...
                    {:else if saveStatus === 'success'} Saved!
                    {:else if saveStatus === 'invalid'} Invalid URL
                    {:else if saveStatus === 'error'} Connection Failed
                    {:else} Save Changes
                    {/if}
                </button>
            </div>
        </form>
    </div>
</div>

<style>
    .view-content { display: flex; flex-direction: column; height: 100%; color: white; }
    .view-header { height: 2rem; display: flex; align-items: center; margin-bottom: 1.5rem; }
    .view-title { margin: 0; font-size: 1.25rem; font-weight: 600; line-height: 1; }
    .view-body { flex: 1; display: flex; flex-direction: column; gap: 1.5rem; }
    .settings-form { display: flex; flex-direction: column; flex: 1; }

    .option-row { display: flex; justify-content: space-between; align-items: center; min-height: 30px; gap: 0.5rem}
    .option-text { display: flex; flex-direction: column; gap: 4px; }
    .label-text { font-size: 0.9rem; font-weight: 500; }
    .subtitle { font-size: 0.75rem; color: #888; }
    .divider { border: none; height: 1px; background: rgba(255, 255, 255, 0.1); margin: 0; }

    .skeleton-input { height: 60px; background: rgba(255,255,255,0.05); border-radius: 8px; animation: pulse 1.5s infinite; }
    .success-msg { color: #10b981; font-size: 0.8rem; margin: 4px 0 0 2px; animation: fadeIn 0.3s ease; }

    .footer { margin-top: auto; }
    .save-btn { width: 100%; background: #ffffff; color: #000000; border: none; padding: 10px; border-radius: 8px; font-weight: 600; font-size: 0.9rem; cursor: pointer; transition: all 0.3s ease; }
    .save-btn:hover { opacity: 0.9; }
    .save-btn:disabled { opacity: 0.7; cursor: wait; }
    .save-btn.checking { background-color: #f59e0b; color: white; }
    .save-btn.success { background-color: #10b981; color: white; }
    .save-btn.error { background-color: #ef4444; color: white; }

    @keyframes pulse { 0% { opacity: 0.5; } 50% { opacity: 0.8; } 100% { opacity: 0.5; } }
    @keyframes fadeIn { from { opacity: 0; transform: translateY(-5px); } to { opacity: 1; transform: translateY(0); } }
</style>
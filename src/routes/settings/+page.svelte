<script lang="ts">
    import { onMount } from 'svelte';
    import { isEnabled, enable, disable } from '@tauri-apps/plugin-autostart';
    import { serverUrl, isSettingsLoaded, loadSettings, updateServerUrl } from '$lib/stores/settings';

    import Toggle from '$lib/components/ui/Toggle.svelte';
    import FormInput from '$lib/components/ui/FormInput.svelte';

    let autoStartActive = $state(false);
    let autoStartProcessing = $state(true);
    let inputUrl = $state($serverUrl);

    let isSaving = $state(false);
    let saveStatus = $state<'idle' | 'checking' | 'success' | 'error' | 'invalid'>('idle');
    let statusMessage = $state("");

    onMount(async () => {
        await loadSettings();
        inputUrl = $serverUrl;
        try { autoStartActive = await isEnabled(); }
        catch (e) { console.warn('Autostart check failed:', e); }
        finally { autoStartProcessing = false; }
    });

    async function toggleAutoStart() {
        if (autoStartProcessing) return;
        const previousState = autoStartActive;
        autoStartActive = !autoStartActive;
        autoStartProcessing = true;
        try { previousState ? await disable() : await enable(); }
        catch (error) { console.error('Failed', error); autoStartActive = previousState; }
        finally { autoStartProcessing = false; }
    }

    async function handleSave(event?: Event) {
        if (event) event.preventDefault();
        if (isSaving) return;

        isSaving = true;
        saveStatus = 'checking';
        statusMessage = "";

        try {
            await updateServerUrl(inputUrl);
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
                <Toggle checked={autoStartActive} isLoading={autoStartProcessing} onToggle={toggleAutoStart} ariaLabel="Run on Startup" />
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
    /* Layout */
    .view-content { display: flex; flex-direction: column; height: 100%; color: white; }
    .view-header { height: 2rem; display: flex; align-items: center; margin-bottom: 1.5rem; }
    .view-title { margin: 0; font-size: 1.25rem; font-weight: 600; line-height: 1; }
    .view-body { flex: 1; display: flex; flex-direction: column; gap: 1.5rem; }
    .settings-form { display: flex; flex-direction: column; flex: 1; }

    /* Toggles */
    .option-row { display: flex; justify-content: space-between; align-items: center; min-height: 30px; }
    .option-text { display: flex; flex-direction: column; gap: 4px; }
    .label-text { font-size: 0.9rem; font-weight: 500; }
    .subtitle { font-size: 0.75rem; color: #888; }
    .divider { border: none; height: 1px; background: rgba(255, 255, 255, 0.1); margin: 0; }

    .skeleton-input { height: 60px; background: rgba(255,255,255,0.05); border-radius: 8px; animation: pulse 1.5s infinite; }
    .success-msg { color: #10b981; font-size: 0.8rem; margin: 4px 0 0 2px; animation: fadeIn 0.3s ease; }

    /* Buttons */
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
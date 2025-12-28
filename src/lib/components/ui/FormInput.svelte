<script lang="ts">
    import type { HTMLInputAttributes } from 'svelte/elements';

    let {
        value = $bindable(),
        label,
        id,
        error = undefined,
        hideLabel = false,
        ...rest
    }: {
        value: string;
        label: string;
        id: string;
        error?: string | boolean;
        hideLabel?: boolean
    } & HTMLInputAttributes = $props();
</script>

<div class="input-wrapper">
    <label for={id} class:sr-only={hideLabel}>{label}</label>

    <input
            {id}
            bind:value
            class:input-error={!!error}
            spellcheck="false"
            {...rest}
    />

    {#if typeof error === 'string' && error}
        <p class="error-msg" role="alert">{error}</p>
    {/if}
</div>

<style>
    .input-wrapper {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
        width: 100%;
    }

    label {
        font-size: 0.8rem;
        color: #888;
        margin-left: 2px;
        font-weight: 500;
    }

    .sr-only {
        position: absolute; width: 1px; height: 1px; padding: 0; margin: -1px;
        overflow: hidden; clip: rect(0, 0, 0, 0); white-space: nowrap; border-width: 0;
    }

    input {
        background: #1a1a1a;
        border: 1px solid #333;
        color: white;
        padding: 12px;
        border-radius: 8px;
        font-size: 0.95rem;
        outline: none;
        transition: all 0.2s;
        width: 100%;
        box-sizing: border-box;
    }

    input:focus { border-color: #555; background: #222; }
    input:disabled { opacity: 0.5; cursor: not-allowed; }

    input.input-error { border-color: #ef4444; color: #ef4444; }

    .error-msg {
        color: #ef4444;
        font-size: 0.8rem;
        margin: -0.25rem 0 0 2px;
        animation: fadeIn 0.3s ease;
    }

    @keyframes fadeIn { from { opacity: 0; transform: translateY(-5px); } to { opacity: 1; transform: translateY(0); } }
</style>
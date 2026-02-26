<script lang="ts">
    import { Play, Trash2, Info, Terminal, Square, Check } from 'lucide-svelte';
    import type { Extension } from '$lib/services/extensions';

    let { extension, isRunning = false, justTriggered = false, onrun, ondelete } = $props<{
        extension: Extension;
        isRunning?: boolean;
        justTriggered?: boolean;
        onrun: (id: string) => void;
        ondelete: (id: string) => void;
    }>();

    // Prioritize "Launched" feedback for the first second
    let showLaunched = $derived(justTriggered);
    let showStop = $derived(isRunning && !justTriggered);
</script>

<div class="extension-card" class:active={isRunning && !justTriggered} class:triggered={justTriggered}>
    <div class="card-content">
        <div class="card-header">
            <div class="icon-container">
                {#if showLaunched}
                    <Check size={18} class="success-check" />
                {:else}
                    <Terminal size={18} />
                {/if}
                
                {#if isRunning}
                    <div class="running-indicator"></div>
                {/if}
            </div>
            
            <div class="info-anchor">
                <div class="info-trigger">
                    <Info size={14} />
                </div>
                <div class="version-tooltip">
                    <span class="label">Version</span>
                    <span class="value">{extension.version}</span>
                </div>
            </div>
        </div>

        <div class="card-body">
            <h3 class="extension-name" title={extension.name}>
                {extension.name}
            </h3>
        </div>
        
        <div class="card-footer">
            <button class="run-btn" onclick={() => onrun(extension.id)}>
                {#if showStop}
                    <Square size={14} fill="currentColor" />
                    <span>Stop</span>
                {:else if showLaunched}
                    <Check size={14} />
                    <span>Launched</span>
                {:else}
                    <Play size={14} fill="currentColor" />
                    <span>Run</span>
                {/if}
            </button>
            <button class="delete-btn" onclick={() => ondelete(extension.id)} aria-label="Delete">
                <Trash2 size={16} />
            </button>
        </div>
    </div>
</div>

<style>
    .extension-card {
        position: relative;
        background: #161616;
        border: 1px solid #262626;
        border-radius: 12px;
        transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
        min-width: 0;
    }

    /* --- STATES --- */
    .extension-card.active {
        border-color: rgba(16, 185, 129, 0.3);
        background: rgba(16, 185, 129, 0.02);
    }

    .extension-card.triggered:not(.active) {
        border-color: #10b981;
        box-shadow: 0 0 15px rgba(16, 185, 129, 0.15);
    }

    .extension-card:hover {
        border-color: #3a3a3a;
        background: #1a1a1a;
    }

    .card-content {
        padding: 16px;
        display: flex;
        flex-direction: column;
        gap: 12px;
    }

    /* --- HEADER & ICONS --- */
    .card-header {
        display: flex;
        justify-content: space-between;
        align-items: flex-start;
    }

    .icon-container {
        position: relative;
        width: 32px;
        height: 32px;
        background: #222;
        border-radius: 8px;
        display: flex;
        align-items: center;
        justify-content: center;
        color: #666;
        transition: all 0.2s;
    }

    .success-check {
        color: #10b981;
        animation: scaleIn 0.2s ease-out;
    }

    @keyframes scaleIn {
        from { transform: scale(0.5); opacity: 0; }
        to { transform: scale(1); opacity: 1; }
    }

    .running-indicator {
        position: absolute;
        top: -2px;
        right: -2px;
        width: 8px;
        height: 8px;
        background: #10b981;
        border-radius: 50%;
        border: 2px solid #161616;
        box-shadow: 0 0 8px rgba(16, 185, 129, 0.4);
    }

    .extension-card.active .icon-container {
        color: #10b981;
        background: rgba(16, 185, 129, 0.1);
    }

    /* --- TOOLTIP --- */
    .info-anchor { position: relative; }
    .info-trigger { color: #444; cursor: help; padding: 4px; transition: color 0.2s; }
    .info-trigger:hover { color: #888; }

    .version-tooltip {
        visibility: hidden;
        opacity: 0;
        position: absolute;
        top: calc(100% + 4px);
        right: 0;
        background: #222;
        border: 1px solid #333;
        padding: 6px 10px;
        border-radius: 8px;
        display: flex;
        flex-direction: column;
        gap: 2px;
        z-index: 100;
        min-width: 80px;
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.5);
        transform: translateY(-4px);
        transition: all 0.15s ease;
        pointer-events: none;
    }

    .info-anchor:hover .version-tooltip {
        visibility: visible; opacity: 1; transform: translateY(0);
    }

    .version-tooltip .label { font-size: 0.6rem; color: #666; font-weight: 700; text-transform: uppercase; }
    .version-tooltip .value { font-size: 0.75rem; color: #ddd; font-weight: 500; }

    /* --- BODY --- */
    .extension-name {
        margin: 0;
        font-size: 0.95rem;
        font-weight: 600;
        color: #eee;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
    }

    /* --- FOOTER BUTTONS --- */
    .card-footer { display: flex; gap: 8px; }

    .run-btn {
        flex: 1;
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 6px;
        background: #eee;
        color: #000;
        border: none;
        padding: 8px;
        border-radius: 6px;
        font-size: 0.8rem;
        font-weight: 600;
        cursor: pointer;
        transition: all 0.2s;
    }

    .run-btn:hover:not(:disabled) { background: #fff; }

    .extension-card.active .run-btn {
        background: #ef4444;
        color: #fff;
    }

    .extension-card.triggered:not(.active) .run-btn {
        background: #10b981;
        color: #fff;
    }

    .delete-btn {
        display: flex;
        align-items: center;
        justify-content: center;
        width: 32px;
        height: 32px;
        background: transparent;
        border: 1px solid #262626;
        color: #444;
        border-radius: 6px;
        cursor: pointer;
        transition: all 0.2s;
    }

    .delete-btn:hover {
        background: rgba(239, 68, 68, 0.1);
        border-color: rgba(239, 68, 68, 0.2);
        color: #ef4444;
    }
</style>

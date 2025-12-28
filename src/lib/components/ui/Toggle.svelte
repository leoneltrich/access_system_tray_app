<script lang="ts">
    let {
        checked = false,
        isLoading = false,
        ariaLabel = "Toggle setting",
        onToggle
    }: {
        checked: boolean,
        isLoading?: boolean,
        ariaLabel?: string,
        onToggle: () => void
    } = $props();
</script>

<div class="toggle-wrapper">
    {#if isLoading}
        <div class="skeleton-toggle"></div>
    {:else}
        <button
                class="toggle"
                class:active={checked}
                onclick={onToggle}
                type="button"
                role="switch"
                aria-checked={checked}
                aria-label={ariaLabel}
        >
            <span class="thumb"></span>
        </button>
    {/if}
</div>

<style>
    .toggle-wrapper {
        width: 44px;
        height: 24px;
        display: flex;
        align-items: center;
        justify-content: flex-end;
    }

    .toggle {
        box-sizing: border-box;
        width: 44px;
        height: 24px;
        background: #333;
        border-radius: 99px;
        border: none;
        position: relative;
        cursor: pointer;
        transition: background 0.3s ease;
        padding: 0;
        flex-shrink: 0;
    }

    .toggle:active { transform: scale(0.95); }
    .toggle.active { background: #ffffff; }

    .thumb {
        width: 20px;
        height: 20px;
        background: white;
        border-radius: 50%;
        position: absolute;
        top: 2px;
        left: 2px;
        transition: transform 0.3s cubic-bezier(0.2, 0.8, 0.2, 1), background 0.3s;
        box-shadow: 0 1px 3px rgba(0,0,0,0.3);
    }

    .toggle.active .thumb {
        transform: translateX(20px);
        background: #000000;
    }

    .skeleton-toggle {
        width: 44px;
        height: 24px;
        background: rgba(255,255,255,0.1);
        border-radius: 99px;
        animation: pulse 1.5s infinite;
    }

    @keyframes pulse {
        0% { opacity: 0.5; }
        50% { opacity: 0.8; }
        100% { opacity: 0.5; }
    }
</style>
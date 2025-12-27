<script lang="ts">
    import { Trash2, ShieldCheck, Loader2, AlertCircle, Clock } from 'lucide-svelte';
    import { requestAccess } from '$lib/stores/servers';
    import { mapBackendError } from '$lib/utils';

    // 1. Define the Shape of the Server Object
    interface Server {
        id: string;
        status: string;
        timeRemaining?: string | number;
    }

    // 2. Define Props using Svelte 5 Interface
    // "ondelete" replaces the event dispatcher
    let { server, ondelete }: { server: Server, ondelete: (id: string) => void } = $props();

    // 3. Use Runes for local state
    let isLoading = $state(false);
    let errorMessage = $state<string | null>(null);

    function formatTime(rawMinutes: string | number | null | undefined): string {
        if (!rawMinutes) return '';
        const totalMinutes = typeof rawMinutes === 'string' ? parseInt(rawMinutes, 10) : rawMinutes;
        if (isNaN(totalMinutes)) return '';

        const hours = Math.floor(totalMinutes / 60);
        const minutes = totalMinutes % 60;

        if (hours > 0) return `${hours}h ${minutes}m`;
        return `${minutes}m`;
    }

    async function handleGetAccess() {
        isLoading = true;
        errorMessage = null;

        try {
            await requestAccess(server.id);
        } catch (err: any) {
            console.error(err);
            errorMessage = mapBackendError(err);

            setTimeout(() => {
                errorMessage = null;
            }, 3000);
        } finally {
            isLoading = false;
        }
    }
</script>

<div class="card">
    <div class="card-header">
        <span class="server-name">{server.id}</span>
        <button
                class="icon-btn delete"
                title="Remove Server"
                onclick={() => ondelete(server.id)}
        >
            <Trash2 size={16} />
        </button>
    </div>

    <div class="card-status-area">
        {#if errorMessage}
      <span class="error-text">
        <AlertCircle size={12}/> {errorMessage}
      </span>
        {:else}

            <div class="status-row">
                <span class="status-dot" class:active={server.status === 'access-granted'}></span>
                <span class="status-text">
            {server.status === 'access-granted' ? 'Active' : 'No Access'}
          </span>
            </div>

            {#if server.status === 'access-granted' && server.timeRemaining}
                <div class="timer-badge">
                    <Clock size={10} />
                    <span>{formatTime(server.timeRemaining)}</span>
                </div>
            {/if}

        {/if}
    </div>

    <div class="card-actions">
        <button
                class="action-btn"
                class:success={server.status === 'access-granted'}
                onclick={handleGetAccess}
                disabled={isLoading}
        >
            {#if isLoading}
                <div class="spin"><Loader2 size={16} /></div>
                <span>Verifying...</span>
            {:else if server.status === 'access-granted'}
                <ShieldCheck size={16} />
                <span>Extend</span>
            {:else}
                <ShieldCheck size={16} />
                <span>Get Access</span>
            {/if}
        </button>
    </div>
</div>

<style>
    /* --- EXACT SAME CSS AS BEFORE --- */
    .card {
        background: #1a1a1a;
        border: 1px solid #333;
        border-radius: 12px;
        padding: 12px;
        display: flex;
        flex-direction: column;
        gap: 12px;
        transition: border-color 0.2s, background-color 0.2s;
    }
    .card:hover { border-color: #555; background-color: #222; }

    .card-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
    }
    .server-name {
        font-weight: 600;
        font-size: 0.95rem;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
    }

    .card-status-area {
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        gap: 6px;
        min-height: 40px;
    }

    .status-row {
        display: flex;
        align-items: center;
        gap: 6px;
        font-size: 0.8rem;
        color: #888;
    }

    .status-dot {
        width: 8px;
        height: 8px;
        border-radius: 50%;
        background-color: #444;
        flex-shrink: 0;
    }
    .status-dot.active { background-color: #10b981; }

    .timer-badge {
        display: inline-flex;
        align-items: center;
        gap: 4px;
        background: rgba(16, 185, 129, 0.1);
        color: #10b981;
        padding: 2px 8px;
        border-radius: 4px;
        font-size: 0.75rem;
        font-weight: 600;
    }

    .error-text {
        color: #ef4444;
        display: flex;
        align-items: center;
        gap: 4px;
        font-size: 0.8rem;
        animation: fadeIn 0.2s ease-in-out;
    }

    @keyframes fadeIn {
        from { opacity: 0; transform: translateY(-2px); }
        to { opacity: 1; transform: translateY(0); }
    }

    .card-actions {
        display: flex;
        gap: 8px;
        margin-top: auto;
        height: 32px;
    }

    .icon-btn {
        background: transparent;
        border: none;
        color: #666;
        cursor: pointer;
        padding: 4px;
        border-radius: 4px;
        transition: color 0.2s;
        display: flex;
        align-items: center;
        justify-content: center;
    }
    .icon-btn:hover { color: white; background: rgba(255,255,255,0.1); }
    .icon-btn.delete:hover { color: #ef4444; }

    .action-btn {
        flex: 1;
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 6px;
        background: #333;
        border: none;
        color: white;
        padding: 8px;
        border-radius: 6px;
        font-size: 0.85rem;
        cursor: pointer;
        transition: all 0.2s;
    }
    .action-btn:hover { background: #444; }
    .action-btn:disabled { opacity: 0.7; cursor: wait; }

    .action-btn.success {
        background: #10b981;
        color: black;
        font-weight: 600;
    }
    .action-btn.success:hover { background: #059669; }

    .spin {
        animation: spin 1s linear infinite;
        display: flex;
    }
    @keyframes spin { 100% { transform: rotate(360deg); } }
</style>
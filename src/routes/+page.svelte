<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { Plus, Trash2, ShieldCheck, Loader2, AlertCircle, Clock } from 'lucide-svelte';
  import { goto } from '$app/navigation';
  import {
    servers,
    loadServers,
    removeServer,
    requestAccess,
    syncAllStatuses
  } from '$lib/stores/servers';

  let loadingStates: Record<string, boolean> = {};
  let errorStates: Record<string, string> = {};
  let pollInterval: any;

  onMount(async () => {
    await loadServers();
    syncAllStatuses();
    pollInterval = setInterval(() => {
      syncAllStatuses();
    }, 5000);
  });

  onDestroy(() => {
    if (pollInterval) clearInterval(pollInterval);
  });

  function goAddServer() {
    goto('/add-server');
  }

  function formatTime(rawMinutes: string | number | null | undefined): string {
    if (!rawMinutes) return '';
    const totalMinutes = typeof rawMinutes === 'string' ? parseInt(rawMinutes, 10) : rawMinutes;
    if (isNaN(totalMinutes)) return '';

    const hours = Math.floor(totalMinutes / 60);
    const minutes = totalMinutes % 60;

    if (hours > 0) return `${hours}h ${minutes}m`;
    return `${minutes}m`;
  }

  async function handleGetAccess(id: string) {
    loadingStates[id] = true;
    loadingStates = { ...loadingStates };
    errorStates[id] = "";

    try {
      await requestAccess(id);
    } catch (err: any) {
      errorStates[id] = "Failed";
      console.error(err);
      setTimeout(() => {
        if(errorStates[id]) {
          delete errorStates[id];
          errorStates = {...errorStates};
        }
      }, 3000);
    } finally {
      delete loadingStates[id];
      loadingStates = { ...loadingStates };
    }
  }
</script>

<div class="view-content">
  <div class="view-header">
    <h2 class="view-title">Dashboard</h2>
  </div>

  <div class="view-body">
    {#if $servers.length === 0}
      <div class="empty-state">
        <p>No servers configured.</p>
        <span class="hint">Click the + button below to add one.</span>
      </div>
    {:else}
      <div class="grid-container">
        {#each $servers as server (server.id)}
          <div class="card">

            <div class="card-header">
              <span class="server-name">{server.id}</span>
              <button
                      class="icon-btn delete"
                      title="Remove Server"
                      on:click={() => removeServer(server.id)}
              >
                <Trash2 size={16} />
              </button>
            </div>

            <div class="card-status-area">
              {#if errorStates[server.id]}
                <span class="error-text">
                  <AlertCircle size={12}/> Failed
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
                      on:click={() => handleGetAccess(server.id)}
                      disabled={loadingStates[server.id]}
              >
                {#if loadingStates[server.id]}
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
        {/each}
      </div>
    {/if}
  </div>

  <div class="footer">
    <button class="add-server-btn" on:click={goAddServer}>
      <Plus size={18} />
      <span>Add Server</span>
    </button>
  </div>
</div>

<style>
  /* --- LAYOUT --- */
  .view-content {
    display: flex;
    flex-direction: column;
    height: 100%;
    position: relative;
  }

  .view-header {
    height: 2rem;
    display: flex;
    align-items: center;
    margin-bottom: 1.5rem;
    flex-shrink: 0;
  }

  .view-title { margin: 0; font-size: 1.25rem; font-weight: 600; }

  .view-body {
    flex: 1;
    overflow-y: auto;
    padding-right: 4px;
    scrollbar-width: none;
  }
  .view-body::-webkit-scrollbar { display: none; }

  .empty-state {
    height: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    color: #666;
  }
  .hint { font-size: 0.8rem; color: #444; }

  .grid-container {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
    gap: 12px;
    padding-bottom: 1rem;
  }

  /* --- CARD --- */
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

  /* --- STATUS AREA --- */
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

  /* TIMER TAG STYLE */
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
  }

  /* --- ACTIONS --- */
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
    flex: 1; /* Spans full width now */
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

  /* Footer */
  .footer { margin-top: 1rem; flex-shrink: 0; }

  .add-server-btn {
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
  .add-server-btn:hover { background: #f0f0f0; }

  .spin {
    animation: spin 1s linear infinite;
    display: flex;
  }
  @keyframes spin { 100% { transform: rotate(360deg); } }
</style>
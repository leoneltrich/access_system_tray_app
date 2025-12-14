<script lang="ts">
  import { onMount } from 'svelte';
  import { Plus, Trash2, RefreshCw, ShieldCheck, Loader2, AlertCircle } from 'lucide-svelte';
  import { goto } from '$app/navigation';
  import {
    servers,
    loadServers,
    removeServer,
    requestAccess
  } from '$lib/stores/servers';

  // Track loading state for each server card independently
  let loadingStates: Record<string, boolean> = {};

  // Track temporary errors to show on cards
  let errorStates: Record<string, string> = {};

  onMount(() => {
    loadServers();
  });

  function goAddServer() {
    goto('/add-server');
  }

  async function handleGetAccess(id: string) {
    loadingStates[id] = true;
    loadingStates = { ...loadingStates }; // Trigger reactivity
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

  function handleRefresh(id: string) {
    handleGetAccess(id);
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

            <div class="card-status">
              {#if errorStates[server.id]}
                <span class="error-text">
                  <AlertCircle size={12}/> Failed
                </span>
              {:else}
                <span class="status-dot" class:active={server.status === 'access-granted'}></span>
                <span class="status-text">
                  {server.status === 'access-granted' ? 'Access Granted' : 'No Access'}
                </span>
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
                  <span>Active</span>
                {:else}
                  <ShieldCheck size={16} />
                  <span>Get Access</span>
                {/if}
              </button>

              <button
                      class="icon-btn refresh"
                      on:click={() => handleRefresh(server.id)}
                      disabled={loadingStates[server.id] || server.status !== 'access-granted'}
                      title="Extend Access"
              >
                <RefreshCw size={16} class={loadingStates[server.id] ? "spin" : ""} />
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

  .view-title {
    margin: 0;
    font-size: 1.25rem;
    font-weight: 600;
  }

  .view-body {
    flex: 1;
    overflow-y: auto;
    padding-right: 4px;
    scrollbar-width: none;
  }
  .view-body::-webkit-scrollbar { display: none; }

  /* --- EMPTY STATE --- */
  .empty-state {
    height: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    color: #666;
  }
  .hint { font-size: 0.8rem; color: #444; }

  /* --- GRID SYSTEM --- */
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

  .card:hover {
    border-color: #555;
    background-color: #222;
  }

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

  .card-status {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 0.8rem;
    color: #888;
    height: 20px;
  }

  .status-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background-color: #444;
  }
  .status-dot.active { background-color: #10b981; }

  .error-text {
    color: #ef4444;
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .card-actions {
    display: flex;
    gap: 8px;
    margin-top: auto;
    height: 32px;
  }

  /* --- BUTTONS --- */
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

  /* Disabled state for icon buttons */
  .icon-btn:disabled {
    opacity: 0.3;
    cursor: not-allowed;
    background: transparent !important;
    color: #666 !important;
  }

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

  /* Footer & Layout */
  .footer {
    margin-top: 1rem;
    flex-shrink: 0;
  }

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
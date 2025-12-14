<script lang="ts">
  import { onMount } from 'svelte';
  import { Plus, Trash2, RefreshCw, ShieldCheck } from 'lucide-svelte';
  import { goto } from '$app/navigation';
  import { servers, loadServers, removeServer } from '$lib/stores/servers';

  onMount(() => {
    loadServers();
  });

  // --- HANDLERS ---

  function goAddServer() {
    goto('/add-server');
  }

  function handleGetAccess(id: string) {
    console.log(`Requesting access for ${id}...`);
  }

  function handleRefresh(id: string) {
    console.log(`Refreshing status for ${id}...`);
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
              <span class="status-dot" class:active={server.status === 'access-granted'}></span>
              <span class="status-text">
                {server.status === 'access-granted' ? 'Access Granted' : 'No Access'}
              </span>
            </div>

            <div class="card-actions">
              <button class="action-btn" on:click={() => handleGetAccess(server.id)}>
                <ShieldCheck size={16} />
                <span>Get Access</span>
              </button>

              {#if server.status === 'access-granted'}
                <button class="icon-btn refresh" on:click={() => handleRefresh(server.id)}>
                  <RefreshCw size={16} />
                </button>
              {/if}
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
  }

  .status-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background-color: #444;
  }
  .status-dot.active {
    background-color: #10b981;
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
    transition: background 0.2s;
  }
  .action-btn:hover { background: #444; }

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
</style>
<script lang="ts">
  import { Plus } from 'lucide-svelte';

  function openAddServerModal() {
    console.log("Add Server Clicked");
  }
</script>

<div class="view-content">
  <div class="view-header">
    <h2 class="view-title">Dashboard</h2>
  </div>

  <div class="view-body">
    <div class="empty-state">
    </div>
  </div>

  <div class="footer">
    <button class="add-server-btn" on:click={openAddServerModal}>
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
    align-items: center;
    justify-content: center;
  }

  /* --- FOOTER --- */
  .footer {
    margin-top: auto;
  }

  /* --- ADD SERVER BUTTON --- */
  .add-server-btn {
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    background: #ffffff;
    color: #000000;
    border: none;

    /* CHANGED: Reduced from 12px to 10px to match Settings/Profile buttons */
    padding: 10px;

    border-radius: 8px;
    font-weight: 600;
    font-size: 0.9rem; /* Matches other buttons */
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .add-server-btn:hover {
    background: #f0f0f0;
    transform: translateY(-1px);
  }

  .add-server-btn:active {
    transform: translateY(1px);
    background: #e0e0e0;
  }
</style>
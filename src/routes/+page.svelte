<script lang="ts">
  // No logic needed for now
</script>

<div class="view-content">
  <div class="view-header">
    <h1 class="view-title">System Ready</h1>
  </div>

  <div class="view-body">
    <p class="description">
      Waiting for tray events...
    </p>

    <div class="card">
      <h3>Quick Actions</h3>
      <div class="actions">
        <button>Restart</button>
        <button>Logs</button>
      </div>
    </div>
  </div>
</div>

<style>
  /* 1. LAYOUT STRUCTURE */
  .view-content {
    display: flex;
    flex-direction: column;
    height: 100%;
  }

  /* 2. HEADER */
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

  /* 3. BODY CONTENT */
  .view-body {
    flex: 1; /* Allows this area to grow if needed */
  }

  .description {
    color: #888;
    margin: 0 0 1rem 0;
    font-size: 0.9rem;
  }

  .card {
    background: rgba(255, 255, 255, 0.03);
    border: 1px solid rgba(255, 255, 255, 0.05);
    border-radius: 12px;
    padding: 1rem;
  }

  .card h3 {
    margin: 0 0 0.75rem 0;
    font-size: 0.9rem;
    color: #ccc;
  }

  .actions {
    display: flex;
    gap: 0.5rem;
  }

  button {
    background: #222;
    border: 1px solid #333;
    color: #ddd;
    padding: 6px 12px;
    border-radius: 6px;
    cursor: pointer;
    font-size: 0.8rem;
    transition: background 0.2s;
  }

  button:hover { background: #333; }
</style>
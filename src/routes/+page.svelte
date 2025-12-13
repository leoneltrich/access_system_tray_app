<script>
</script>

<div class="window-wrapper">
  <div class="app-container">
    <slot />
  </div>
</div>

<style>
  /* 1. NUCLEAR RESET: Force the browser window to be empty */
  :global(html), :global(body) {
    margin: 0 !important;
    padding: 0 !important;
    background: transparent !important;
    border: none !important;
    outline: none !important;
    box-shadow: none !important;
    width: 100%;
    height: 100%;
    overflow: hidden; /* Stops scrollbars appearing in the gap */
  }

  .window-wrapper {
    width: 100vw;
    height: 100vh;

    padding-top: 10px;
    padding-bottom: 10px;

    display: flex;
    flex-direction: column;
    box-sizing: border-box;

    background: transparent !important;
    border: none !important;
    box-shadow: none !important;
  }

  .app-container {
    flex: 1;
    width: 100%;

    background-color: #111111;

    border-radius: 18px;

    overflow: hidden;

    padding: 1rem;
    box-sizing: border-box;
  }
</style>
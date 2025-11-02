<script>
  import { createEventDispatcher } from 'svelte';

  export let systemInfo;
  export let selectedBrowser;

  const dispatch = createEventDispatcher();

  function formatMemory(mb) {
    if (mb >= 1024) {
      return (mb / 1024).toFixed(2) + ' GB';
    }
    return mb.toFixed(0) + ' MB';
  }
</script>

<div class="browser-list">
  <h3>Browsers ({systemInfo?.browsers.length || 0})</h3>
  
  {#if systemInfo && systemInfo.browsers.length > 0}
    <div class="browsers">
      {#each systemInfo.browsers as browser (browser.name)}
        <button
          class="browser-item"
          class:active={selectedBrowser?.name === browser.name}
          on:click={() => dispatch('select', browser)}
        >
          <div class="browser-header">
            <span class="browser-name">{browser.name}</span>
            <span class="browser-memory">{formatMemory(browser.total_memory_mb)}</span>
          </div>
          <div class="browser-meta">
            <span>{browser.process_count} process{browser.process_count !== 1 ? 'es' : ''}</span>
          </div>
        </button>
      {/each}
    </div>
  {:else}
    <div class="empty-browsers">
      <p>No browsers detected</p>
      <p class="hint">Make sure at least one browser is running</p>
    </div>
  {/if}
</div>

<style>
  .browser-list h3 {
    font-size: 0.875rem;
    font-weight: 600;
    margin: 0 0 1rem 0;
    color: #495057;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .browsers {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .browser-item {
    width: 100%;
    text-align: left;
    padding: 0.75rem;
    background: #ffffff;
    border: 1px solid #e0e0e0;
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s;
  }

  .browser-item:hover {
    border-color: #646cff;
    background: #f8f9fa;
  }

  .browser-item.active {
    border-color: #646cff;
    background: #e7e9ff;
    box-shadow: 0 2px 4px rgba(100, 108, 255, 0.2);
  }

  .browser-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.25rem;
  }

  .browser-name {
    font-weight: 600;
    font-size: 0.9rem;
    color: #213547;
  }

  .browser-memory {
    font-weight: 600;
    font-size: 0.875rem;
    color: #646cff;
  }

  .browser-meta {
    font-size: 0.75rem;
    color: #6c757d;
  }

  .empty-browsers {
    text-align: center;
    padding: 2rem 1rem;
    color: #6c757d;
  }

  .empty-browsers p {
    margin: 0.25rem 0;
  }

  .hint {
    font-size: 0.75rem;
    color: #adb5bd;
  }
</style>


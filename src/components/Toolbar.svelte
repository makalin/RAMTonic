<script>
  import { createEventDispatcher } from 'svelte';

  export let selectedCount = 0;
  export let sortBy = 'memory';
  export let sortAscending = false;

  const dispatch = createEventDispatcher();

  function handleSortChange(event) {
    const newSortBy = event.target.value;
    dispatch('sort', { sortBy: newSortBy, ascending: sortAscending });
  }

  function toggleSortOrder() {
    dispatch('sort', { sortBy, ascending: !sortAscending });
  }

  async function handleExport(format) {
    dispatch('export', { format });
  }
</script>

<div class="toolbar">
  <div class="toolbar-left">
    <label class="sort-label">
      Sort by:
      <select bind:value={sortBy} on:change={handleSortChange} class="sort-select">
        <option value="memory">Memory</option>
        <option value="cpu">CPU</option>
        <option value="name">Name</option>
        <option value="type">Type</option>
      </select>
    </label>
    <button class="sort-btn" on:click={toggleSortOrder} title="Toggle sort order">
      {sortAscending ? '↑' : '↓'}
    </button>
  </div>

  <div class="toolbar-right">
    <button class="export-btn" on:click={() => handleExport('json')} title="Export as JSON">
      📥 JSON
    </button>
    <button class="export-btn" on:click={() => handleExport('csv')} title="Export as CSV">
      📥 CSV
    </button>
    <button class="settings-btn" on:click={() => dispatch('open-settings')} title="Settings">
      ⚙️
    </button>
  </div>
</div>

<style>
  .toolbar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem 1rem;
    background: #ffffff;
    border-bottom: 1px solid #e0e0e0;
    margin-bottom: 1rem;
    border-radius: 8px;
  }

  .toolbar-left {
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }

  .sort-label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.875rem;
    color: #495057;
  }

  .sort-select {
    padding: 0.375rem 0.5rem;
    border: 1px solid #e0e0e0;
    border-radius: 4px;
    font-size: 0.875rem;
    background: white;
    cursor: pointer;
  }

  .sort-btn {
    padding: 0.375rem 0.75rem;
    background: #f1f1f1;
    border: 1px solid #e0e0e0;
    border-radius: 4px;
    cursor: pointer;
    font-size: 1rem;
  }

  .sort-btn:hover {
    background: #e0e0e0;
  }

  .toolbar-right {
    display: flex;
    gap: 0.5rem;
  }

  .export-btn,
  .settings-btn {
    padding: 0.5rem 0.75rem;
    background: #f1f1f1;
    border: 1px solid #e0e0e0;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.875rem;
  }

  .export-btn:hover,
  .settings-btn:hover {
    background: #e0e0e0;
  }
</style>


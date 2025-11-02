<script>
  import { createEventDispatcher } from 'svelte';

  export let searchQuery = '';
  export let selectedProcessType = '';
  export let minMemory = '';
  export let maxMemory = '';

  const dispatch = createEventDispatcher();

  const processTypes = ['Main Process', 'Tab', 'Extension', 'GPU/Renderer', 'Utility', 'All'];

  function handleSearchChange() {
    dispatch('filter', {
      searchQuery,
      processType: selectedProcessType === 'All' ? null : selectedProcessType || null,
      minMemory: minMemory ? parseFloat(minMemory) : null,
      maxMemory: maxMemory ? parseFloat(maxMemory) : null,
    });
  }

  function clearFilters() {
    searchQuery = '';
    selectedProcessType = '';
    minMemory = '';
    maxMemory = '';
    handleSearchChange();
  }
</script>

<div class="search-filter">
  <div class="search-bar">
    <input
      type="text"
      placeholder="Search processes..."
      bind:value={searchQuery}
      on:input={handleSearchChange}
      class="search-input"
    />
    <button class="clear-btn" on:click={clearFilters} title="Clear filters">
      ✕
    </button>
  </div>

  <div class="filters">
    <select
      bind:value={selectedProcessType}
      on:change={handleSearchChange}
      class="filter-select"
    >
      <option value="">All Types</option>
      {#each processTypes as type}
        <option value={type}>{type}</option>
      {/each}
    </select>

    <div class="memory-range">
      <input
        type="number"
        placeholder="Min MB"
        bind:value={minMemory}
        on:input={handleSearchChange}
        class="memory-input"
        min="0"
        step="0.1"
      />
      <span>-</span>
      <input
        type="number"
        placeholder="Max MB"
        bind:value={maxMemory}
        on:input={handleSearchChange}
        class="memory-input"
        min="0"
        step="0.1"
      />
    </div>
  </div>
</div>

<style>
  .search-filter {
    background: #ffffff;
    padding: 1rem;
    border-radius: 8px;
    margin-bottom: 1rem;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
  }

  .search-bar {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 0.75rem;
  }

  .search-input {
    flex: 1;
    padding: 0.5rem 0.75rem;
    border: 1px solid #e0e0e0;
    border-radius: 6px;
    font-size: 0.9rem;
  }

  .search-input:focus {
    outline: none;
    border-color: #646cff;
  }

  .clear-btn {
    padding: 0.5rem 0.75rem;
    background: #f1f1f1;
    border: 1px solid #e0e0e0;
    border-radius: 6px;
    cursor: pointer;
  }

  .clear-btn:hover {
    background: #e0e0e0;
  }

  .filters {
    display: flex;
    gap: 1rem;
    align-items: center;
  }

  .filter-select {
    padding: 0.5rem 0.75rem;
    border: 1px solid #e0e0e0;
    border-radius: 6px;
    font-size: 0.9rem;
    background: white;
    cursor: pointer;
  }

  .memory-range {
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }

  .memory-input {
    width: 100px;
    padding: 0.5rem 0.75rem;
    border: 1px solid #e0e0e0;
    border-radius: 6px;
    font-size: 0.9rem;
  }

  .memory-input:focus {
    outline: none;
    border-color: #646cff;
  }
</style>


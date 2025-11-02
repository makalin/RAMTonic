<script>
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri';

  export let browserName = null;

  let stats = null;
  let loading = false;

  async function loadStats() {
    loading = true;
    try {
      stats = await invoke('get_process_stats', { browserName: browserName });
    } catch (err) {
      console.error('Error loading stats:', err);
    } finally {
      loading = false;
    }
  }

  onMount(() => {
    loadStats();
  });

  $: if (browserName !== undefined) {
    loadStats();
  }

  function formatMemory(mb) {
    if (mb >= 1024) {
      return (mb / 1024).toFixed(2) + ' GB';
    }
    return mb.toFixed(1) + ' MB';
  }
</script>

{#if stats && !loading}
  <div class="process-stats">
    <h3>Statistics</h3>
    <div class="stats-grid">
      <div class="stat-item">
        <span class="stat-label">Total Processes</span>
        <span class="stat-value">{stats.total_processes}</span>
      </div>
      <div class="stat-item">
        <span class="stat-label">Total Memory</span>
        <span class="stat-value">{formatMemory(stats.total_memory_mb)}</span>
      </div>
      <div class="stat-item">
        <span class="stat-label">Average Memory</span>
        <span class="stat-value">{formatMemory(stats.average_memory_mb)}</span>
      </div>
      <div class="stat-item">
        <span class="stat-label">Max Memory</span>
        <span class="stat-value">{formatMemory(stats.max_memory_mb)}</span>
      </div>
      <div class="stat-item">
        <span class="stat-label">Total CPU</span>
        <span class="stat-value">{stats.total_cpu_percent.toFixed(1)}%</span>
      </div>
    </div>

    {#if Object.keys(stats.by_type).length > 0}
      <div class="type-stats">
        <h4>By Process Type</h4>
        <div class="type-list">
          {#each Object.entries(stats.by_type) as [type, typeStats]}
            <div class="type-item">
              <span class="type-name">{type}</span>
              <span class="type-count">{typeStats.count}</span>
              <span class="type-memory">{formatMemory(typeStats.total_memory_mb)}</span>
              <span class="type-avg">Avg: {formatMemory(typeStats.average_memory_mb)}</span>
            </div>
          {/each}
        </div>
      </div>
    {/if}
  </div>
{:else if loading}
  <div class="process-stats">
    <p>Loading statistics...</p>
  </div>
{/if}

<style>
  .process-stats {
    background: #f8f9fa;
    padding: 1rem;
    border-radius: 8px;
    margin-bottom: 1rem;
  }

  .process-stats h3 {
    font-size: 0.875rem;
    font-weight: 600;
    margin: 0 0 1rem 0;
    color: #495057;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
    gap: 1rem;
    margin-bottom: 1rem;
  }

  .stat-item {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .stat-label {
    font-size: 0.75rem;
    color: #6c757d;
  }

  .stat-value {
    font-size: 1rem;
    font-weight: 600;
    color: #213547;
  }

  .type-stats h4 {
    font-size: 0.875rem;
    font-weight: 600;
    margin: 1rem 0 0.5rem 0;
    color: #495057;
  }

  .type-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .type-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.5rem;
    background: white;
    border-radius: 4px;
    font-size: 0.875rem;
  }

  .type-name {
    font-weight: 500;
    flex: 1;
  }

  .type-count {
    color: #6c757d;
    margin: 0 0.5rem;
  }

  .type-memory {
    font-weight: 600;
    color: #646cff;
    margin: 0 0.5rem;
  }

  .type-avg {
    color: #6c757d;
    font-size: 0.75rem;
  }
</style>


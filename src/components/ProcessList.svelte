<script>
  import { createEventDispatcher } from 'svelte';

  export let processes;
  export let selectedProcesses;

  const dispatch = createEventDispatcher();

  function formatMemory(mb) {
    if (mb >= 1024) {
      return (mb / 1024).toFixed(2) + ' GB';
    }
    return mb.toFixed(1) + ' MB';
  }

  // Sort processes by memory usage (descending)
  $: sortedProcesses = [...processes].sort((a, b) => b.memory_mb - a.memory_mb);

  function getProcessTypeColor(type) {
    switch (type) {
      case 'Main Process':
        return '#007bff';
      case 'Tab':
        return '#28a745';
      case 'Extension':
        return '#ffc107';
      case 'GPU/Renderer':
        return '#dc3545';
      case 'Utility':
        return '#6c757d';
      default:
        return '#6c757d';
    }
  }
</script>

<div class="process-list">
  <table class="process-table">
    <thead>
      <tr>
        <th class="checkbox-col"></th>
        <th class="name-col">Process Name</th>
        <th class="type-col">Type</th>
        <th class="memory-col">Memory</th>
        <th class="cpu-col">CPU %</th>
        <th class="pid-col">PID</th>
      </tr>
    </thead>
    <tbody>
      {#each sortedProcesses as process (process.pid)}
        <tr class="process-row" class:selected={selectedProcesses.has(process.pid)}>
          <td class="checkbox-col">
            <input
              type="checkbox"
              checked={selectedProcesses.has(process.pid)}
              on:change={() => dispatch('toggle', process.pid)}
            />
          </td>
          <td class="name-col">
            <div class="process-name">{process.name}</div>
          </td>
          <td class="type-col">
            <span 
              class="process-type"
              style="background-color: {getProcessTypeColor(process.process_type) + '20'}; color: {getProcessTypeColor(process.process_type)}"
            >
              {process.process_type}
            </span>
          </td>
          <td class="memory-col">
            <div class="memory-cell">
              <span class="memory-value">{formatMemory(process.memory_mb)}</span>
              <div class="memory-bar-mini">
                <div 
                  class="memory-fill-mini"
                  style="width: {(process.memory_mb / sortedProcesses[0].memory_mb) * 100}%"
                ></div>
              </div>
            </div>
          </td>
          <td class="cpu-col">
            <span class="cpu-value">{process.cpu_percent.toFixed(1)}%</span>
          </td>
          <td class="pid-col">
            <span class="pid-value">{process.pid}</span>
          </td>
        </tr>
      {/each}
    </tbody>
  </table>
</div>

<style>
  .process-list {
    background: #ffffff;
    border-radius: 8px;
    overflow: hidden;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
  }

  .process-table {
    width: 100%;
    border-collapse: collapse;
  }

  .process-table thead {
    background: #f8f9fa;
    border-bottom: 2px solid #e0e0e0;
  }

  .process-table th {
    padding: 0.75rem 1rem;
    text-align: left;
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    color: #6c757d;
  }

  .process-table td {
    padding: 0.75rem 1rem;
    border-bottom: 1px solid #f0f0f0;
  }

  .process-row {
    transition: background-color 0.15s;
  }

  .process-row:hover {
    background: #f8f9fa;
  }

  .process-row.selected {
    background: #e7e9ff;
  }

  .checkbox-col {
    width: 40px;
  }

  .name-col {
    min-width: 200px;
  }

  .type-col {
    width: 140px;
  }

  .memory-col {
    width: 150px;
  }

  .cpu-col {
    width: 100px;
  }

  .pid-col {
    width: 80px;
  }

  .process-name {
    font-weight: 500;
    color: #213547;
    word-break: break-word;
  }

  .process-type {
    display: inline-block;
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    font-size: 0.75rem;
    font-weight: 500;
  }

  .memory-cell {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .memory-value {
    font-weight: 600;
    color: #213547;
  }

  .memory-bar-mini {
    width: 100%;
    height: 4px;
    background: #e9ecef;
    border-radius: 2px;
    overflow: hidden;
  }

  .memory-fill-mini {
    height: 100%;
    background: #646cff;
    transition: width 0.3s ease;
  }

  .cpu-value {
    font-weight: 500;
    color: #213547;
  }

  .pid-value {
    font-family: 'Monaco', 'Menlo', 'Consolas', monospace;
    font-size: 0.875rem;
    color: #6c757d;
  }

  input[type="checkbox"] {
    cursor: pointer;
    width: 18px;
    height: 18px;
  }
</style>


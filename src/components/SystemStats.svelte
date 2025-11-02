<script>
  export let systemInfo;
</script>

{#if systemInfo}
  <div class="system-stats">
    <h3>System Memory</h3>
    <div class="memory-bar">
      <div class="memory-fill" style="width: {(systemInfo.used_system_memory_mb / systemInfo.total_system_memory_mb) * 100}%"></div>
    </div>
    <div class="memory-info">
      <span>{systemInfo.used_system_memory_mb.toFixed(0)} MB / {systemInfo.total_system_memory_mb.toFixed(0)} MB</span>
      <span>{(systemInfo.used_system_memory_mb / systemInfo.total_system_memory_mb * 100).toFixed(1)}%</span>
    </div>
  </div>
{/if}

<style>
  .system-stats {
    margin-bottom: 2rem;
    padding: 1rem;
    background: #f8f9fa;
    border-radius: 8px;
  }

  .system-stats h3 {
    font-size: 0.875rem;
    font-weight: 600;
    margin: 0 0 0.75rem 0;
    color: #495057;
  }

  .memory-bar {
    width: 100%;
    height: 20px;
    background: #e9ecef;
    border-radius: 10px;
    overflow: hidden;
    margin-bottom: 0.5rem;
  }

  .memory-fill {
    height: 100%;
    background: linear-gradient(90deg, #28a745 0%, #ffc107 50%, #dc3545 100%);
    transition: width 0.3s ease;
  }

  .memory-info {
    display: flex;
    justify-content: space-between;
    font-size: 0.75rem;
    color: #6c757d;
  }
</style>


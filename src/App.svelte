<script>
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri';
  import BrowserList from './components/BrowserList.svelte';
  import ProcessList from './components/ProcessList.svelte';
  import SystemStats from './components/SystemStats.svelte';
  import SearchFilter from './components/SearchFilter.svelte';
  import Toolbar from './components/Toolbar.svelte';
  import SettingsPanel from './components/SettingsPanel.svelte';
  import ProcessStats from './components/ProcessStats.svelte';
  import { save } from '@tauri-apps/api/dialog';
  import { writeTextFile } from '@tauri-apps/api/fs';

  let systemInfo = null;
  let selectedBrowser = null;
  let selectedProcesses = new Set();
  let loading = false;
  let error = null;
  let autoRefresh = false;
  let refreshInterval = null;
  let settingsOpen = false;
  let filteredProcesses = null;
  let sortBy = 'memory';
  let sortAscending = false;
  let searchQuery = '';
  let selectedProcessType = '';
  let minMemory = '';
  let maxMemory = '';
  let settings = null;

  async function loadSystemInfo() {
    loading = true;
    error = null;
    try {
      systemInfo = await invoke('get_system_info');
      
      // If selected browser is no longer present, clear selection
      if (selectedBrowser && !systemInfo.browsers.find(b => b.name === selectedBrowser.name)) {
        selectedBrowser = null;
        selectedProcesses.clear();
      }
    } catch (err) {
      error = err.toString();
      console.error('Error loading system info:', err);
    } finally {
      loading = false;
    }
  }

  async function refresh() {
    await loadSystemInfo();
    if (selectedBrowser) {
      applyFiltersAndSort();
    }
    await checkMemoryAlert();
  }

  function handleBrowserSelect(browser) {
    selectedBrowser = browser;
    selectedProcesses.clear();
    filteredProcesses = null;
    applyFiltersAndSort();
  }

  function handleProcessToggle(pid) {
    if (selectedProcesses.has(pid)) {
      selectedProcesses.delete(pid);
    } else {
      selectedProcesses.add(pid);
    }
    selectedProcesses = selectedProcesses; // Trigger reactivity
  }

  function handleSelectAll() {
    if (selectedBrowser) {
      selectedBrowser.processes.forEach(p => selectedProcesses.add(p.pid));
    }
    selectedProcesses = selectedProcesses;
  }

  function handleDeselectAll() {
    selectedProcesses.clear();
    selectedProcesses = selectedProcesses;
  }

  async function handleTerminateSelected() {
    if (selectedProcesses.size === 0) return;
    
    const pids = Array.from(selectedProcesses);
    loading = true;
    try {
      await invoke('terminate_processes', { pids });
      selectedProcesses.clear();
      selectedProcesses = selectedProcesses;
      await loadSystemInfo();
    } catch (err) {
      error = err.toString();
      console.error('Error terminating processes:', err);
    } finally {
      loading = false;
    }
  }

  async function loadSettings() {
    try {
      settings = await invoke('get_settings');
      if (settings) {
        sortBy = settings.default_sort_by || 'memory';
        sortAscending = settings.default_sort_ascending || false;
        if (settings.auto_refresh_interval) {
          // Will be used when auto-refresh is enabled
        }
      }
    } catch (err) {
      console.error('Error loading settings:', err);
    }
  }

  async function handleClearMemory() {
    if (selectedProcesses.size === 0) return;
    
    const pids = Array.from(selectedProcesses);
    loading = true;
    try {
      await invoke('clear_memory', { pids });
      await loadSystemInfo();
    } catch (err) {
      error = err.toString();
      console.error('Error clearing memory:', err);
    } finally {
      loading = false;
    }
  }

  async function handleExport(event) {
    const { format } = event.detail;
    loading = true;
    try {
      const data = await invoke('export_data', { format });
      
      let filePath;
      if (format === 'json') {
        filePath = await save({
          filters: [{ name: 'JSON', extensions: ['json'] }],
          defaultPath: 'ramtonic-export.json',
        });
      } else {
        filePath = await save({
          filters: [{ name: 'CSV', extensions: ['csv'] }],
          defaultPath: 'ramtonic-export.csv',
        });
      }
      
      if (filePath) {
        await writeTextFile(filePath, data);
        error = null;
      }
    } catch (err) {
      error = err.toString();
      console.error('Error exporting data:', err);
    } finally {
      loading = false;
    }
  }

  function handleFilter(event) {
    const { searchQuery: query, processType, minMemory: min, maxMemory: max } = event.detail;
    searchQuery = query || '';
    selectedProcessType = processType || '';
    minMemory = min !== null ? min.toString() : '';
    maxMemory = max !== null ? max.toString() : '';
    
    if (selectedBrowser) {
      applyFiltersAndSort();
    }
  }

  function handleSort(event) {
    const { sortBy: newSortBy, ascending } = event.detail;
    sortBy = newSortBy;
    sortAscending = ascending;
    
    if (selectedBrowser) {
      applyFiltersAndSort();
    }
  }

  async function applyFiltersAndSort() {
    if (!selectedBrowser) return;
    
    try {
      let processes = selectedBrowser.processes;
      
      // Apply filters
      if (searchQuery || selectedProcessType || minMemory || maxMemory) {
        const filtered = await invoke('filter_processes', {
          browserName: selectedBrowser.name,
          processType: selectedProcessType || null,
          minMemoryMb: minMemory ? parseFloat(minMemory) : null,
          maxMemoryMb: maxMemory ? parseFloat(maxMemory) : null,
          searchQuery: searchQuery || null,
        });
        processes = filtered;
      }
      
      // Apply sort
      const sorted = await invoke('sort_processes', {
        processes,
        sortBy,
        ascending: sortAscending,
      });
      
      filteredProcesses = sorted;
    } catch (err) {
      console.error('Error applying filters/sort:', err);
      filteredProcesses = null;
    }
  }

  function handleSettingsSaved(event) {
    loadSettings();
    if (autoRefresh && settings && settings.auto_refresh_interval) {
      if (refreshInterval) {
        clearInterval(refreshInterval);
      }
      refreshInterval = setInterval(() => {
        refresh();
      }, settings.auto_refresh_interval);
    }
  }

  async function checkMemoryAlert() {
    if (!settings || !settings.memory_alert_threshold) return;
    
    try {
      const exceeded = await invoke('check_memory_threshold', {
        thresholdPercent: settings.memory_alert_threshold,
      });
      
      if (exceeded && systemInfo) {
        const usagePercent = (systemInfo.used_system_memory_mb / systemInfo.total_system_memory_mb) * 100;
        error = `⚠️ Memory usage is at ${usagePercent.toFixed(1)}%! Consider freeing up memory.`;
      }
    } catch (err) {
      // Ignore alert errors
    }
  }

  function toggleAutoRefresh() {
    autoRefresh = !autoRefresh;
    
    if (autoRefresh) {
      const interval = settings?.auto_refresh_interval || 2000;
      refreshInterval = setInterval(async () => {
        await refresh();
        await checkMemoryAlert();
      }, interval);
    } else {
      if (refreshInterval) {
        clearInterval(refreshInterval);
        refreshInterval = null;
      }
    }
  }

  $: if (selectedBrowser && !filteredProcesses) {
    applyFiltersAndSort();
  }

  $: displayProcesses = filteredProcesses || selectedBrowser?.processes || [];

  onMount(async () => {
    await loadSettings();
    await loadSystemInfo();
    
    if (settings?.refresh_on_startup) {
      await refresh();
    }
    
    await checkMemoryAlert();
    
    return () => {
      if (refreshInterval) {
        clearInterval(refreshInterval);
      }
    };
  });
</script>

<div class="app-container">
  <header class="app-header">
    <h1>🧪 RAMTonic</h1>
    <div class="header-actions">
      <button 
        class={autoRefresh ? 'success' : 'secondary'}
        on:click={toggleAutoRefresh}
      >
        {autoRefresh ? '⏸ Stop Auto-Refresh' : '▶ Start Auto-Refresh'}
      </button>
      <button class="secondary" on:click={refresh} disabled={loading}>
        {loading ? '⏳ Refreshing...' : '🔄 Refresh'}
      </button>
    </div>
  </header>

  {#if error}
    <div class="error-banner">
      <strong>Error:</strong> {error}
    </div>
  {/if}

  <main class="app-main">
    <div class="sidebar">
      <SystemStats {systemInfo} />
      <BrowserList 
        {systemInfo}
        {selectedBrowser}
        on:select={e => handleBrowserSelect(e.detail)}
      />
    </div>

    <div class="content">
      {#if selectedBrowser}
        <Toolbar
          {selectedCount={selectedProcesses.size}}
          bind:sortBy
          bind:sortAscending
          on:sort={handleSort}
          on:export={handleExport}
          on:open-settings={() => settingsOpen = true}
        />
        
        <SearchFilter
          bind:searchQuery
          bind:selectedProcessType
          bind:minMemory
          bind:maxMemory
          on:filter={handleFilter}
        />
        
        <ProcessStats browserName={selectedBrowser.name} />
        
        <div class="process-header">
          <h2>{selectedBrowser.name} Processes</h2>
          <div class="process-actions">
            <button class="secondary" on:click={handleSelectAll}>
              Select All
            </button>
            <button class="secondary" on:click={handleDeselectAll}>
              Deselect All
            </button>
            <button 
              class="success" 
              on:click={handleClearMemory}
              disabled={selectedProcesses.size === 0}
              title="Clear memory (graceful cleanup)"
            >
              Clear Selected ({selectedProcesses.size})
            </button>
            <button 
              class="danger" 
              on:click={handleTerminateSelected}
              disabled={selectedProcesses.size === 0}
            >
              Terminate Selected ({selectedProcesses.size})
            </button>
          </div>
        </div>
        <ProcessList
          processes={displayProcesses}
          {selectedProcesses}
          on:toggle={e => handleProcessToggle(e.detail)}
        />
      {:else}
        <div class="empty-state">
          <h2>Select a browser to view its processes</h2>
          <p>Choose a browser from the left sidebar to see detailed memory and CPU usage information.</p>
        </div>
      {/if}
    </div>
    
    <SettingsPanel
      open={settingsOpen}
      on:close={() => settingsOpen = false}
      on:settings-saved={handleSettingsSaved}
    />
  </main>
</div>

<style>
  .app-container {
    display: flex;
    flex-direction: column;
    height: 100vh;
    width: 100%;
    background: #f5f5f5;
  }

  .app-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem 1.5rem;
    background: #ffffff;
    border-bottom: 1px solid #e0e0e0;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
  }

  .app-header h1 {
    font-size: 1.5rem;
    font-weight: 600;
    margin: 0;
  }

  .header-actions {
    display: flex;
    gap: 0.5rem;
  }

  .error-banner {
    background: #fee;
    color: #c33;
    padding: 0.75rem 1.5rem;
    border-bottom: 1px solid #fcc;
  }

  .app-main {
    display: flex;
    flex: 1;
    overflow: hidden;
  }

  .sidebar {
    width: 320px;
    background: #ffffff;
    border-right: 1px solid #e0e0e0;
    overflow-y: auto;
    padding: 1rem;
  }

  .content {
    flex: 1;
    overflow-y: auto;
    padding: 1.5rem;
  }

  .process-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.5rem;
    padding-bottom: 1rem;
    border-bottom: 2px solid #e0e0e0;
  }

  .process-header h2 {
    font-size: 1.25rem;
    font-weight: 600;
    margin: 0;
  }

  .process-actions {
    display: flex;
    gap: 0.5rem;
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: #666;
    text-align: center;
  }

  .empty-state h2 {
    font-size: 1.5rem;
    margin-bottom: 0.5rem;
  }

  .empty-state p {
    font-size: 1rem;
    max-width: 500px;
  }
</style>


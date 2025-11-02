<script>
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri';
  import { createEventDispatcher } from 'svelte';

  export let open = false;

  const dispatch = createEventDispatcher();

  let settings = {
    auto_refresh_interval: 2000,
    memory_alert_threshold: 85.0,
    default_sort_by: 'memory',
    default_sort_ascending: false,
    show_system_stats: true,
    refresh_on_startup: true,
  };

  let loading = false;
  let saved = false;

  async function loadSettings() {
    loading = true;
    try {
      settings = await invoke('get_settings');
    } catch (err) {
      console.error('Error loading settings:', err);
    } finally {
      loading = false;
    }
  }

  async function saveSettings() {
    loading = true;
    saved = false;
    try {
      await invoke('save_settings', { settings });
      saved = true;
      dispatch('settings-saved', settings);
      setTimeout(() => saved = false, 2000);
    } catch (err) {
      console.error('Error saving settings:', err);
    } finally {
      loading = false;
    }
  }

  onMount(() => {
    loadSettings();
  });

  $: if (open) {
    loadSettings();
  }
</script>

{#if open}
  <div class="settings-overlay" on:click={() => dispatch('close')}>
    <div class="settings-panel" on:click|stopPropagation={() => {}}>
      <div class="settings-header">
        <h2>Settings</h2>
        <button class="close-btn" on:click={() => dispatch('close')}>✕</button>
      </div>

      <div class="settings-content">
        <div class="setting-group">
          <label>
            <span>Auto-Refresh Interval (ms)</span>
            <input
              type="number"
              bind:value={settings.auto_refresh_interval}
              min="500"
              step="100"
            />
          </label>
        </div>

        <div class="setting-group">
          <label>
            <span>Memory Alert Threshold (%)</span>
            <input
              type="number"
              bind:value={settings.memory_alert_threshold}
              min="0"
              max="100"
              step="1"
            />
          </label>
        </div>

        <div class="setting-group">
          <label>
            <span>Default Sort By</span>
            <select bind:value={settings.default_sort_by}>
              <option value="memory">Memory</option>
              <option value="cpu">CPU</option>
              <option value="name">Name</option>
              <option value="type">Type</option>
            </select>
          </label>
        </div>

        <div class="setting-group">
          <label>
            <input
              type="checkbox"
              bind:checked={settings.default_sort_ascending}
            />
            <span>Ascending Sort Order</span>
          </label>
        </div>

        <div class="setting-group">
          <label>
            <input
              type="checkbox"
              bind:checked={settings.show_system_stats}
            />
            <span>Show System Stats</span>
          </label>
        </div>

        <div class="setting-group">
          <label>
            <input
              type="checkbox"
              bind:checked={settings.refresh_on_startup}
            />
            <span>Refresh on Startup</span>
          </label>
        </div>
      </div>

      <div class="settings-footer">
        <button class="save-btn" on:click={saveSettings} disabled={loading}>
          {loading ? 'Saving...' : saved ? '✓ Saved' : 'Save Settings'}
        </button>
        <button class="cancel-btn" on:click={() => dispatch('close')}>
          Cancel
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .settings-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .settings-panel {
    background: white;
    border-radius: 12px;
    width: 90%;
    max-width: 500px;
    max-height: 80vh;
    overflow-y: auto;
    box-shadow: 0 4px 24px rgba(0, 0, 0, 0.2);
  }

  .settings-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.5rem;
    border-bottom: 1px solid #e0e0e0;
  }

  .settings-header h2 {
    margin: 0;
    font-size: 1.25rem;
    font-weight: 600;
  }

  .close-btn {
    background: none;
    border: none;
    font-size: 1.5rem;
    cursor: pointer;
    color: #6c757d;
    padding: 0;
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .close-btn:hover {
    color: #213547;
    background: #f1f1f1;
    border-radius: 4px;
  }

  .settings-content {
    padding: 1.5rem;
  }

  .setting-group {
    margin-bottom: 1.5rem;
  }

  .setting-group label {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .setting-group label span:first-child {
    font-weight: 500;
    color: #495057;
  }

  .setting-group input[type="number"],
  .setting-group select {
    padding: 0.5rem 0.75rem;
    border: 1px solid #e0e0e0;
    border-radius: 6px;
    font-size: 0.9rem;
  }

  .setting-group input[type="checkbox"] {
    margin-right: 0.5rem;
  }

  .setting-group label:has(input[type="checkbox"]) {
    flex-direction: row;
    align-items: center;
  }

  .settings-footer {
    display: flex;
    gap: 0.5rem;
    justify-content: flex-end;
    padding: 1.5rem;
    border-top: 1px solid #e0e0e0;
  }

  .save-btn {
    background: #646cff;
    color: white;
  }

  .save-btn:hover {
    background: #535bf2;
  }

  .cancel-btn {
    background: #f1f1f1;
    color: #213547;
  }
</style>


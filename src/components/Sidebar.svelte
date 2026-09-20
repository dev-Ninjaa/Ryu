<script>
  import { createEventDispatcher } from 'svelte';
  import {
    collections, history, request, envVars,
    saveCollections, loadHistory, clearHistory, restoreDefaultCollections,
    getDialog, getInvoke,
  } from '../store.js';

  export let darkTheme = true;

  const dispatch = createEventDispatcher();

  let collectionSearch = '';
  let historySearch = '';
  let dropdownOpen = false;
  let helpOpen = false;

  // ── Collections ──────────────────────────────────────────────────────────────
  $: filteredCollections = $collections.filter(col => {
    if (!collectionSearch) return true;
    const f = collectionSearch.toLowerCase();
    if (col.name.toLowerCase().includes(f)) return true;
    if (col.requests?.some(r => (r.name || r.url).toLowerCase().includes(f) || r.method.toLowerCase().includes(f))) return true;
    if (col.folders?.some(folder =>
      folder.name.toLowerCase().includes(f) ||
      folder.requests?.some(r => (r.name || r.url).toLowerCase().includes(f))
    )) return true;
    return false;
  });

  // Expand-state per collection index
  let expandedCollections = {};
  let expandedFolders = {};

  function toggleCollection(idx) {
    expandedCollections[idx] = !expandedCollections[idx];
    expandedCollections = { ...expandedCollections };
  }
  function toggleFolder(colIdx, folderIdx) {
    const key = `${colIdx}-${folderIdx}`;
    expandedFolders[key] = !expandedFolders[key];
    expandedFolders = { ...expandedFolders };
  }

  function newCollection() {
    const name = prompt('New collection name:');
    if (!name?.trim()) return;
    collections.update(cols => {
      const updated = [...cols, { name: name.trim(), requests: [], folders: [] }];
      localStorage.setItem('collections', JSON.stringify(updated));
      return updated;
    });
  }

  function editCollection(idx) {
    const col = $collections[idx];
    const newName = prompt('Edit collection name:', col.name);
    if (!newName?.trim() || newName.trim() === col.name) return;
    collections.update(cols => {
      const updated = [...cols];
      updated[idx] = { ...updated[idx], name: newName.trim() };
      localStorage.setItem('collections', JSON.stringify(updated));
      return updated;
    });
  }

  function duplicateCollection(idx) {
    const col = $collections[idx];
    const newName = prompt('New collection name:', `${col.name} Copy`);
    if (!newName?.trim()) return;
    collections.update(cols => {
      const updated = [...cols, {
        name: newName.trim(),
        requests: col.requests ? col.requests.map(r => ({ ...r })) : [],
        folders: col.folders ? col.folders.map(f => ({ ...f, requests: f.requests.map(r => ({ ...r })) })) : [],
      }];
      localStorage.setItem('collections', JSON.stringify(updated));
      return updated;
    });
  }

  async function deleteCollection(idx) {
    const col = $collections[idx];
    const dialog = getDialog();
    let confirmed = false;
    if (dialog) {
      confirmed = await dialog.confirm(`Delete "${col.name}" and all its requests?`, { title: 'Delete Collection', type: 'warning' });
    } else {
      confirmed = confirm(`Delete "${col.name}" and all its requests?`);
    }
    if (!confirmed) return;
    collections.update(cols => {
      const updated = cols.filter((_, i) => i !== idx);
      localStorage.setItem('collections', JSON.stringify(updated));
      return updated;
    });
  }

  function addRequestToCollection(colIdx) {
    const name = prompt('New request name:');
    if (!name?.trim()) return;
    collections.update(cols => {
      const updated = [...cols];
      updated[colIdx].requests.push({ ...(JSON.parse(JSON.stringify($request))), name: name.trim() });
      localStorage.setItem('collections', JSON.stringify(updated));
      expandedCollections[colIdx] = true;
      expandedCollections = { ...expandedCollections };
      return updated;
    });
  }

  function loadCollectionRequest(colIdx, reqIdx, folderIdx = null) {
    const col = $collections[colIdx];
    const req = folderIdx != null
      ? col.folders[folderIdx].requests[reqIdx]
      : col.requests[reqIdx];
    if (req) {
      const sanitized = {
        ...req,
        params: (req.params || []).map(p => ({ enabled: true, ...p })),
        headers: (req.headers || []).map(h => ({ enabled: true, ...h })),
      };
      request.set(sanitized);
    }
  }

  function deleteCollectionRequest(colIdx, reqIdx, folderIdx = null) {
    collections.update(cols => {
      const updated = [...cols];
      if (folderIdx != null) {
        updated[colIdx].folders[folderIdx].requests.splice(reqIdx, 1);
      } else {
        updated[colIdx].requests.splice(reqIdx, 1);
      }
      localStorage.setItem('collections', JSON.stringify(updated));
      return updated;
    });
  }

  function editCollectionRequest(colIdx, reqIdx, folderIdx = null) {
    const col = $collections[colIdx];
    const req = folderIdx != null ? col.folders[folderIdx].requests[reqIdx] : col.requests[reqIdx];
    const newName = prompt('Edit request name:', req.name || req.url);
    if (!newName?.trim()) return;
    collections.update(cols => {
      const updated = [...cols];
      if (folderIdx != null) {
        updated[colIdx].folders[folderIdx].requests[reqIdx].name = newName.trim();
      } else {
        updated[colIdx].requests[reqIdx].name = newName.trim();
      }
      localStorage.setItem('collections', JSON.stringify(updated));
      return updated;
    });
  }

  // Export / Import collections
  async function exportCollections() {
    const invoke = getInvoke();
    if (invoke) {
      try { await invoke('export_collections', { collections: $collections }); } catch (e) { console.error(e); }
    } else {
      const blob = new Blob([JSON.stringify({ version: '1.0', collections: $collections }, null, 2)], { type: 'application/json' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a'); a.href = url; a.download = 'collections.json';
      document.body.appendChild(a); a.click(); a.remove(); URL.revokeObjectURL(url);
    }
    dropdownOpen = false;
  }

  let importFileInput;
  function triggerImport() { importFileInput?.click(); dropdownOpen = false; }

  async function handleImport(e) {
    const file = e.target.files[0];
    if (!file) return;
    try {
      const text = await file.text();
      const payload = JSON.parse(text);
      const imported = payload.collections ?? (Array.isArray(payload) ? payload : []);
      collections.update(cols => {
        const updated = [...cols, ...imported];
        localStorage.setItem('collections', JSON.stringify(updated));
        return updated;
      });
    } catch (err) { alert('Failed to import: ' + err.message); }
    e.target.value = null;
  }

  async function clearAllCollections() {
    const dialog = getDialog();
    let confirmed = false;
    if (dialog) {
      confirmed = await dialog.confirm('Clear all collections?', { title: 'Clear Collections', type: 'warning' });
    } else {
      confirmed = confirm('Clear all collections?');
    }
    if (!confirmed) return;
    collections.set([]);
    localStorage.setItem('collections', '[]');
    dropdownOpen = false;
  }

  // ── History ──────────────────────────────────────────────────────────────────
  $: filteredHistory = $history.filter(item => {
    if (!historySearch) return true;
    const f = historySearch.toLowerCase();
    return item.url?.toLowerCase().includes(f) || item.method?.toLowerCase().includes(f);
  });

  function loadHistoryItem(item) {
    request.set({
      method: item.method ?? 'GET',
      url: item.url ?? '',
      params: item.params ?? [],
      headers: item.headers ?? [],
      body: item.body ?? '',
      auth: item.auth ?? { type: 'none', token: '', apiKey: '', apiValue: '', apiLocation: 'header', username: '', password: '' },
    });
  }

  async function handleClearHistory() {
    const dialog = getDialog();
    let confirmed = false;
    if (dialog) {
      confirmed = await dialog.confirm('Clear all history?', { title: 'Clear History', type: 'warning' });
    } else {
      confirmed = confirm('Clear all history?');
    }
    if (confirmed) await clearHistory();
  }
  function handleRestoreDefaults() {
    restoreDefaultCollections();
    dropdownOpen = false;
  }
</script>

<aside class="sidebar">
  <!-- Header -->
  <div class="sidebar-header">
    <h2>Collections</h2>
    <div class="header-actions">
      <!-- Dropdown -->
      <div class="dropdown-menu">
        <button class="btn-icon" title="Collections menu" on:click={() => (dropdownOpen = !dropdownOpen)}>
          <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
            <circle cx="8" cy="2"  r="1.5" fill="currentColor"/>
            <circle cx="8" cy="8"  r="1.5" fill="currentColor"/>
            <circle cx="8" cy="14" r="1.5" fill="currentColor"/>
          </svg>
        </button>
        {#if dropdownOpen}
          <div class="dropdown-content">
            <button class="dropdown-item" on:click={exportCollections}>Export Collections</button>
            <button class="dropdown-item" on:click={triggerImport}>Import Collections</button>
            <button class="dropdown-item" on:click={handleRestoreDefaults}>Restore Default Collections</button>
            <button class="dropdown-item" on:click={clearAllCollections}>Clear All Collections</button>
          </div>
        {/if}
        <input type="file" bind:this={importFileInput} accept=".json" style="display:none" on:change={handleImport} />
      </div>

      <!-- Help -->
      <button class="btn-icon" title="Keyboard shortcuts" on:click={() => (helpOpen = !helpOpen)}>
        <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
          <circle cx="8" cy="8" r="6" stroke="currentColor" stroke-width="1.5"/>
          <path d="M6 6.5a2 2 0 1 1 2 2v1" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
          <circle cx="8" cy="11.5" r="0.6" fill="currentColor"/>
        </svg>
      </button>

      <!-- Theme toggle -->
      <button class="btn-icon" title="Toggle theme" on:click={() => dispatch('toggleTheme')}>
        <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
          {#if darkTheme}
            <path d="M8 1v2M8 13v2M1 8h2M13 8h2M3.5 3.5l1.5 1.5M11 11l1.5 1.5M3.5 12.5l1.5-1.5M11 5l1.5-1.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
            <circle cx="8" cy="8" r="3" stroke="currentColor" stroke-width="1.5"/>
          {:else}
            <path d="M13.5 10A6 6 0 0 1 6 2.5a6 6 0 1 0 7.5 7.5z" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
          {/if}
        </svg>
      </button>

      <!-- New collection -->
      <button class="btn-icon" title="New Collection" on:click={newCollection}>
        <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
          <path d="M8 2v12M2 8h12" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
        </svg>
      </button>
    </div>
  </div>

  <!-- Shortcuts help panel -->
  {#if helpOpen}
    <div class="help-panel">
      <p><kbd>Ctrl+Enter</kbd> Send request</p>
      <p><kbd>Ctrl+S</kbd> Save request</p>
      <p><kbd>Ctrl+O</kbd> Load request</p>
      <p><kbd>Ctrl+B</kbd> Toggle sidebar</p>
      <p><kbd>Ctrl+Shift+N</kbd> New collection</p>
    </div>
  {/if}

  <!-- Collection search -->
  <div class="sidebar-search">
    <input type="text" class="search-input" placeholder="Search collections…" bind:value={collectionSearch} />
  </div>

  <!-- Collections list -->
  <div class="collections-list">
    {#if filteredCollections.length === 0}
      <div class="empty-state">
        {collectionSearch ? `No collections match "${collectionSearch}"` : 'No collections yet'}
      </div>
    {:else}
      {#each filteredCollections as col, i}
        {@const realIdx = $collections.indexOf(col)}
        <div class="collection-item">
          <!-- Folder header -->
          <div class="collection-folder" role="button" tabindex="0"
            on:click={() => toggleCollection(realIdx)}
            on:keydown={e => e.key === 'Enter' && toggleCollection(realIdx)}>
            <button class="collection-folder-toggle" on:click|stopPropagation={() => toggleCollection(realIdx)}>
              {expandedCollections[realIdx] ? '▼' : '▶'}
            </button>
            <span class="collection-name">{col.name}</span>
            <div class="collection-actions" on:click|stopPropagation>
              <button class="collection-edit-btn" title="Edit" on:click={() => editCollection(realIdx)}>
                <svg width="12" height="12" viewBox="0 0 16 16" fill="none"><path d="M11.5 2.5L13.5 4.5L6.5 11.5L4.5 13.5L2.5 11.5L9.5 4.5Z" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/><path d="M9.5 4.5L11.5 6.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/></svg>
              </button>
              <button class="collection-duplicate-btn" title="Duplicate" on:click={() => duplicateCollection(realIdx)}>
                <svg width="12" height="12" viewBox="0 0 16 16" fill="none"><path d="M4 6V2C4 1.45 4.45 1 5 1H13C13.55 1 14 1.45 14 2V10C14 10.55 13.55 11 13 11H9" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/><rect x="2" y="6" width="8" height="8" rx="1" stroke="currentColor" stroke-width="1.5"/></svg>
              </button>
              <button class="collection-delete-btn" title="Delete" on:click={() => deleteCollection(realIdx)}>
                <svg width="12" height="12" viewBox="0 0 16 16" fill="none"><path d="M12 4H4V12C4 13.1 4.9 14 6 14H10C11.1 14 12 13.1 12 12V4Z" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/><path d="M2 4H14" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/><path d="M10 2H6C5.45 2 5 2.45 5 3V4H11V3C11 2.45 10.55 2 10 2Z" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/></svg>
              </button>
              <button class="collection-add-btn" title="Add request" on:click={() => addRequestToCollection(realIdx)}>+</button>
            </div>
          </div>

          <!-- Requests -->
          {#if expandedCollections[realIdx]}
            <div class="collection-requests">
              <!-- Sub-folders -->
              {#if col.folders}
                {#each col.folders as folder, fi}
                  <div class="collection-subfolder">
                    <div class="collection-folder subfolder"
                      role="button" tabindex="0"
                      on:click={() => toggleFolder(realIdx, fi)}
                      on:keydown={e => e.key === 'Enter' && toggleFolder(realIdx, fi)}>
                      <button class="collection-folder-toggle" on:click|stopPropagation={() => toggleFolder(realIdx, fi)}>
                        {expandedFolders[`${realIdx}-${fi}`] ? '▼' : '▶'}
                      </button>
                      <span class="collection-name">{folder.name}</span>
                    </div>
                    {#if expandedFolders[`${realIdx}-${fi}`]}
                      <div class="subfolder-requests">
                        {#each folder.requests as req, ri}
                          <div class="collection-request">
                            <div class="collection-request-content" role="button" tabindex="0"
                              on:click={() => loadCollectionRequest(realIdx, ri, fi)}
                              on:keydown={e => e.key === 'Enter' && loadCollectionRequest(realIdx, ri, fi)}>
                              <span class="history-method {req.method}">{req.method}</span>
                              <span class="collection-request-name">{req.name || req.url}</span>
                            </div>
                            <div class="collection-request-actions">
                              <button class="collection-request-edit-btn" on:click={() => editCollectionRequest(realIdx, ri, fi)}>
                                <svg width="11" height="11" viewBox="0 0 16 16" fill="none"><path d="M11.5 2.5L13.5 4.5L6.5 11.5L4.5 13.5L2.5 11.5L9.5 4.5Z" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/></svg>
                              </button>
                              <button class="collection-request-delete-btn" on:click={() => deleteCollectionRequest(realIdx, ri, fi)}>
                                <svg width="11" height="11" viewBox="0 0 16 16" fill="none"><path d="M12 4H4V12C4 13.1 4.9 14 6 14H10C11.1 14 12 13.1 12 12V4Z" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/><path d="M2 4H14" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/></svg>
                              </button>
                            </div>
                          </div>
                        {/each}
                      </div>
                    {/if}
                  </div>
                {/each}
              {/if}
              <!-- Direct requests -->
              {#if col.requests}
                {#each col.requests as req, ri}
                  <div class="collection-request">
                    <div class="collection-request-content" role="button" tabindex="0"
                      on:click={() => loadCollectionRequest(realIdx, ri)}
                      on:keydown={e => e.key === 'Enter' && loadCollectionRequest(realIdx, ri)}>
                      <span class="history-method {req.method}">{req.method}</span>
                      <span class="collection-request-name">{req.name || req.url}</span>
                    </div>
                    <div class="collection-request-actions">
                      <button class="collection-request-edit-btn" title="Edit" on:click={() => editCollectionRequest(realIdx, ri)}>
                        <svg width="11" height="11" viewBox="0 0 16 16" fill="none"><path d="M11.5 2.5L13.5 4.5L6.5 11.5L4.5 13.5L2.5 11.5L9.5 4.5Z" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/></svg>
                      </button>
                      <button class="collection-request-delete-btn" title="Delete" on:click={() => deleteCollectionRequest(realIdx, ri)}>
                        <svg width="11" height="11" viewBox="0 0 16 16" fill="none"><path d="M12 4H4V12C4 13.1 4.9 14 6 14H10C11.1 14 12 13.1 12 12V4Z" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/><path d="M2 4H14" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/></svg>
                      </button>
                    </div>
                  </div>
                {/each}
              {/if}
            </div>
          {/if}
        </div>
      {/each}
    {/if}
  </div>

  <!-- Vertical resize handle -->
  <div class="sidebar-resize-handle-vertical"></div>

  <!-- History section -->
  <div class="sidebar-section">
    <div class="section-header">
      <h3>History</h3>
      <div class="section-actions">
        <button class="btn-icon" title="Clear history" on:click={handleClearHistory}>
          <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
            <path d="M2 4h12M5.5 4V3a1 1 0 011-1h3a1 1 0 011 1v1m2 0v9a2 2 0 01-2 2h-5a2 2 0 01-2-2V4h9z" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
          </svg>
        </button>
      </div>
    </div>
    <div class="sidebar-search">
      <input type="text" class="search-input" placeholder="Search history…" bind:value={historySearch} />
    </div>
    <div class="history-list">
      {#if filteredHistory.length === 0}
        <div class="empty-state">No requests yet</div>
      {:else}
        {#each filteredHistory as item}
          <button class="history-item" on:click={() => loadHistoryItem(item)}>
            <div class="history-item-header">
              <span class="history-method {item.method}">{item.method}</span>
              <span class="history-status {item.status >= 400 ? 'error' : 'success'}">{item.status}</span>
            </div>
            <div class="history-url">{item.url}</div>
            {#if item.timestamp}
              <div class="history-time">{new Date(item.timestamp).toLocaleTimeString()}</div>
            {/if}
          </button>
        {/each}
      {/if}
    </div>
  </div>
</aside>

<style>
  .sidebar {
    background: var(--bg-panel);
    border-right: 1px solid var(--border-subtle);
    display: flex;
    flex-direction: column;
    overflow: hidden;
    width: 100%;
  }

  .sidebar-header {
    padding: var(--spacing-lg);
    border-bottom: 1px solid var(--border-subtle);
    display: flex;
    align-items: center;
    justify-content: space-between;
  }
  .sidebar-header h2 {
    font-size: 11px; font-weight: 600; color: var(--text-muted);
    text-transform: uppercase; letter-spacing: 0.8px;
  }
  .header-actions { display: flex; gap: var(--spacing-xs); }

  .help-panel {
    background: var(--bg-panel-alt);
    padding: var(--spacing-md) var(--spacing-lg);
    border-bottom: 1px solid var(--border-subtle);
    font-size: var(--font-size-xs);
    color: var(--text-secondary);
    display: flex; flex-direction: column; gap: 6px;
  }
  .help-panel kbd {
    background: var(--bg-root); border: 1px solid var(--border-color);
    border-radius: 3px; padding: 1px 5px; font-family: var(--font-mono);
    font-size: 10px; margin-right: 4px;
  }

  .dropdown-menu { position: relative; }
  .dropdown-content {
    position: absolute; top: 100%; right: 0;
    background: var(--bg-panel); border: 1px solid var(--border-color);
    border-radius: var(--radius-md); box-shadow: var(--shadow-md);
    min-width: 180px; z-index: 1000; margin-top: var(--spacing-xs);
  }
  .dropdown-item {
    display: flex; align-items: center; gap: var(--spacing-sm);
    width: 100%; padding: var(--spacing-sm) var(--spacing-md);
    background: none; border: none; color: var(--text-primary);
    text-align: left; cursor: pointer; border-radius: var(--radius-sm);
    transition: background 120ms ease;
  }
  .dropdown-item:hover { background: var(--bg-hover); }

  .sidebar-search {
    padding: var(--spacing-md) var(--spacing-lg);
    border-bottom: 1px solid var(--border-subtle);
  }
  .search-input {
    width: 100%; background: var(--bg-panel-alt);
    border: 1px solid var(--border-color); border-radius: var(--radius-sm);
    padding: var(--spacing-sm) var(--spacing-md);
    font-size: var(--font-size-sm);
  }

  .collections-list {
    flex: 1; overflow-y: auto; padding: var(--spacing-sm); min-height: 100px;
  }
  .collection-item { margin-bottom: var(--spacing-sm); }

  .collection-folder {
    display: flex; align-items: center; gap: var(--spacing-sm);
    padding: var(--spacing-sm); background: transparent; border: none;
    border-radius: var(--radius-md); cursor: pointer;
    transition: background 120ms ease; width: 100%;
  }
  .collection-folder:hover { background: var(--bg-panel-alt); }
  .collection-folder.subfolder {
    padding: var(--spacing-xs) var(--spacing-sm);
    border-left: 2px solid var(--border-subtle);
    margin-left: var(--spacing-sm);
  }
  .collection-folder.subfolder:hover { border-left-color: var(--accent); }

  .collection-folder-toggle {
    background: none; border: none; color: var(--text-muted);
    cursor: pointer; font-size: 12px; padding: 0; width: 16px; flex-shrink: 0;
  }
  .collection-name {
    flex: 1; min-width: 0; overflow: hidden;
    text-overflow: ellipsis; white-space: nowrap;
  }
  .collection-actions {
    display: flex; align-items: center; gap: var(--spacing-xs); margin-left: auto;
  }
  .collection-edit-btn, .collection-duplicate-btn, .collection-delete-btn, .collection-add-btn {
    background: none; border: none; color: var(--text-muted);
    cursor: pointer; padding: 2px; border-radius: var(--radius-sm);
    display: flex; align-items: center; justify-content: center;
    transition: all 120ms ease;
  }
  .collection-edit-btn:hover, .collection-duplicate-btn:hover { background: var(--bg-panel-alt); color: var(--text-secondary); }
  .collection-delete-btn:hover { background: rgba(220,53,69,0.1); color: var(--error); }
  .collection-add-btn { padding: 2px 6px; font-weight: 600; }
  .collection-add-btn:hover { background: var(--bg-panel-alt); color: var(--text-primary); }

  .collection-requests { margin-left: var(--spacing-lg); margin-top: var(--spacing-xs); }
  .collection-subfolder { margin-bottom: var(--spacing-xs); }
  .subfolder-requests { margin-left: var(--spacing-lg); }

  .collection-request {
    display: flex; align-items: center;
    padding: var(--spacing-xs) var(--spacing-sm);
    font-size: var(--font-size-sm); border-radius: var(--radius-sm);
    transition: background 120ms ease; margin-bottom: 1px;
    width: 100%; background: none; border: none; text-align: left; cursor: pointer;
  }
  .collection-request:hover { background: var(--bg-panel-alt); }
  .collection-request-content {
    display: flex; align-items: center; gap: var(--spacing-sm);
    flex: 1; min-width: 0; cursor: pointer;
  }
  .collection-request-name {
    flex: 1; min-width: 0; overflow: hidden;
    text-overflow: ellipsis; white-space: nowrap;
  }
  .collection-request-actions {
    display: flex; align-items: center; gap: var(--spacing-xs);
    opacity: 0; transition: opacity 120ms ease;
  }
  .collection-request:hover .collection-request-actions { opacity: 1; }
  .collection-request-edit-btn, .collection-request-delete-btn {
    background: none; border: none; color: var(--text-muted);
    cursor: pointer; padding: 2px; border-radius: var(--radius-sm);
    display: flex; align-items: center; transition: all 120ms ease;
  }
  .collection-request-edit-btn:hover { background: var(--bg-panel-alt); color: var(--text-secondary); }
  .collection-request-delete-btn:hover { background: rgba(220,53,69,0.1); color: var(--error); }

  /* Resize handle */
  .sidebar-resize-handle-vertical {
    height: 4px; background: var(--border-subtle);
    cursor: ns-resize; flex-shrink: 0; transition: background 120ms ease;
  }
  .sidebar-resize-handle-vertical:hover { background: var(--border-color); }

  /* History */
  .sidebar-section {
    flex: 1; display: flex; flex-direction: column;
    min-height: 200px; overflow: hidden;
    border-top: 1px solid var(--border-subtle);
  }
  .section-header {
    display: flex; align-items: center; justify-content: space-between;
    padding: var(--spacing-lg); border-bottom: 1px solid var(--border-subtle);
  }
  .section-header h3 {
    font-size: 11px; font-weight: 600; color: var(--text-muted);
    text-transform: uppercase; letter-spacing: 0.8px; margin: 0;
  }
  .section-actions { display: flex; gap: var(--spacing-xs); }

  .history-list { flex: 1; overflow-y: auto; padding: var(--spacing-sm); }
  .history-item {
    width: 100%; padding: 10px var(--spacing-md); margin-bottom: var(--spacing-xs);
    background: transparent; border: none; border-radius: var(--radius-md);
    cursor: pointer; transition: background 120ms ease; text-align: left;
  }
  .history-item:hover { background: var(--bg-panel-alt); }
  .history-item-header {
    display: flex; align-items: center; gap: var(--spacing-sm);
    margin-bottom: var(--spacing-xs);
  }
  .history-status { font-size: 11px; font-weight: 600; margin-left: auto; opacity: 0.7; }
  .history-status.success { color: var(--status-success); }
  .history-status.error   { color: var(--status-server-error); }
  .history-url {
    font-size: 11px; color: var(--text-muted);
    white-space: nowrap; overflow: hidden; text-overflow: ellipsis;
    font-family: var(--font-mono);
  }
  .history-time { font-size: 11px; color: var(--text-muted); margin-top: var(--spacing-xs); }
</style>

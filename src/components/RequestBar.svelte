<script>
  import { createEventDispatcher } from 'svelte';
  import {
    request, response, collections, envVars,
    updateRequest, loadHistory, getInvoke, getDialog,
    validateJson, getUnresolvedVars, saveCollections,
  } from '../store.js';

  export let darkTheme = true;
  const dispatch = createEventDispatcher();

  let sending = false;
  let notification = null;

  const METHODS = ['GET', 'POST', 'PUT', 'DELETE', 'PATCH'];

  // ── Send request ─────────────────────────────────────────────────────────────
  async function handleSend() {
    if (!$request.url?.trim()) { alert('Please enter a URL'); return; }

    const method = $request.method.toUpperCase();
    // JSON body validation
    if (method !== 'GET' && method !== 'DELETE') {
      const ctHeader = $request.headers.find(h => h.enabled && h.key.toLowerCase() === 'content-type');
      if (ctHeader?.value.includes('application/json') && !validateJson($request.body)) {
        alert('Invalid JSON body'); return;
      }
    }

    // Auth validation
    if ($request.auth.type === 'bearer' && !$request.auth.token) { alert('Bearer token is required'); return; }
    if ($request.auth.type === 'basic' && (!$request.auth.username || !$request.auth.password)) {
      alert('Username and password are required for Basic auth'); return;
    }

    const invoke = getInvoke();
    if (!invoke) { alert('Tauri invoke not found. Are you running outside the desktop app?'); return; }

    sending = true;
    try {
      const req = JSON.parse(JSON.stringify($request));
      const res = await invoke('send_request', { req });
      response.set(res);
      await loadHistory();
    } catch (err) {
      alert('Request failed: ' + (err.message ?? err));
    } finally {
      sending = false;
    }
  }

  // ── cURL export ───────────────────────────────────────────────────────────────
  async function handleExportCurl() {
    if (!$request.url?.trim()) { alert('Please enter a URL'); return; }
    const invoke = getInvoke();
    if (!invoke) { alert('Tauri invoke not found'); return; }
    try {
      const cmd = await invoke('export_curl', { req: JSON.parse(JSON.stringify($request)) });
      await navigator.clipboard.writeText(cmd);
      showNotification('cURL copied to clipboard');
    } catch (err) {
      alert('Failed to export cURL: ' + err);
    }
  }

  // ── Save / Load ───────────────────────────────────────────────────────────────
  async function handleSave() {
    if (!$request.url?.trim()) { alert('Please enter a URL before saving'); return; }
    const invoke = getInvoke();
    if (!invoke) { alert('Tauri invoke not found'); return; }
    try {
      await invoke('save_request', { req: JSON.parse(JSON.stringify($request)) });
      showNotification('Request saved');
    } catch (err) {
      if (!String(err).includes('No file selected')) alert('Failed to save: ' + err);
    }
  }

  async function handleLoad() {
    const invoke = getInvoke();
    if (!invoke) { alert('Tauri invoke not found'); return; }
    try {
      const loaded = await invoke('load_request');
      if (loaded) { request.set(loaded); showNotification('Request loaded'); }
    } catch (err) {
      if (!String(err).includes('No file selected')) alert('Failed to load: ' + err);
    }
  }

  // ── Save to collection ────────────────────────────────────────────────────────
  async function handleSaveToCollection() {
    if (!$request.url?.trim()) { alert('Please enter a URL'); return; }
    if ($collections.length === 0) { alert('No collections. Create one first.'); return; }
    const names = $collections.map(c => c.name);
    const selected = prompt(`Save to collection:\n${names.join('\n')}`);
    const idx = names.indexOf(selected);
    if (idx < 0) { alert('Collection not found'); return; }
    const name = prompt('Request name:', $request.url);
    if (!name?.trim()) return;
    collections.update(cols => {
      const updated = [...cols];
      updated[idx].requests.push({ ...JSON.parse(JSON.stringify($request)), name: name.trim() });
      localStorage.setItem('collections', JSON.stringify(updated));
      return updated;
    });
    showNotification(`Saved to "${$collections[idx].name}"`);
  }

  // ── Notification helper ───────────────────────────────────────────────────────
  function showNotification(msg) {
    notification = msg;
    setTimeout(() => (notification = null), 2500);
  }

  // ── Keyboard handler (global) ─────────────────────────────────────────────────
  import { onMount } from 'svelte';
  onMount(() => {
    function onKey(e) {
      if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') { e.preventDefault(); handleSend(); }
      if ((e.ctrlKey || e.metaKey) && !e.shiftKey && e.key === 's') { e.preventDefault(); handleSave(); }
      if ((e.ctrlKey || e.metaKey) && !e.shiftKey && e.key === 'o') { e.preventDefault(); handleLoad(); }
      if ((e.ctrlKey || e.metaKey) && e.shiftKey && e.key === 'S') { e.preventDefault(); handleSaveToCollection(); }
    }
    window.addEventListener('keydown', onKey);
    return () => window.removeEventListener('keydown', onKey);
  });
</script>

<section class="request-bar">
  <div class="request-controls">
    <!-- Sidebar toggle -->
    <button class="btn-icon" title="Toggle sidebar" on:click={() => dispatch('toggleSidebar')}>
      <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
        <path d="M2 4h12M2 8h12M2 12h12" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
      </svg>
    </button>

    <!-- Method select -->
    <select class="method-select" bind:value={$request.method}>
      {#each METHODS as m}
        <option value={m}>{m}</option>
      {/each}
    </select>

    <!-- URL input -->
    <input
      type="text"
      class="url-input"
      class:unresolved-var={getUnresolvedVars($request.url, $envVars).length > 0}
      placeholder="https://api.example.com/endpoint"
      bind:value={$request.url}
      on:keydown={e => e.key === 'Enter' && handleSend()}
    />

    <!-- Send -->
    <button class="btn-primary btn-send" disabled={sending} on:click={handleSend}>
      {#if sending}
        <svg class="spinner" width="16" height="16" viewBox="0 0 16 16">
          <circle cx="8" cy="8" r="6" stroke="currentColor" stroke-width="2" fill="none" stroke-dasharray="30" stroke-linecap="round"/>
        </svg>
      {:else}
        Send
      {/if}
    </button>

    <!-- cURL -->
    <button class="btn-secondary" title="Copy as cURL" on:click={handleExportCurl}>
      <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
        <path d="M13.5 2h-11C1.67 2 1 2.67 1 3.5v9c0 .83.67 1.5 1.5 1.5h11c.83 0 1.5-.67 1.5-1.5v-9c0-.83-.67-1.5-1.5-1.5zm-11 1h11c.28 0 .5.22.5.5v1.5H2V3.5c0-.28.22-.5.5-.5zm11 10h-11c-.28 0-.5-.22-.5-.5V6h12v6.5c0 .28-.22.5-.5.5z" fill="currentColor"/>
        <path d="M4 8h2v1H4zm3 0h5v1H7zm-3 2h5v1H4z" fill="currentColor"/>
      </svg>
    </button>

    <!-- Save -->
    <button class="btn-secondary" title="Save Request (Ctrl+S)" on:click={handleSave}>
      <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
        <path d="M13 2H3c-.55 0-1 .45-1 1v10c0 .55.45 1 1 1h10c.55 0 1-.45 1-1V3c0-.55-.45-1-1-1zm-1 1v3H4V3h8zM3 13V7h10v6H3z" fill="currentColor"/>
        <path d="M5 4h1v2H5zm5 6H6v3h4v-3z" fill="currentColor"/>
      </svg>
    </button>

    <!-- Save to collection -->
    <button class="btn-secondary" title="Save to Collection (Ctrl+Shift+S)" on:click={handleSaveToCollection}>
      <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
        <path d="M2 3h12v10H2z" fill="currentColor"/>
        <path d="M4 5h8v1H4zm0 2h6v1H4zm0 2h8v1H4z" fill="currentColor"/>
      </svg>
    </button>

    <!-- Load -->
    <button class="btn-secondary" title="Load Request (Ctrl+O)" on:click={handleLoad}>
      <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
        <path d="M13 2H3c-.55 0-1 .45-1 1v10c0 .55.45 1 1 1h10c.55 0 1-.45 1-1V3c0-.55-.45-1-1-1zm0 11H3V3h10v10z" fill="currentColor"/>
        <path d="M11 7H9V5H7v2H5l3 3 3-3z" fill="currentColor"/>
      </svg>
    </button>
  </div>
</section>

{#if notification}
  <div class="notification success">{notification}</div>
{/if}

<style>
  .request-bar {
    padding: var(--spacing-xl);
    background: var(--bg-panel);
    border-bottom: 1px solid var(--border-subtle);
  }
  .request-controls {
    display: flex;
    gap: var(--spacing-md);
    align-items: center;
    padding: var(--spacing-xs);
    background: var(--bg-panel-alt);
    border-radius: var(--radius-md);
    box-shadow: inset 0 1px 2px rgba(0,0,0,0.2);
  }
  .method-select {
    background: var(--bg-panel); border: 1px solid var(--border-color);
    border-radius: var(--radius-sm); padding: var(--spacing-md);
    font-weight: 600; font-family: var(--font-mono); cursor: pointer;
    transition: background 120ms ease; width: 110px; flex-shrink: 0;
  }
  .method-select:hover { background: var(--bg-hover); }
  .url-input {
    flex: 1;
    background: var(--bg-panel); border: 1px solid var(--border-color);
    border-radius: var(--radius-sm); padding: var(--spacing-md) var(--spacing-lg);
    font-family: var(--font-mono); font-size: var(--font-size-sm);
    transition: background 120ms ease, border-color 120ms ease, box-shadow 120ms ease;
  }
  .url-input:hover { background: var(--bg-hover); }
  .url-input:focus { border-color: var(--accent); background: var(--bg-hover); box-shadow: 0 0 0 3px var(--accent-muted); }

  .btn-send { min-width: 80px; flex-shrink: 0; }

  @keyframes spin { to { transform: rotate(360deg); } }
  .spinner { animation: spin 1s linear infinite; transform-origin: center; }
</style>

<script>
  import { onMount } from 'svelte';
  import { loadHistory, loadEnvVars, loadCollections, loadAgent37Config } from './store.js';
  import Sidebar from './components/Sidebar.svelte';
  import RequestBar from './components/RequestBar.svelte';
  import RequestEditor from './components/RequestEditor.svelte';
  import ResponseViewer from './components/ResponseViewer.svelte';
  import Agent37Panel from './components/Agent37Panel.svelte';
  import { response } from './store.js';

  let sidebarCollapsed = false;
  let sidebarWidth = 280;
  let isResizing = false;
  let darkTheme = true;
  let settingsOpen = false;

  onMount(async () => {
    await Promise.all([loadHistory(), loadEnvVars(), loadAgent37Config()]);
    loadCollections();

    // Verify Test API server status on startup
    fetch('http://localhost:3000/api/users')
      .then(res => res.ok ? res.json() : null)
      .then(data => {
        if (data) {
          console.log('[Startup Test] Test API active on http://localhost:3000 (users count:', data.length, ')');
        }
      })
      .catch(err => {
        console.warn('[Startup Test] Local Test API ping skipped or unreachable:', err.message);
      });

    // Keyboard shortcuts
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  });

  function handleKeyDown(e) {
    if ((e.ctrlKey || e.metaKey) && e.key === 'b') {
      e.preventDefault();
      sidebarCollapsed = !sidebarCollapsed;
    }
    if ((e.ctrlKey || e.metaKey) && e.key === ',') {
      e.preventDefault();
      settingsOpen = !settingsOpen;
    }
  }

  function toggleTheme() {
    darkTheme = !darkTheme;
    document.body.classList.toggle('light-theme', !darkTheme);
  }

  // Sidebar drag-resize
  function startResize(e) {
    isResizing = true;
    document.addEventListener('mousemove', onMouseMove);
    document.addEventListener('mouseup', stopResize);
  }

  function onMouseMove(e) {
    if (!isResizing) return;
    const newWidth = Math.min(Math.max(e.clientX, 180), 480);
    sidebarWidth = newWidth;
  }

  function stopResize() {
    isResizing = false;
    document.removeEventListener('mousemove', onMouseMove);
    document.removeEventListener('mouseup', stopResize);
  }
</script>

<div
  class="app-container"
  class:collapsed={sidebarCollapsed}
  style="--sidebar-width: {sidebarCollapsed ? 0 : sidebarWidth}px"
>
  {#if !sidebarCollapsed}
    <Sidebar on:toggleTheme={toggleTheme} {darkTheme} />
    <!-- Resizer -->
    <div
      class="resizer horizontal"
      class:resizing={isResizing}
      on:mousedown={startResize}
      role="separator"
      aria-orientation="vertical"
      aria-label="Resize sidebar"
    ></div>
  {/if}

  {#if sidebarCollapsed}
    <button
      class="btn-icon collapsed-toggle"
      title="Show sidebar"
      aria-label="Show sidebar"
      on:click={() => (sidebarCollapsed = false)}
    >
      <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
        <path d="M6 4l6 4-6 4" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
    </button>
  {/if}

  <main class="main-content">
    <RequestBar
      on:toggleSidebar={() => (sidebarCollapsed = !sidebarCollapsed)}
      on:toggleTheme={toggleTheme}
      on:openSettings={() => (settingsOpen = !settingsOpen)}
      {darkTheme}
    />
    <RequestEditor />
    {#if $response}
      <ResponseViewer />
    {/if}
  </main>
</div>

<!-- ── Settings drawer ──────────────────────────────────────────────────────── -->
{#if settingsOpen}
  <!-- Backdrop -->
  <div
    class="settings-backdrop"
    role="presentation"
    on:click={() => (settingsOpen = false)}
  ></div>

  <!-- Drawer -->
  <aside class="settings-drawer" aria-label="Settings">
    <div class="settings-drawer-header">
      <h2>Settings</h2>
      <button
        class="btn-icon"
        title="Close settings (Ctrl+,)"
        aria-label="Close settings"
        on:click={() => (settingsOpen = false)}
      >
        <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
          <path d="M3 3l10 10M13 3L3 13" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
        </svg>
      </button>
    </div>

    <div class="settings-section-label">Agent37 Runner</div>
    <div class="settings-drawer-body">
      <Agent37Panel />
    </div>
  </aside>
{/if}

<style>
  :global(*) {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
  }
  :global(html, body) {
    height: 100%;
    overflow: hidden;
  }
  :global(body) {
    font-family: var(--font-sans);
    font-size: var(--font-size-base);
    color: var(--text-primary);
    background: var(--bg-root);
    line-height: 1.5;
    -webkit-font-smoothing: antialiased;
  }
  :global(code, pre) { font-family: var(--font-mono); }
  :global(input, textarea, select, button) { font-family: inherit; font-size: inherit; color: inherit; }
  :global(input:focus, textarea:focus, select:focus) { outline: none; }
  :global(button:focus-visible) { outline: 2px solid var(--accent); outline-offset: 2px; }
  :global(::placeholder) { color: var(--text-muted); opacity: 0.6; }
  :global(::-webkit-scrollbar) { width: 8px; height: 8px; }
  :global(::-webkit-scrollbar-track) { background: transparent; }
  :global(::-webkit-scrollbar-thumb) { background: rgba(255,255,255,0.1); border-radius: var(--radius-sm); }
  :global(::-webkit-scrollbar-thumb:hover) { background: rgba(255,255,255,0.15); }
  :global(body.light-theme ::-webkit-scrollbar-thumb) { background: rgba(0,0,0,0.2); }
  :global(.empty-state) { padding: var(--spacing-2xl) var(--spacing-xl); text-align: center; color: var(--text-muted); font-size: var(--font-size-sm); }
  :global(.unresolved-var) { color: var(--warning) !important; text-decoration: underline wavy var(--warning); }

  .app-container {
    display: grid;
    grid-template-columns: var(--sidebar-width) 4px 1fr;
    height: 100vh;
    overflow: hidden;
    transition: grid-template-columns 0.3s ease;
  }
  .app-container.collapsed {
    grid-template-columns: 0px 0px 1fr;
  }
  .resizer.horizontal {
    width: 4px;
    height: 100%;
    cursor: col-resize;
    background: transparent;
    transition: background 0.2s;
    z-index: 10;
  }
  .resizer.horizontal:hover,
  .resizer.horizontal.resizing {
    background: var(--accent);
  }
  .collapsed-toggle {
    position: absolute;
    top: 16px;
    left: 16px;
    z-index: 10;
    background: var(--bg-panel);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-sm);
    padding: var(--spacing-sm);
  }
  .main-content {
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  /* ── Settings drawer ──────────────────────────────────────────────────────── */
  .settings-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.45);
    z-index: 200;
    animation: fadeBackdrop 0.18s ease-out;
  }
  @keyframes fadeBackdrop {
    from { opacity: 0; }
    to   { opacity: 1; }
  }

  .settings-drawer {
    position: fixed;
    top: 0;
    right: 0;
    bottom: 0;
    width: 400px;
    max-width: 92vw;
    background: var(--bg-panel);
    border-left: 1px solid var(--border-color);
    box-shadow: -4px 0 24px rgba(0, 0, 0, 0.4);
    z-index: 201;
    display: flex;
    flex-direction: column;
    animation: slideDrawer 0.2s ease-out;
  }
  @keyframes slideDrawer {
    from { transform: translateX(100%); }
    to   { transform: translateX(0); }
  }

  .settings-drawer-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--spacing-lg) var(--spacing-xl);
    border-bottom: 1px solid var(--border-subtle);
    flex-shrink: 0;
  }
  .settings-drawer-header h2 {
    font-size: 14px;
    font-weight: 600;
    color: var(--text-primary);
    margin: 0;
  }

  .settings-section-label {
    font-size: 11px;
    font-weight: 600;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.8px;
    padding: var(--spacing-lg) var(--spacing-xl) var(--spacing-sm);
    flex-shrink: 0;
  }

  .settings-drawer-body {
    flex: 1;
    overflow-y: auto;
    padding: 0 var(--spacing-xl) var(--spacing-xl);
  }
</style>

<script>
  import { response } from '../store.js';

  let activeTab = 'body';
  let prettyPrint = true;
  let fullscreen = false;

  $: res = $response;
  $: statusClass = res ? `status-${Math.floor(res.status / 100)}xx` : '';

  function formatJSON(obj) {
    const json = typeof obj === 'string' ? obj : JSON.stringify(obj, null, 2);
    return json
      .replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;')
      .replace(/"([^"]+)":/g, '<span class="json-key">"$1"</span>:')
      .replace(/: "([^"]*)"/g, ': <span class="json-string">"$1"</span>')
      .replace(/: (\d+\.?\d*)/g, ': <span class="json-number">$1</span>')
      .replace(/: (true|false)/g, ': <span class="json-boolean">$1</span>')
      .replace(/: null/g, ': <span class="json-null">null</span>');
  }

  function formatXML(xml) {
    return xml.replace(/></g, '>\n<').trim();
  }

  function getContentType(headers) {
    if (!headers) return null;
    const key = Object.keys(headers).find(k => k.toLowerCase() === 'content-type');
    return key ? headers[key] : null;
  }

  $: bodyContent = (() => {
    if (!res?.body) return { html: false, text: '(No content)' };
    const ct = getContentType(res.headers);
    const isJson = ct?.includes('application/json');
    const isXml  = ct?.includes('xml');
    const isHtml = ct?.includes('text/html');

    if (prettyPrint) {
      if (isJson && typeof res.body === 'object') return { html: true, text: formatJSON(res.body) };
      if (isJson && typeof res.body === 'string') {
        try { return { html: true, text: formatJSON(JSON.parse(res.body)) }; } catch {}
      }
      if (isXml) return { html: false, text: formatXML(String(res.body)) };
      if (isHtml) return { html: true, text: String(res.body) };
    }
    return { html: false, text: typeof res.body === 'string' ? res.body : JSON.stringify(res.body) };
  })();

  $: headersContent = res?.headers ? formatJSON(res.headers) : '';
</script>

<section class="response-viewer" class:fullscreen>
  <!-- Meta bar -->
  <div class="response-header">
    <div class="response-meta">
      {#if res}
        <span class="status-badge {statusClass}">{res.status} {res.statusText ?? ''}</span>
        <span class="meta-item">Time: <strong>{res.time}ms</strong></span>
        <span class="meta-item">Size: <strong>{res.size} KB</strong></span>
      {/if}
    </div>
  </div>

  <!-- Response tabs -->
  <div class="tabs" role="tablist">
    <button class="tab-btn" class:active={activeTab === 'body'}    role="tab" on:click={() => (activeTab = 'body')}>Body</button>
    <button class="tab-btn" class:active={activeTab === 'headers'} role="tab" on:click={() => (activeTab = 'headers')}>Headers</button>
  </div>

  <!-- Controls -->
  <div class="response-controls">
    <button class="btn-secondary btn-small" class:active={prettyPrint}  on:click={() => (prettyPrint = true)}>Pretty</button>
    <button class="btn-secondary btn-small" class:active={!prettyPrint} on:click={() => (prettyPrint = false)}>Raw</button>
    <button class="btn-secondary btn-small" title="Toggle fullscreen" on:click={() => (fullscreen = !fullscreen)}>
      {#if fullscreen}
        <svg width="14" height="14" viewBox="0 0 16 16" fill="none">
          <path d="M6 10H2v4M10 10h4v4M6 6H2V2M10 6h4V2" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
        </svg>
      {:else}
        <svg width="14" height="14" viewBox="0 0 16 16" fill="none">
          <path d="M3 7V3h4M13 7V3h-4M3 13h4v-4M13 13h-4v-4" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
        </svg>
      {/if}
    </button>
  </div>

  <!-- Body panel -->
  {#if activeTab === 'body'}
    <div class="response-body-wrap">
      <pre class="response-body">
        {#if bodyContent.html}
          <code>{@html bodyContent.text}</code>
        {:else}
          <code>{bodyContent.text}</code>
        {/if}
      </pre>
    </div>
  {:else}
    <div class="response-body-wrap">
      <pre class="response-body"><code>{@html headersContent}</code></pre>
    </div>
  {/if}
</section>

<style>
  .response-viewer {
    flex: 1; display: flex; flex-direction: column;
    overflow: hidden; background: var(--bg-root);
  }
  .response-viewer.fullscreen {
    position: fixed; top: 0; left: 0;
    width: 100vw; height: 100vh; z-index: 1000; background: var(--bg-root);
  }
  .response-header {
    padding: var(--spacing-lg); background: var(--bg-root);
    border-bottom: 1px solid var(--border-subtle);
  }
  .response-meta { display: flex; align-items: center; gap: var(--spacing-lg); flex-wrap: wrap; }
  .meta-item { font-size: 12px; color: var(--text-muted); }
  .meta-item strong { color: var(--text-secondary); font-family: var(--font-mono); font-weight: 500; }

  .response-controls {
    display: flex; gap: var(--spacing-sm);
    padding: var(--spacing-sm) var(--spacing-lg);
    border-bottom: 1px solid var(--border-subtle);
  }
  .btn-small { padding: var(--spacing-xs) var(--spacing-sm); font-size: var(--font-size-xs); }
  .btn-small.active { border-color: var(--accent); color: var(--accent); }

  .response-body-wrap { flex: 1; overflow: auto; }
  .response-body {
    margin: 0; padding: var(--spacing-xl);
    background: var(--bg-panel); font-size: var(--font-size-sm);
    line-height: 1.7; color: var(--text-primary);
  }
  .response-body code {
    display: block; white-space: pre; font-family: var(--font-mono);
  }
</style>

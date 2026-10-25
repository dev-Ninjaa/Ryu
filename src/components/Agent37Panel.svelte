<script>
  import {
    agent37Config,
    agent37TokenStored,
    agent37SetRunnerUrl,
    agent37SetToken,
    agent37DeleteToken,
    agent37TestConnection,
    loadAgent37Config,
  } from '../store.js';

  // ── Local state ──────────────────────────────────────────────────────────────
  let urlInput = '';
  let tokenInput = '';
  let showToken = false;

  /** @type {'idle'|'testing'|'saving'} */
  let busy = 'idle';

  /**
   * Connection result object returned by the Rust command, or null.
   * Shape: { kind: string, runnerUrl?, version?, status?, message? }
   */
  let connectionResult = null;

  // Keep local URL input in sync with the persisted value.
  $: urlInput = $agent37Config?.runnerUrl ?? '';

  // ── Handlers ─────────────────────────────────────────────────────────────────

  async function handleSaveUrl() {
    if (busy !== 'idle') return;
    busy = 'saving';
    connectionResult = null;
    try {
      await agent37SetRunnerUrl(urlInput.trim());
    } catch (e) {
      connectionResult = { kind: 'invalid_url', message: String(e) };
    } finally {
      busy = 'idle';
    }
  }

  async function handleSaveToken() {
    if (busy !== 'idle') return;
    const val = tokenInput.trim();
    if (!val) return;
    busy = 'saving';
    try {
      await agent37SetToken(val);
      tokenInput = '';
    } catch (e) {
      connectionResult = { kind: 'network_error', message: String(e) };
    } finally {
      busy = 'idle';
    }
  }

  async function handleDeleteToken() {
    if (busy !== 'idle') return;
    busy = 'saving';
    connectionResult = null;
    try {
      await agent37DeleteToken();
    } catch (e) {
      connectionResult = { kind: 'network_error', message: String(e) };
    } finally {
      busy = 'idle';
    }
  }

  async function handleTestConnection() {
    if (busy !== 'idle') return;
    busy = 'testing';
    connectionResult = null;
    try {
      connectionResult = await agent37TestConnection();
    } catch (e) {
      connectionResult = { kind: 'network_error', message: String(e) };
    } finally {
      busy = 'idle';
    }
  }

  // ── Result rendering helpers ──────────────────────────────────────────────────

  function resultClass(result) {
    if (!result) return '';
    return result.kind === 'ok' ? 'result-ok' : 'result-error';
  }

  function resultIcon(result) {
    if (!result) return '';
    return result.kind === 'ok' ? '✓' : '✕';
  }

  function resultSummary(result) {
    if (!result) return '';
    switch (result.kind) {
      case 'ok':
        return `Connected — ryu-agent37-runner v${result.version}`;
      case 'http_error':
        return `HTTP ${result.status}: ${result.message}`;
      case 'invalid_response':
        return `Unexpected response: ${result.message}`;
      case 'invalid_url':
        return `Invalid URL: ${result.message}`;
      case 'timeout':
        return `Timeout: ${result.message}`;
      case 'network_error':
        return `Network error: ${result.message}`;
      default:
        return result.message ?? 'Unknown error';
    }
  }
</script>

<!-- ── Info banner ──────────────────────────────────────────────────────────── -->
<div class="info-box">
  <p>
    Agent37 is a remote Node.js runner. Configure its base URL and bearer token
    here. The token is stored in the OS credential vault and never written to
    disk or logs.
  </p>
  <p class="info-note">
    Local HTTP execution remains the default. Agent37 remote execution is not
    active yet.
  </p>
</div>

<!-- ── Runner URL ──────────────────────────────────────────────────────────── -->
<section class="config-section">
  <label class="field-label" for="runner-url">Runner base URL</label>
  <div class="field-row">
    <input
      id="runner-url"
      class="field-input"
      type="url"
      placeholder="https://host-3738.agent37.app"
      bind:value={urlInput}
      on:keydown={e => e.key === 'Enter' && handleSaveUrl()}
      autocomplete="off"
      spellcheck="false"
    />
    <button
      class="btn-secondary"
      disabled={busy !== 'idle'}
      on:click={handleSaveUrl}
    >
      {busy === 'saving' ? 'Saving…' : 'Save'}
    </button>
  </div>
  <p class="field-hint">
    Supply the root URL only — no path. Example:
    <code>https://abc123-3738.agent37.app</code>
  </p>
</section>

<!-- ── Bearer token ────────────────────────────────────────────────────────── -->
<section class="config-section">
  <label class="field-label" for="runner-token">Bearer token</label>

  <div class="token-status">
    {#if $agent37TokenStored}
      <span class="token-stored-badge">Token stored in OS keychain</span>
      <button
        class="btn-danger-sm"
        disabled={busy !== 'idle'}
        on:click={handleDeleteToken}
      >Remove</button>
    {:else}
      <span class="token-absent-badge">No token stored</span>
    {/if}
  </div>

  <div class="field-row">
    <div class="token-input-wrap">
      <input
        id="runner-token"
        class="field-input"
        type={showToken ? 'text' : 'password'}
        placeholder={$agent37TokenStored ? 'Enter a new token to replace…' : 'Paste bearer token…'}
        bind:value={tokenInput}
        on:keydown={e => e.key === 'Enter' && handleSaveToken()}
        autocomplete="off"
        spellcheck="false"
      />
      <button
        class="btn-icon btn-reveal"
        title={showToken ? 'Hide token' : 'Show token'}
        on:click={() => (showToken = !showToken)}
        tabindex="-1"
      >
        {#if showToken}
          <!-- eye-off -->
          <svg width="14" height="14" viewBox="0 0 16 16" fill="none">
            <path d="M2 2l12 12M6.5 6.6A2 2 0 0 0 9.4 9.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
            <path d="M4.3 4.4C2.9 5.3 1.7 6.5 1 8c1.3 3 4.2 5 7 5 1.2 0 2.3-.3 3.3-.9M7 3.1c.3 0 .7-.1 1-.1 2.8 0 5.7 2 7 5-.4.8-.9 1.5-1.5 2.1" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
          </svg>
        {:else}
          <!-- eye -->
          <svg width="14" height="14" viewBox="0 0 16 16" fill="none">
            <path d="M1 8c1.3-3 4.2-5 7-5s5.7 2 7 5c-1.3 3-4.2 5-7 5S2.3 11 1 8z" stroke="currentColor" stroke-width="1.5"/>
            <circle cx="8" cy="8" r="2" stroke="currentColor" stroke-width="1.5"/>
          </svg>
        {/if}
      </button>
    </div>
    <button
      class="btn-secondary"
      disabled={busy !== 'idle' || !tokenInput.trim()}
      on:click={handleSaveToken}
    >
      {busy === 'saving' ? 'Saving…' : 'Save Token'}
    </button>
  </div>
  <p class="field-hint">
    The token is written directly to the OS credential vault — it never
    touches disk, source code, or exported settings.
  </p>
</section>

<!-- ── Test Connection ─────────────────────────────────────────────────────── -->
<section class="config-section">
  <div class="test-row">
    <button
      class="btn-primary test-btn"
      disabled={busy !== 'idle' || !urlInput.trim()}
      on:click={handleTestConnection}
    >
      {#if busy === 'testing'}
        <span class="spinner" aria-hidden="true"></span>
        Testing…
      {:else}
        Test Connection
      {/if}
    </button>
    {#if !$agent37TokenStored}
      <span class="test-hint">Save a token first</span>
    {/if}
  </div>

  {#if connectionResult}
    <div class="result-banner {resultClass(connectionResult)}" role="status">
      <span class="result-icon" aria-hidden="true">{resultIcon(connectionResult)}</span>
      <span class="result-text">{resultSummary(connectionResult)}</span>
    </div>
  {/if}

  <p class="field-hint disclaimer">
    A live Agent37 connection is only confirmed after an authenticated
    end-to-end test succeeds. The public runner URL is not hardcoded and must
    be verified separately.
  </p>
</section>

<style>
  /* ── Layout ─────────────────────────────────────────────────────────────── */
  .info-box {
    background: var(--bg-panel-alt);
    border-radius: var(--radius-md);
    border: 1px solid var(--border-color);
    padding: var(--spacing-md) var(--spacing-lg);
    margin-bottom: var(--spacing-lg);
  }
  .info-box p {
    margin: 0;
    font-size: var(--font-size-sm);
    color: var(--text-secondary);
    line-height: 1.6;
  }
  .info-box p + p { margin-top: var(--spacing-sm); }
  .info-note { color: var(--text-muted) !important; font-style: italic; }

  .config-section {
    margin-bottom: var(--spacing-xl);
  }

  /* ── Fields ─────────────────────────────────────────────────────────────── */
  .field-label {
    display: block;
    font-size: var(--font-size-xs);
    font-weight: 600;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.6px;
    margin-bottom: var(--spacing-sm);
  }

  .field-row {
    display: flex;
    gap: var(--spacing-sm);
    align-items: center;
  }

  .field-input {
    flex: 1;
    background: var(--bg-panel);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-sm);
    padding: var(--spacing-sm) var(--spacing-md);
    font-size: var(--font-size-sm);
    font-family: var(--font-mono);
    color: var(--text-primary);
    transition: border-color var(--transition-fast), background var(--transition-fast);
  }
  .field-input:hover { background: var(--bg-panel-alt); }
  .field-input:focus { border-color: var(--accent); background: var(--bg-panel-alt); outline: none; }

  .field-hint {
    margin-top: var(--spacing-sm);
    font-size: var(--font-size-xs);
    color: var(--text-muted);
    line-height: 1.5;
  }
  .field-hint code {
    font-family: var(--font-mono);
    background: var(--bg-root);
    padding: 1px 5px;
    border-radius: var(--radius-sm);
    font-size: 10px;
    color: var(--accent);
  }
  .disclaimer { font-style: italic; }

  /* ── Token ──────────────────────────────────────────────────────────────── */
  .token-status {
    display: flex;
    align-items: center;
    gap: var(--spacing-sm);
    margin-bottom: var(--spacing-sm);
  }
  .token-stored-badge {
    font-size: var(--font-size-xs);
    color: var(--success);
    background: rgba(63, 185, 80, 0.1);
    border: 1px solid rgba(63, 185, 80, 0.25);
    border-radius: var(--radius-sm);
    padding: 2px 8px;
    font-weight: 500;
  }
  .token-absent-badge {
    font-size: var(--font-size-xs);
    color: var(--text-muted);
    background: var(--bg-panel-alt);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-sm);
    padding: 2px 8px;
  }

  .token-input-wrap {
    flex: 1;
    position: relative;
    display: flex;
    align-items: center;
  }
  .token-input-wrap .field-input { width: 100%; padding-right: 36px; }

  .btn-reveal {
    position: absolute;
    right: 6px;
    color: var(--text-muted);
    padding: 4px;
  }
  .btn-reveal:hover { color: var(--text-secondary); background: none; }

  .btn-danger-sm {
    background: none;
    border: 1px solid rgba(248, 81, 73, 0.4);
    color: var(--error);
    border-radius: var(--radius-sm);
    padding: 2px 10px;
    font-size: var(--font-size-xs);
    cursor: pointer;
    transition: background var(--transition-fast);
  }
  .btn-danger-sm:hover { background: rgba(248, 81, 73, 0.1); }
  .btn-danger-sm:disabled { opacity: 0.4; cursor: not-allowed; }

  /* ── Test Connection ─────────────────────────────────────────────────────── */
  .test-row {
    display: flex;
    align-items: center;
    gap: var(--spacing-md);
    margin-bottom: var(--spacing-md);
  }
  .test-btn {
    min-width: 140px;
    display: inline-flex;
    align-items: center;
    gap: var(--spacing-sm);
  }
  .test-hint {
    font-size: var(--font-size-xs);
    color: var(--text-muted);
  }

  /* Spinner */
  .spinner {
    display: inline-block;
    width: 12px;
    height: 12px;
    border: 2px solid rgba(255, 255, 255, 0.3);
    border-top-color: #fff;
    border-radius: 50%;
    animation: spin 0.7s linear infinite;
  }
  @keyframes spin { to { transform: rotate(360deg); } }

  /* ── Result banner ───────────────────────────────────────────────────────── */
  .result-banner {
    display: flex;
    align-items: flex-start;
    gap: var(--spacing-sm);
    padding: var(--spacing-sm) var(--spacing-md);
    border-radius: var(--radius-md);
    font-size: var(--font-size-sm);
    line-height: 1.5;
    border: 1px solid transparent;
    margin-bottom: var(--spacing-sm);
    animation: fadeIn 0.2s ease-out;
  }
  @keyframes fadeIn { from { opacity: 0; transform: translateY(-4px); } to { opacity: 1; transform: none; } }

  .result-ok {
    background: rgba(63, 185, 80, 0.1);
    border-color: rgba(63, 185, 80, 0.25);
    color: var(--success);
  }
  .result-error {
    background: rgba(248, 81, 73, 0.08);
    border-color: rgba(248, 81, 73, 0.25);
    color: var(--error);
  }

  .result-icon {
    font-weight: 700;
    flex-shrink: 0;
    line-height: 1.5;
  }
  .result-text { flex: 1; word-break: break-word; }
</style>

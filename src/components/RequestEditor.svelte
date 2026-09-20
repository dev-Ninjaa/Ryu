<script>
  import { request, envVars, updateRequest, updateAuth, getUnresolvedVars } from '../store.js';
  import EnvVarsPanel from './EnvVarsPanel.svelte';

  let activeTab = 'params';
  const TABS = ['params', 'auth', 'headers', 'body', 'env'];

  // ── Auth ──────────────────────────────────────────────────────────────────────
  const AUTH_TYPES = [
    { value: 'none',   label: 'No Auth' },
    { value: 'bearer', label: 'Bearer Token' },
    { value: 'apikey', label: 'API Key' },
    { value: 'basic',  label: 'Basic Auth' },
  ];

  // ── Body type ─────────────────────────────────────────────────────────────────
  let bodyType = 'raw';

  // ── KV helpers ────────────────────────────────────────────────────────────────
  function addParam() {
    updateRequest({ params: [...$request.params, { key: '', value: '', enabled: true }] });
  }
  function removeParam(i) {
    const params = [...$request.params];
    params.splice(i, 1);
    updateRequest({ params });
  }
  function updateParam(i, field, val) {
    const params = [...$request.params];
    params[i] = { ...params[i], [field]: val };
    updateRequest({ params });
  }

  function addHeader() {
    updateRequest({ headers: [...$request.headers, { key: '', value: '', enabled: true }] });
  }
  function removeHeader(i) {
    const headers = [...$request.headers];
    headers.splice(i, 1);
    updateRequest({ headers });
  }
  function updateHeader(i, field, val) {
    const headers = [...$request.headers];
    headers[i] = { ...headers[i], [field]: val };
    updateRequest({ headers });
  }

  // Form data
  function addFormData() {
    const fd = [...($request.formData ?? []), { key: '', value: '', enabled: true }];
    updateRequest({ formData: fd });
  }
  function removeFormData(i) {
    const fd = [...($request.formData ?? [])];
    fd.splice(i, 1);
    updateRequest({ formData: fd });
  }
  function updateFormData(i, field, val) {
    const fd = [...($request.formData ?? [])];
    fd[i] = { ...fd[i], [field]: val };
    updateRequest({ formData: fd });
  }
</script>

<section class="request-editor">
  <div class="editor-header">
    <div class="tabs" role="tablist">
      {#each TABS as tab}
        <button
          class="tab-btn"
          class:active={activeTab === tab}
          role="tab"
          aria-selected={activeTab === tab}
          on:click={() => (activeTab = tab)}
        >
          {tab.charAt(0).toUpperCase() + tab.slice(1)}
        </button>
      {/each}
    </div>
  </div>

  <div class="tab-content">
    <!-- Params -->
    {#if activeTab === 'params'}
      <div class="tab-panel">
        <div class="key-value-editor">
          {#each $request.params as param, i}
            <div class="kv-row">
              <input type="checkbox" checked={param.enabled} on:change={e => updateParam(i, 'enabled', e.target.checked)} />
              <input type="text" placeholder="key"
                class:unresolved-var={getUnresolvedVars(param.key, $envVars).length > 0}
                value={param.key}
                on:input={e => updateParam(i, 'key', e.target.value)} />
              <input type="text" placeholder="value"
                class:unresolved-var={getUnresolvedVars(param.value, $envVars).length > 0}
                value={param.value}
                on:input={e => updateParam(i, 'value', e.target.value)} />
              <button class="btn-icon-small btn-delete-kv" on:click={() => removeParam(i)}>×</button>
            </div>
          {/each}
          {#if $request.params.length === 0}
            <div class="empty-state">No parameters</div>
          {/if}
        </div>
        <button class="btn-secondary btn-add" on:click={addParam}>+ Add Parameter</button>
      </div>

    <!-- Auth -->
    {:else if activeTab === 'auth'}
      <div class="tab-panel">
        <div class="auth-selector">
          <label>Auth Type</label>
          <select class="auth-type-select" value={$request.auth.type} on:change={e => updateAuth({ type: e.target.value })}>
            {#each AUTH_TYPES as at}
              <option value={at.value}>{at.label}</option>
            {/each}
          </select>
        </div>
        <div class="auth-fields">
          {#if $request.auth.type === 'none'}
            <div class="empty-state">No authentication</div>
          {:else if $request.auth.type === 'bearer'}
            <div class="auth-field">
              <label>Token</label>
              <input type="password" placeholder="Enter bearer token"
                class:unresolved-var={getUnresolvedVars($request.auth.token, $envVars).length > 0}
                value={$request.auth.token}
                on:input={e => updateAuth({ token: e.target.value })} />
            </div>
          {:else if $request.auth.type === 'apikey'}
            <div class="auth-field">
              <label>Key</label>
              <input type="text" placeholder="e.g. X-API-Key"
                class:unresolved-var={getUnresolvedVars($request.auth.apiKey, $envVars).length > 0}
                value={$request.auth.apiKey}
                on:input={e => updateAuth({ apiKey: e.target.value })} />
            </div>
            <div class="auth-field">
              <label>Value</label>
              <input type="password" placeholder="Enter API key value"
                class:unresolved-var={getUnresolvedVars($request.auth.apiValue, $envVars).length > 0}
                value={$request.auth.apiValue}
                on:input={e => updateAuth({ apiValue: e.target.value })} />
            </div>
            <div class="auth-field">
              <label>Add to</label>
              <select value={$request.auth.apiLocation} on:change={e => updateAuth({ apiLocation: e.target.value })}>
                <option value="header">Header</option>
                <option value="query">Query Params</option>
              </select>
            </div>
          {:else if $request.auth.type === 'basic'}
            <div class="auth-field">
              <label>Username</label>
              <input type="text" placeholder="Enter username"
                class:unresolved-var={getUnresolvedVars($request.auth.username, $envVars).length > 0}
                value={$request.auth.username}
                on:input={e => updateAuth({ username: e.target.value })} />
            </div>
            <div class="auth-field">
              <label>Password</label>
              <input type="password" placeholder="Enter password"
                class:unresolved-var={getUnresolvedVars($request.auth.password, $envVars).length > 0}
                value={$request.auth.password}
                on:input={e => updateAuth({ password: e.target.value })} />
            </div>
          {/if}
        </div>
      </div>

    <!-- Headers -->
    {:else if activeTab === 'headers'}
      <div class="tab-panel">
        <div class="key-value-editor">
          {#each $request.headers as header, i}
            <div class="kv-row">
              <input type="checkbox" checked={header.enabled} on:change={e => updateHeader(i, 'enabled', e.target.checked)} />
              <input type="text" placeholder="key"
                class:unresolved-var={getUnresolvedVars(header.key, $envVars).length > 0}
                value={header.key}
                on:input={e => updateHeader(i, 'key', e.target.value)} />
              <input type="text" placeholder="value"
                class:unresolved-var={getUnresolvedVars(header.value, $envVars).length > 0}
                value={header.value}
                on:input={e => updateHeader(i, 'value', e.target.value)} />
              <button class="btn-icon-small btn-delete-kv" on:click={() => removeHeader(i)}>×</button>
            </div>
          {/each}
          {#if $request.headers.length === 0}
            <div class="empty-state">No headers</div>
          {/if}
        </div>
        <button class="btn-secondary btn-add" on:click={addHeader}>+ Add Header</button>
      </div>

    <!-- Body -->
    {:else if activeTab === 'body'}
      <div class="tab-panel">
        <div class="body-options">
          <label>
            <input type="radio" name="bodyType" value="raw" bind:group={bodyType} /> Raw
          </label>
          <label>
            <input type="radio" name="bodyType" value="form-data" bind:group={bodyType} /> Form Data
          </label>
        </div>

        {#if bodyType === 'raw'}
          <textarea
            class="body-editor"
            placeholder={'{\n  "key": "value"\n}'}
            class:unresolved-var={getUnresolvedVars($request.body, $envVars).length > 0}
            value={$request.body}
            on:input={e => updateRequest({ body: e.target.value })}
          ></textarea>
        {:else}
          <div class="key-value-editor">
            {#each ($request.formData ?? []) as fd, i}
              <div class="kv-row">
                <input type="checkbox" checked={fd.enabled} on:change={e => updateFormData(i, 'enabled', e.target.checked)} />
                <input type="text" placeholder="key" value={fd.key} on:input={e => updateFormData(i, 'key', e.target.value)} />
                <input type="text" placeholder="value" value={fd.value} on:input={e => updateFormData(i, 'value', e.target.value)} />
                <button class="btn-icon-small" on:click={() => removeFormData(i)}>×</button>
              </div>
            {/each}
          </div>
          <button class="btn-secondary btn-add" on:click={addFormData}>+ Add Field</button>
        {/if}
      </div>

    <!-- Env -->
    {:else if activeTab === 'env'}
      <div class="tab-panel">
        <EnvVarsPanel />
      </div>
    {/if}
  </div>
</section>

<style>
  .request-editor {
    background: var(--bg-root);
    border-bottom: 1px solid var(--border-subtle);
    min-height: 200px;
    display: flex;
    flex-direction: column;
  }
  .editor-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: var(--spacing-lg) var(--spacing-lg) 0;
  }
  .tab-content { flex: 1; display: flex; flex-direction: column; overflow: hidden; }
  .tab-panel {
    display: flex; flex-direction: column; padding: var(--spacing-lg);
    flex: 1; overflow: auto; gap: var(--spacing-sm);
  }
  .key-value-editor { display: flex; flex-direction: column; gap: var(--spacing-sm); }
  .btn-add { margin-top: var(--spacing-sm); align-self: flex-start; }

  .body-options { display: flex; gap: var(--spacing-lg); margin-bottom: var(--spacing-sm); }
  .body-options label { display: flex; align-items: center; gap: var(--spacing-sm); cursor: pointer; font-size: var(--font-size-sm); }

  .body-editor {
    width: 100%; min-height: 120px; flex: 1;
    background: var(--bg-panel); border: 1px solid var(--border-color);
    border-radius: var(--radius-sm); padding: var(--spacing-md);
    font-family: var(--font-mono); font-size: var(--font-size-sm);
    resize: vertical; transition: background 120ms ease, border-color 120ms ease;
  }
  .body-editor:hover { background: var(--bg-panel-alt); }
  .body-editor:focus { border-color: var(--accent); background: var(--bg-panel-alt); }

  .auth-selector { margin-bottom: var(--spacing-lg); }
  .auth-selector label {
    display: block; margin-bottom: var(--spacing-sm);
    font-size: 12px; color: var(--text-muted); font-weight: 500;
    text-transform: uppercase; letter-spacing: 0.5px;
  }
  .auth-type-select {
    width: 100%; max-width: 300px;
    background: var(--bg-panel); border: 1px solid var(--border-color);
    border-radius: var(--radius-sm); padding: var(--spacing-sm) var(--spacing-md);
    cursor: pointer;
  }
  .auth-fields { display: flex; flex-direction: column; gap: var(--spacing-md); }
  .auth-field { display: flex; flex-direction: column; gap: var(--spacing-xs); }
  .auth-field label { font-size: 12px; color: var(--text-muted); font-weight: 500; }
  .auth-field input, .auth-field select {
    background: var(--bg-panel); border: 1px solid var(--border-color);
    border-radius: var(--radius-sm); padding: var(--spacing-sm) var(--spacing-md);
    transition: background 120ms ease, border-color 120ms ease;
  }
  .auth-field input:focus, .auth-field select:focus { border-color: var(--accent); background: var(--bg-panel-alt); }
</style>

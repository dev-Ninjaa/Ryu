<script>
  import { envVars, setEnvVar, deleteEnvVar, getInvoke } from '../store.js';

  function isValidKey(key) {
    return /^[A-Za-z_][A-Za-z0-9_]*$/.test(key);
  }

  // Local masked state per index
  let masked = {};
  function toggleMask(i) {
    masked[i] = !masked[i];
    masked = { ...masked };
  }

  async function handleAdd() {
    const key = `VAR_${Date.now()}`;
    await setEnvVar(key, '');
  }

  async function handleDelete(key) {
    await deleteEnvVar(key);
  }

  async function handleToggle(envVar) {
    await setEnvVar(envVar.key, envVar.value, !envVar.enabled);
  }

  // Debounced key/value update
  let timers = {};
  async function handleChange(envVar, field, val) {
    clearTimeout(timers[envVar.key + field]);
    timers[envVar.key + field] = setTimeout(async () => {
      if (field === 'key') {
        if (!val.trim() || !isValidKey(val.trim())) return;
        const old = envVar.key;
        if (val.trim() === old) return;
        // Check duplicate
        if ($envVars.some(v => v.key === val.trim() && v.key !== old)) return;
        await deleteEnvVar(old);
        await setEnvVar(val.trim(), envVar.value, envVar.enabled);
      } else {
        await setEnvVar(envVar.key, val, envVar.enabled);
      }
    }, 500);
  }

  async function handleExport() {
    const invoke = getInvoke();
    if (invoke) {
      try { await invoke('export_env_vars'); } catch (e) { alert('Export failed: ' + e.message); }
    } else {
      const data = { version: '1.0', exportedAt: new Date().toISOString(), env: $envVars };
      const blob = new Blob([JSON.stringify(data, null, 2)], { type: 'application/json' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a'); a.href = url; a.download = 'env-vars.json';
      document.body.appendChild(a); a.click(); a.remove(); URL.revokeObjectURL(url);
    }
  }

  let importInput;
  async function handleImport(e) {
    const file = e.target.files[0];
    if (!file) return;
    try {
      const text = await file.text();
      const payload = JSON.parse(text);
      const items = payload.env ?? (Array.isArray(payload) ? payload : []);
      for (const item of items) {
        await setEnvVar(item.key, item.value ?? '', item.enabled !== false);
      }
    } catch (err) {
      alert('Failed to import: ' + err.message);
    } finally {
      e.target.value = null;
    }
  }

  async function copyValue(val) {
    try { await navigator.clipboard.writeText(val); } catch {}
  }
</script>

<div class="env-info">
  <p>Use <code>{'{{VARIABLE}}'}</code> syntax in URL, headers, body, and auth fields.</p>
</div>

<div class="key-value-editor env-list">
  {#each $envVars as envVar, i}
    <div class="kv-row env-row">
      <input type="checkbox" checked={envVar.enabled} on:change={() => handleToggle(envVar)} />
      <input type="text" class="env-key" placeholder="VARIABLE_NAME"
        value={envVar.key}
        on:input={e => handleChange(envVar, 'key', e.target.value)} />
      <div class="env-value-wrap">
        <input
          type={masked[i] ? 'password' : 'text'}
          class="env-value"
          placeholder="value"
          value={envVar.value}
          on:input={e => handleChange(envVar, 'value', e.target.value)}
        />
        <button class="btn-icon-small btn-mask" title="Mask/Unmask" on:click={() => toggleMask(i)}>
          {masked[i] ? '👁' : '●●'}
        </button>
        <button class="btn-icon-small btn-copy" title="Copy value" on:click={() => copyValue(envVar.value)}>⧉</button>
      </div>
      <button class="btn-icon-small btn-delete-env" on:click={() => handleDelete(envVar.key)}>×</button>
    </div>
  {/each}
  {#if $envVars.length === 0}
    <div class="empty-state">No environment variables</div>
  {/if}
</div>

<div class="env-actions">
  <button class="btn-secondary btn-add" on:click={handleAdd}>+ Add Variable</button>
  <button class="btn-secondary" on:click={handleExport}>Export Variables</button>
  <button class="btn-secondary" on:click={() => importInput?.click()}>Import Variables</button>
  <input type="file" bind:this={importInput} accept=".json" style="display:none" on:change={handleImport} />
</div>

<style>
  .env-info {
    padding: var(--spacing-md) var(--spacing-lg);
    background: var(--bg-panel-alt); border-radius: var(--radius-md);
    margin-bottom: var(--spacing-md);
  }
  .env-info p { margin: 0; font-size: var(--font-size-sm); color: var(--text-secondary); }
  .env-info code {
    background: var(--bg-root); padding: 2px 6px; border-radius: var(--radius-sm);
    font-family: var(--font-mono); font-size: var(--font-size-xs); color: var(--accent);
  }
  .env-list { margin-bottom: var(--spacing-sm); }
  .env-row { grid-template-columns: 24px 180px 1fr 32px; }
  .env-key { font-family: var(--font-mono); text-transform: uppercase; letter-spacing: 0.5px; }
  .env-value-wrap { display: flex; align-items: center; gap: 6px; }
  .env-value { width: 100%; }
  .btn-mask, .btn-copy {
    background: none; border: none; cursor: pointer;
    font-size: 12px; padding: 6px; border-radius: var(--radius-sm);
    color: var(--text-muted);
  }
  .btn-mask:hover, .btn-copy:hover { background: var(--bg-panel-alt); }
  .env-actions { display: flex; gap: var(--spacing-sm); flex-wrap: wrap; margin-top: var(--spacing-sm); }
</style>

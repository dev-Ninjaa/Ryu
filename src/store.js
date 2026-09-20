import { writable, get } from 'svelte/store';

// ─── Tauri helpers ────────────────────────────────────────────────────────────
export function getInvoke() {
  return window.__TAURI__?.core?.invoke ?? window.__TAURI__?.invoke ?? null;
}
export function getDialog() {
  return window.__TAURI__?.dialog ?? window.__TAURI__?.core?.dialog ?? null;
}

// ─── Initial state shapes ─────────────────────────────────────────────────────
function defaultRequest() {
  return {
    method: 'GET',
    url: '',
    params: [],
    headers: [],
    body: '',
    auth: {
      type: 'none',
      token: '',
      apiKey: '',
      apiValue: '',
      apiLocation: 'header',
      username: '',
      password: '',
    },
  };
}

// ─── Stores ───────────────────────────────────────────────────────────────────
export const request   = writable(defaultRequest());
export const response  = writable(null);
export const history   = writable([]);
export const envVars   = writable([]);
export const collections = writable([]);

// ─── Request helpers ──────────────────────────────────────────────────────────
export function updateRequest(updates) {
  request.update(r => ({ ...r, ...updates }));
}

export function updateAuth(updates) {
  request.update(r => ({ ...r, auth: { ...r.auth, ...updates } }));
}

export function resetRequest() {
  request.set(defaultRequest());
}

// ─── History ──────────────────────────────────────────────────────────────────
export async function loadHistory() {
  const invoke = getInvoke();
  if (!invoke) return;
  try {
    const data = await invoke('get_history');
    history.set(data);
  } catch (e) {
    console.error('Failed to load history:', e);
    history.set([]);
  }
}

export async function clearHistory() {
  const invoke = getInvoke();
  if (!invoke) return;
  await invoke('clear_history');
  history.set([]);
}

// ─── Environment variables ────────────────────────────────────────────────────
export async function loadEnvVars() {
  const invoke = getInvoke();
  if (!invoke) return;
  try {
    const data = await invoke('get_env_vars');
    envVars.set(data);
  } catch (e) {
    console.error('Failed to load env vars:', e);
    envVars.set([]);
  }
}

export async function setEnvVar(key, value, enabled = true) {
  const invoke = getInvoke();
  if (!invoke) return;
  await invoke('set_env_var', { key, value, enabled });
  await loadEnvVars();
}

export async function deleteEnvVar(key) {
  const invoke = getInvoke();
  if (!invoke) return;
  await invoke('delete_env_var', { key });
  await loadEnvVars();
}

const defaultCollections = [
  {
    name: 'Ryu Tests',
    requests: [],
    folders: [
      {
        name: 'Basic API Tests',
        requests: [
          { name: 'Get Users', method: 'GET', url: 'http://localhost:3000/api/users', headers: [], params: [], body: '', auth: { type: 'none' } },
          { name: 'Create User', method: 'POST', url: 'http://localhost:3000/api/users', headers: [{ enabled: true, key: 'Content-Type', value: 'application/json' }], params: [], body: '{"name": "John Doe"}', auth: { type: 'none' } },
          { name: 'Echo Request', method: 'POST', url: 'http://localhost:3000/api/echo', headers: [{ enabled: true, key: 'Content-Type', value: 'application/json' }, { enabled: true, key: 'X-Custom-Header', value: 'test-value' }], params: [{ enabled: true, key: 'param1', value: 'value1' }], body: '{"message": "Hello World"}', auth: { type: 'none' } },
          { name: 'Error Test', method: 'GET', url: 'http://localhost:3000/api/error', headers: [], params: [], body: '', auth: { type: 'none' } },
        ],
      },
      {
        name: 'API Hardening Tests',
        requests: [
          { name: 'Query Parameters Test', method: 'GET', url: 'http://localhost:3000/api/params', headers: [], params: [{ enabled: true, key: 'q', value: 'test' }, { enabled: true, key: 'page', value: '1' }, { enabled: true, key: 'limit', value: '10' }], body: '', auth: { type: 'none' } },
          { name: 'Headers Test', method: 'GET', url: 'http://localhost:3000/api/headers', headers: [{ enabled: true, key: 'X-Test', value: 'custom-header-value' }, { enabled: true, key: 'X-API-Version', value: '1.0' }], params: [], body: '', auth: { type: 'none' } },
          { name: 'JSON Body Test', method: 'POST', url: 'http://localhost:3000/api/body', headers: [{ enabled: true, key: 'Content-Type', value: 'application/json' }], params: [], body: '{"key": "value", "number": 42, "array": [1, 2, 3]}', auth: { type: 'none' } },
          { name: 'Text Body Test', method: 'POST', url: 'http://localhost:3000/api/body', headers: [{ enabled: true, key: 'Content-Type', value: 'text/plain' }], params: [], body: 'This is plain text content', auth: { type: 'none' } },
          { name: 'URL Parameters Test', method: 'GET', url: 'http://localhost:3000/api/env/my-test-value', headers: [], params: [], body: '', auth: { type: 'none' } },
        ],
      },
      {
        name: 'Authentication Tests',
        requests: [
          { name: 'Bearer Token Auth', method: 'GET', url: 'http://localhost:3000/api/auth/bearer', headers: [{ enabled: true, key: 'Authorization', value: 'Bearer my-token' }], params: [], body: '', auth: { type: 'bearer', token: 'my-token' } },
          { name: 'Basic Auth', method: 'GET', url: 'http://localhost:3000/api/auth/basic', headers: [], params: [], body: '', auth: { type: 'basic', username: 'admin', password: 'password' } },
          { name: 'API Key in Header', method: 'GET', url: 'http://localhost:3000/api/auth/apikey', headers: [{ enabled: true, key: 'X-API-Key', value: 'key123' }], params: [], body: '', auth: { type: 'none' } },
          { name: 'API Key in Query', method: 'GET', url: 'http://localhost:3000/api/auth/apikey', headers: [], params: [{ enabled: true, key: 'api_key', value: 'key123' }], body: '', auth: { type: 'none' } },
        ],
      },
      {
        name: 'Advanced Tests',
        requests: [
          { name: 'Mixed Request Test', method: 'POST', url: 'http://localhost:3000/api/mixed', headers: [{ enabled: true, key: 'Content-Type', value: 'application/json' }, { enabled: true, key: 'Authorization', value: 'Bearer test-token' }, { enabled: true, key: 'X-Custom', value: 'header-value' }], params: [{ enabled: true, key: 'query_param', value: 'query_value' }], body: '{"data": "test", "id": 123}', auth: { type: 'bearer', token: 'test-token' } },
          { name: 'Invalid JSON Body', method: 'POST', url: 'http://localhost:3000/api/body', headers: [{ enabled: true, key: 'Content-Type', value: 'application/json' }], params: [], body: '{"invalid": json}', auth: { type: 'none' } },
        ],
      },
    ],
  },
];

// ─── Collections (localStorage) ───────────────────────────────────────────────
export function loadCollections() {
  try {
    const saved = localStorage.getItem('collections');
    if (saved) {
      const parsed = JSON.parse(saved);
      if (Array.isArray(parsed) && parsed.length > 0) {
        collections.set(parsed);
        return;
      }
    }
    // If no collections saved or empty, set default test collections
    collections.set(defaultCollections);
    localStorage.setItem('collections', JSON.stringify(defaultCollections));
  } catch (e) {
    console.error('Failed to load collections:', e);
    collections.set(defaultCollections);
  }
}

export function restoreDefaultCollections() {
  collections.set(defaultCollections);
  localStorage.setItem('collections', JSON.stringify(defaultCollections));
}

export function saveCollections() {
  collections.subscribe(cols => {
    localStorage.setItem('collections', JSON.stringify(cols));
  })(); // immediate one-shot read
}

// ─── Utility ──────────────────────────────────────────────────────────────────
export function escapeHtml(text) {
  if (text == null) return '';
  const d = document.createElement('div');
  d.textContent = String(text);
  return d.innerHTML;
}

export function getUnresolvedVars(text, vars) {
  if (!text || typeof text !== 'string') return [];
  const matches = text.match(/{{([^}]+)}}/g) ?? [];
  const keys = vars.map(v => v.key);
  return [...new Set(matches.map(m => m.slice(2, -2)).filter(k => !keys.includes(k)))];
}

export function validateJson(text) {
  if (!text?.trim()) return true;
  try { JSON.parse(text); return true; } catch { return false; }
}

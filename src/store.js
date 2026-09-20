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

import defaultCollectionsData from '../test/api/test-collections.json';

const defaultCollections = defaultCollectionsData.collections || [];

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

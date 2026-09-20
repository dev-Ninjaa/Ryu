# Changelog

All notable changes to Ryu are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.2.0] - 2026-09-19

### Changed

- Rebranded to **Ryu**
- Migrated frontend from vanilla HTML/CSS/JS to **Svelte + Vite**
- Replaced static file serving with Vite dev server (port 1420)
- State management now uses Svelte stores (`writable`) instead of global JS objects
- Consolidated all CSS into scoped Svelte component styles and a global `app.css`
- Updated `tauri.conf.json`: `productName = "Ryu"`, `identifier = "com.ryu.dev"`

### Removed

- All legacy `src/scripts/` JS files
- All legacy `src/styles/` CSS files

---

## [0.1.0] - 2026-09-19

### Initial Release

First stable release — a local-first API testing tool.

#### Added

**Core Features**
- HTTP request execution (GET, POST, PUT, DELETE, PATCH)
- Rust-based HTTP engine using reqwest
- Request builder with URL, params, headers, body
- Response viewer with status, timing, size, and pretty JSON
- Authentication support (Bearer Token, API Key, Basic Auth)

**Storage & Persistence**
- SQLite-based request history
- History sidebar with click-to-load
- Clear history functionality
- Persistent across app restarts

**Environment Variables**
- `{{VARIABLE}}` syntax support
- SQLite storage for env vars
- Variable resolution in URL, params, headers, body, auth
- Env tab for managing variables

**Collections**
- Folders and direct requests
- Import / Export as JSON
- Per-collection CRUD

**File Operations**
- Save requests as JSON files
- Load requests from JSON files
- Native file dialogs (save/open)

**Developer Experience**
- cURL export with copy to clipboard
- Dark mode UI with light theme toggle
- Keyboard shortcuts (Ctrl+Enter, Ctrl+S, Ctrl+O, Ctrl+B)
- Native desktop performance — no cloud, no accounts, no telemetry

**Technical**
- Tauri 2.x desktop framework
- Rust backend with tokio async runtime
- SQLite for local storage
- Svelte 4 frontend with Vite 5

---

## Version History

- **0.2.0** (2026-09-19) — Svelte migration, Ryu rebranding
- **0.1.0** (2026-09-19) — Initial release

---

## Links

- [GitHub Repository](https://github.com/dev-Ninjaa/ryu)
- [Documentation](docs/)
- [Issue Tracker](https://github.com/dev-Ninjaa/ryu/issues)

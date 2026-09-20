# Ryu — Architecture

## Overview

```
┌─────────────────────────────────────────────────────────────┐
│                     Desktop Window (Tauri)                  │
│                     1200×800, Centered                      │
│  ┌───────────────────────────────────────────────────────┐  │
│  │              Svelte Frontend (Vite, port 1420)        │  │
│  │                                                       │  │
│  │  ┌─────────────┬──────────────────────────────────┐  │  │
│  │  │             │  Request Bar                     │  │  │
│  │  │  Sidebar    ├──────────────────────────────────┤  │  │
│  │  │ (Collections│  Request Editor (Tabs)           │  │  │
│  │  │  + History) ├──────────────────────────────────┤  │  │
│  │  │             │  Response Viewer                 │  │  │
│  │  └─────────────┴──────────────────────────────────┘  │  │
│  └───────────────────────────────────────────────────────┘  │
│                              │                              │
│                         Tauri IPC                           │
│                              │                              │
│  ┌───────────────────────────────────────────────────────┐  │
│  │              Rust Backend                             │  │
│  │  ├── engine/   HTTP execution (reqwest) + auth        │  │
│  │  ├── store/    SQLite persistence (history)           │  │
│  │  ├── env/      Environment variables (SQLite)        │  │
│  │  └── workspace/ File I/O (save/load requests)        │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

## File Structure

```
ryu/
│
├── index.html                   # Vite entry point
├── vite.config.js               # Vite + Svelte plugin (port 1420)
├── svelte.config.js             # Svelte preprocessor
├── package.json                 # npm/bun scripts + dev deps
├── .cargo/config.toml           # Cargo build config (jobs = 4)
│
├── src/                         # Svelte frontend
│   ├── main.js                  # Mounts App.svelte
│   ├── app.css                  # Global CSS variables + shared styles
│   ├── store.js                 # Svelte writable stores (state)
│   ├── App.svelte               # Root layout, resize, theme, shortcuts
│   └── components/
│       ├── Sidebar.svelte       # Collections + History
│       ├── RequestBar.svelte    # Method / URL / Send / cURL / Save
│       ├── RequestEditor.svelte # Params / Auth / Headers / Body / Env tabs
│       ├── EnvVarsPanel.svelte  # Environment variable management
│       └── ResponseViewer.svelte# Response display + syntax highlight
│
├── src-tauri/                   # Rust backend
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── build.rs
│   ├── capabilities/main.json
│   └── src/
│       ├── main.rs              # Tauri commands + app setup
│       ├── models.rs            # Shared data models
│       ├── engine/              # HTTP execution, auth, cURL export
│       ├── store/               # SQLite history persistence
│       ├── env/                 # Env var storage + resolution
│       └── workspace/           # File I/O (save/load requests)
│
├── docs/                        # Documentation
├── test/api/                    # Local test REST API (Express)
└── .github/workflows/           # CI/CD (release builds)
```

## Data Flow

```
User clicks Send
      │
      ▼
RequestBar.svelte  ←  reads $request store
      │
      ▼
invoke('send_request', req)   ─── Tauri IPC ───▶  Rust
                                                     │
                                                     ▼
                                           resolve env vars
                                                     │
                                                     ▼
                                         reqwest HTTP call
                                                     │
                                                     ▼
                                         save to SQLite history
                                                     │
                                                     ▼
                                 ApiResponse { status, body, headers, time, size }
                                                     │
      ◀──────────────────────────────────────────────┘
      │
      ▼
response store updated  →  ResponseViewer.svelte re-renders
```

## How to Start

| Command | From | What happens |
|---|---|---|
| `npm run tauri:dev` | repo root | Vite starts on :1420, Rust compiles, window opens |
| `bun tauri dev` | repo root | Same via Bun |
| `cargo tauri dev` | repo root | Same via Cargo Tauri CLI |

All three trigger `beforeDevCommand = "npm run dev"` which starts Vite first.

## Technology Stack

| Layer | Technology |
|---|---|
| UI framework | Svelte 4 |
| Build tool | Vite 5 |
| Desktop shell | Tauri 2.x |
| HTTP client | reqwest 0.12 |
| Async runtime | tokio 1 |
| Database | SQLite via rusqlite (bundled) |
| Date/time | chrono 0.4 |

## Security

- All HTTP requests go through the Rust backend — JS never touches the network directly
- Tauri IPC is the only bridge between frontend and backend
- All data stored locally — no cloud, no telemetry
- CSP policies enforced by Tauri

---

**Last Updated**: September 19, 2026

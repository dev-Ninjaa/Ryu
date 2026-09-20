# How to Run Ryu

## Quick Start

```bash
npm install
npm run tauri:dev
```

The app window opens automatically once Rust finishes compiling.

---

## All Ways to Start

| Command | Requires |
|---|---|
| `npm run tauri:dev` | Node.js + npm |
| `bun tauri dev` | Bun |
| `cargo tauri dev` (from repo root) | Cargo Tauri CLI |

All three do the same thing:
1. Start the Vite dev server on port 1420
2. Compile the Rust backend
3. Open the Ryu desktop window

> `cargo tauri dev` works from the repo root because Tauri CLI walks up the directory tree to find `src-tauri/tauri.conf.json`.

---

## First Time Setup

### Install Rust
```bash
# Windows (PowerShell)
winget install Rustlang.Rustup

# macOS / Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Install Node.js (18+) or Bun
- Node: [nodejs.org](https://nodejs.org/)
- Bun: [bun.sh](https://bun.sh/)

### Linux system deps
```bash
sudo apt update && sudo apt install -y \
  libwebkit2gtk-4.1-dev libssl-dev libgtk-3-dev \
  libayatana-appindicator3-dev librsvg2-dev
```

---

## Build for Production

```bash
npm run tauri:build   # or: bun tauri build
```

Output locations:
- Windows: `src-tauri/target/release/bundle/msi/`
- macOS:   `src-tauri/target/release/bundle/dmg/`
- Linux:   `src-tauri/target/release/bundle/deb/` and `appimage/`

---

## Troubleshooting

| Issue | Fix |
|---|---|
| `cargo: command not found` | Install Rust from rustup.rs |
| Port 1420 in use | Change `server.port` in `vite.config.js` and `devUrl` in `tauri.conf.json` |
| Window doesn't open | Check terminal for errors; run `cargo clean` then retry |
| Slow first build | Normal — Rust compiles all deps once |
| UI looks broken | Run `npm install`, check F12 console |

---

## Keyboard Shortcuts

| Shortcut | Action |
|---|---|
| `Ctrl+Enter` | Send request |
| `Ctrl+S` | Save request to file |
| `Ctrl+O` | Load request from file |
| `Ctrl+B` | Toggle sidebar |
| `Ctrl+Shift+S` | Save to collection |
| `F12` | DevTools (dev mode only) |

---

**Last Updated**: September 19, 2026

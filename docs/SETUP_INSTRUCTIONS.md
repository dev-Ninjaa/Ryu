# Setup Instructions

**Get Ryu running in under 2 minutes.**

---

## Quick Setup

### 1. Install Rust

```bash
# Windows (PowerShell)
winget install Rustlang.Rustup

# macOS / Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Verify: `rustc --version` — should be 1.77.2 or later.

### 2. Install Node.js (18+) or Bun

- Node: [nodejs.org](https://nodejs.org/)
- Bun: `curl -fsSL https://bun.sh/install | bash`

### 3. Clone and run

```bash
git clone https://github.com/dev-Ninjaa/ryu.git
cd ryu
npm install        # or: bun install
npm run tauri:dev  # or: bun tauri dev  |  cargo tauri dev
```

First run: 2–5 min. Subsequent runs: ~10 sec.

---

## Platform Notes

### Windows
- Install **Visual Studio C++ Build Tools** → "Desktop development with C++"
- WebView2 is pre-installed on Windows 10/11

### macOS
```bash
xcode-select --install
```

### Linux (Ubuntu/Debian)
```bash
sudo apt update && sudo apt install -y \
  libwebkit2gtk-4.1-dev build-essential curl wget file \
  libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
```

---

## Troubleshooting

| Problem | Fix |
|---|---|
| `rustc not found` | Restart terminal or run `source $HOME/.cargo/env` |
| `linker not found` (Windows) | Install VS C++ Build Tools |
| `webkit2gtk not found` (Linux) | `sudo apt install libwebkit2gtk-4.1-dev` |
| Permission denied (macOS/Linux) | `chmod +x src-tauri/target/debug/ryu` |
| Build too slow | Normal for first build — deps are cached after |

---

## Quick Reference

| Command | Purpose |
|---|---|
| `npm run tauri:dev` | Development mode |
| `npm run tauri:build` | Production build |
| `cd src-tauri && cargo check` | Check Rust compilation |
| `cd src-tauri && cargo clean` | Clear build cache |

| Shortcut | Action |
|---|---|
| `Ctrl+Enter` | Send request |
| `Ctrl+S` | Save request |
| `Ctrl+O` | Load request |
| `Ctrl+B` | Toggle sidebar |
| `F12` | DevTools (dev mode) |

---

## Uninstall

```bash
# macOS
rm -rf /Applications/Ryu.app

# Linux
sudo apt remove ryu   # if installed via .deb

# Remove dev files
cd ryu && rm -rf src-tauri/target/
```

---

**Last Updated**: September 19, 2026

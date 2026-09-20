# Building Ryu

Complete guide to building from source.

---

## Prerequisites

### All Platforms

**Rust** (1.77.2+)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustc --version && cargo --version
```

**Node.js** (18+) or **Bun** (1.0+)
```bash
node --version && npm --version
# or
bun --version
```

### Platform Dependencies

#### Windows
- Visual Studio C++ Build Tools — "Desktop development with C++" workload
- WebView2 (pre-installed on Windows 10/11)

#### macOS
```bash
xcode-select --install
```

#### Linux (Ubuntu/Debian)
```bash
sudo apt update && sudo apt install -y \
  libwebkit2gtk-4.1-dev build-essential curl wget file \
  libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
```

#### Linux (Fedora)
```bash
sudo dnf install webkit2gtk4.1-devel openssl-devel curl wget file \
  gtk3-devel libappindicator-gtk3-devel librsvg2-devel
```

---

## Development Build

```bash
git clone https://github.com/dev-Ninjaa/ryu.git
cd ryu
npm install           # or: bun install
npm run tauri:dev     # or: bun tauri dev  |  cargo tauri dev
```

First run compiles all Rust dependencies (~2–5 min). Subsequent runs: ~10 sec.

---

## Production Build

```bash
npm run tauri:build   # or: bun tauri build
```

### Output Locations

**Windows**
```
src-tauri/target/release/
├── ryu.exe
└── bundle/
    ├── msi/ryu_0.1.0_x64_en-US.msi
    └── nsis/ryu_0.1.0_x64-setup.exe
```

**macOS**
```
src-tauri/target/release/bundle/
├── macos/Ryu.app
└── dmg/ryu_0.1.0_x64.dmg
```

**Linux**
```
src-tauri/target/release/bundle/
├── appimage/ryu_0.1.0_amd64.AppImage
├── deb/ryu_0.1.0_amd64.deb
└── rpm/ryu-0.1.0-1.x86_64.rpm
```

---

## Troubleshooting

| Problem | Fix |
|---|---|
| "linker not found" (Windows) | Install VS C++ Build Tools |
| "webkit2gtk not found" (Linux) | `sudo apt install libwebkit2gtk-4.1-dev` |
| "openssl not found" | `sudo apt install libssl-dev` / `brew install openssl` |
| Out of memory | `npm run tauri:build -- -j 2` |

---

## Clean Build

```bash
cd src-tauri && cargo clean
```

Frees ~2 GB but requires a full rebuild.

---

**Last Updated**: September 19, 2026

# Ryu — Quick Start

## Prerequisites

1. **Rust** 1.77.2+: [rustup.rs](https://rustup.rs/)
2. **Node.js** 18+ **or Bun** 1.0+

## Run

```bash
# Clone
git clone https://github.com/dev-Ninjaa/ryu.git
cd ryu

# Install frontend deps
npm install    # or: bun install

# Start (choose any one)
npm run tauri:dev    # npm
bun tauri dev        # bun
cargo tauri dev      # cargo (from repo root)
```

First run: ~2–5 min (Rust compiles dependencies). Subsequent runs: ~10 sec.

## What to Expect

- A desktop window opens titled **"Ryu"** (1200×800)
- Dark theme by default — toggle with the sun/moon icon in the sidebar header
- All features work immediately

## Build for Production

```bash
npm run tauri:build   # or: bun tauri build
```

Output: `src-tauri/target/release/bundle/`

---

**Last Updated**: September 19, 2026

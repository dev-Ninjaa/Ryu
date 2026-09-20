import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';

export default defineConfig({
  plugins: [svelte()],
  // Tauri dev server listens on port 1420 by default
  server: {
    port: 1420,
    strictPort: true,
  },
  // Ensure env variables from the host are not exposed
  envPrefix: ['VITE_', 'TAURI_'],
  build: {
    // Output to dist/ so Tauri can pick it up
    outDir: 'dist',
    // Produce small bundles for the desktop app
    target: ['es2021', 'chrome100', 'safari13'],
    minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
    sourcemap: !!process.env.TAURI_DEBUG,
  },
});

/* -------------------------------------------------------------------

                   🗲 Storm Software - Telepathic

 This code was released as part of the Telepathic project. Telepathic
 is maintained by Storm Software under the Apache-2.0 license, and is
 free for commercial and private use. For more information, please visit
 our licensing page at https://stormsoftware.com/licenses/projects/telepathic.

 Website:                  https://stormsoftware.com
 Repository:               https://github.com/storm-software/telepathic
 Documentation:            https://docs.telepathic.sh
 Contact:                  https://stormsoftware.com/contact

 SPDX-License-Identifier:  Apache-2.0

 ------------------------------------------------------------------- */

import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";

const host = process.env.TAURI_DEV_HOST;
const platform = process.env.TAURI_ENV_PLATFORM;

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],

  // Prevent Vite from obscuring Rust errors
  clearScreen: false,

  // Env variables starting with envPrefix are exposed through import.meta.env
  envPrefix: ["VITE_", "TAURI_ENV_*"],

  server: {
    // Must match devUrl port in src-tauri/tauri.conf.json
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421
        }
      : undefined,
    watch: {
      ignored: ["**/src-tauri/**"]
    }
  },

  build: {
    // Tauri webview targets; es2021 used on Unix because Vite 8 cannot transpile to safari13
    target: platform
      ? platform === "windows"
        ? "chrome105"
        : "es2021"
      : "esnext",
    minify: process.env.TAURI_ENV_DEBUG ? false : "esbuild",
    sourcemap: !!process.env.TAURI_ENV_DEBUG
  }
});

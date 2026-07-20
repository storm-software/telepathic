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

import napi from "@powerlines/plugin-napi-rs";
import tsdown from "@powerlines/plugin-tsdown";
import type { UserConfig } from "powerlines";
import { defineConfig } from "powerlines/config";

const config: UserConfig = defineConfig({
  input: ["src/index.ts", "src/helpers/*.ts", "src/lib/*.ts"],
  platform: "node",
  output: {
    format: ["cjs", "esm"]
  },
  resolve: {
    external: ["@telepathic/sdk-*", "telepathic-nodejs-sdk.*"],
    skipNodeModulesBundle: true
  },
  plugins: [
    tsdown(),
    napi({
      binaryName: "telepathic-nodejs-sdk",
      packageName: "@telepathic/sdk",
      targets: [
        "x86_64-apple-darwin",
        "aarch64-apple-darwin",
        "x86_64-unknown-linux-gnu",
        "x86_64-pc-windows-msvc",
        "x86_64-unknown-linux-musl",
        "aarch64-unknown-linux-gnu",
        "i686-pc-windows-msvc",
        "armv7-unknown-linux-gnueabihf",
        "aarch64-linux-android",
        "x86_64-unknown-freebsd",
        "aarch64-unknown-linux-musl",
        "aarch64-pc-windows-msvc",
        "armv7-linux-androideabi",
        "wasm32-wasip1-threads"
      ],
      wasm: {
        initialMemory: 16384,
        browser: {
          fs: true,
          asyncInit: true
        }
      },
      jsBinding: "../src/bindings.cjs",
      dts: "../src/bindings.d.cts",
      dtsHeader:
        "export type MaybePromise<T> = T | Promise<T>\nexport type Nullable<T> = T | null | undefined\ntype VoidNullable<T = void> = T | null | undefined | void\nexport type BindingStringOrRegex = string | RegExp\ntype BindingResult<T> = { errors: BindingError[], isBindingErrors: boolean } | T\n\n",
      npmDir: "npm",
      outputDir: "artifacts",
      manifestPath: "./Cargo.toml"
    })
  ]
});

export default config;

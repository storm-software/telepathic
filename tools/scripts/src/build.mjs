#!/usr/bin/env zx
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

import { $, argv, chalk, echo, path } from "zx";

/**
 * Force one shared Cargo target dir for the whole monorepo build.
 *
 * The Storm rust Nx plugin defaults each crate to
 * `dist/{projectRoot}/target`. That makes `native-tree-sitter` (and every
 * other dep) recompile once per crate — currently ~10–16GB × N target dirs.
 *
 * `.cargo/config.toml` already sets `target-dir = "dist/target"`. Pin the
 * env vars too so Nx/napi/tauri child processes cannot fork off per-crate dirs.
 */
function pinSharedCargoTargetDir() {
  const targetDir = path.resolve("dist/target");
  process.env.CARGO_TARGET_DIR = targetDir;
  process.env.CARGO_BUILD_TARGET_DIR = targetDir;
  return targetDir;
}

async function runLogged(command, timeout, failureMessage) {
  const proc = command.timeout(timeout);
  proc.stdout.on("data", data => {
    echo`${data}`;
  });
  const result = await proc;
  if (result.exitCode !== 0) {
    throw new Error(`${failureMessage}: \n\n${result.message}\n`);
  }
  return result;
}

try {
  let configuration = argv.configuration;
  if (!configuration) {
    if (argv.prod) {
      configuration = "production";
    } else if (argv.dev) {
      configuration = "development";
    } else {
      configuration = "production";
    }
  }

  const cargoTargetDir = pinSharedCargoTargetDir();
  const cargoRelease = configuration === "production";

  echo`${chalk.whiteBright(
    ` 🏗️  Building the monorepo in ${configuration} mode...`
  )}`;
  echo`${chalk.dim(`    Cargo target dir: ${cargoTargetDir}`)}`;

  await runLogged(
    $`pnpm bootstrap`,
    `${1 * 60}s`,
    "An error occurred while bootstrapping the monorepo"
  );

  // One workspace Cargo invocation builds every member (including
  // telepathic-tree-sitter) into the shared target dir. Later Nx/napi/tauri
  // builds reuse those artifacts instead of recompiling grammars N times.
  const cargoArgs = ["build", "--workspace"];
  if (cargoRelease) {
    cargoArgs.push("--release");
  }

  echo`${chalk.whiteBright(
    ` 🦀  Cargo workspace build (${cargoRelease ? "release" : "dev"})...`
  )}`;
  await runLogged(
    $`cargo ${cargoArgs}`,
    `${80 * 60}s`,
    "An error occurred while building the Cargo workspace"
  );

  // Exclude native-* crates from run-many: Cargo already built them. If the
  // Storm rust plugin re-infers per-crate `build` targets, running those would
  // only re-invoke cargo (and historically forked target dirs).
  await runLogged(
    $`pnpm nx run-many --target=build --exclude=monorepo,native-* --configuration=${configuration} --outputStyle=dynamic-legacy --parallel=5`,
    `${80 * 60}s`,
    `An error occurred while building the monorepo in ${configuration} mode`
  );

  echo`${chalk.green(
    ` ✔ Successfully built the monorepo in ${configuration} mode!`
  )}\n`;
} catch (error) {
  echo`${chalk.red(error?.message ? error.message : "A failure occurred while building the monorepo")}`;

  process.exit(1);
}

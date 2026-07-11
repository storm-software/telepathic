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

import { $, chalk, echo } from "zx";

try {
  echo`${chalk.whiteBright(
    ` 🏭  Generating code for the Tree-Sitter grammars...\n`
  )}`;

  const proc = $`cargo build -p native-tree-sitter`.timeout(`${2 * 60}s`);
  proc.stdout.on("data", data => {
    echo`${data}`;
  });
  const result = await proc;
  if (result.exitCode !== 0) {
    throw new Error(
      `An error occurred while building the native-tree-sitter crate: \n\n${result.message}\n`
    );
  }
  echo`${chalk.green(
    ` ✔ Successfully generated code for the Tree-Sitter grammars!\n`
  )}`;
} catch (error) {
  echo`${chalk.red(error?.message ? error.message : "A failure occurred while generating code for the Tree-Sitter grammars")}`;

  process.exit(1);
}

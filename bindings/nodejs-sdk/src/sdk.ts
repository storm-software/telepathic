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

import { NativeBindingSDK } from "./native";
import type { Telepathic } from "./types/sdk";

export async function createTelepathic(userConfig = {}): Promise<Telepathic> {
  const bindings = new NativeBindingSDK(userConfig);
  const session = await bindings.getSession();

  return {
    session,
    getSchema: async () => bindings.getSchema(),
    listRepositories: async () => bindings.listRepositories(),
    listProjects: async input => bindings.listProjects(input),
    writeGraph: async input => bindings.writeGraph(input),
    readGraph: async input => bindings.readGraph(input),
    queryGraph: async input => bindings.queryGraph(input),
    searchGraph: async input => bindings.searchGraph(input),
    traceGraph: async input => bindings.traceGraph(input),
    exportOkf: async input => bindings.exportOkf(input)
  };
}

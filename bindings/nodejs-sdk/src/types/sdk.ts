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

import type {
  BindingDefinition,
  BindingExportOkfInput,
  BindingExportOkfOutput,
  BindingGetSchemaOutput,
  BindingIndexRepositoryOutput,
  BindingListProjectsInput,
  BindingListProjectsOutput,
  BindingListRepositoriesOutput,
  BindingQueryGraphInput,
  BindingQueryGraphOutput,
  BindingReadGraphInput,
  BindingReadGraphOutput,
  BindingSearchGraphInput,
  BindingSearchGraphOutput,
  BindingSession,
  BindingTraceGraphInput,
  BindingTraceGraphOutput,
  BindingWriteGraphInput,
  BindingWriteGraphOutput
} from "../bindings.cjs";

export type {
  BindingDefinition,
  BindingExportOkfInput,
  BindingExportOkfOutput,
  BindingGetSchemaOutput,
  BindingIndexRepositoryOutput,
  BindingListProjectsInput,
  BindingListProjectsOutput,
  BindingListRepositoriesOutput,
  BindingQueryGraphInput,
  BindingQueryGraphOutput,
  BindingReadGraphInput,
  BindingReadGraphOutput,
  BindingSearchGraphInput,
  BindingSearchGraphOutput,
  BindingTraceGraphInput,
  BindingTraceGraphOutput,
  BindingWriteGraphInput,
  BindingWriteGraphOutput
};

export interface Telepathic {
  session: BindingSession;
  getSchema: () => Promise<BindingGetSchemaOutput>;
  listRepositories: () => Promise<BindingListRepositoriesOutput>;
  listProjects: (
    input: BindingListProjectsInput
  ) => Promise<BindingListProjectsOutput>;
  writeGraph: (
    input: BindingWriteGraphInput
  ) => Promise<BindingWriteGraphOutput>;
  readGraph: (input: BindingReadGraphInput) => Promise<BindingReadGraphOutput>;
  queryGraph: (
    input: BindingQueryGraphInput
  ) => Promise<BindingQueryGraphOutput>;
  searchGraph: (
    input: BindingSearchGraphInput
  ) => Promise<BindingSearchGraphOutput>;
  traceGraph: (
    input: BindingTraceGraphInput
  ) => Promise<BindingTraceGraphOutput>;
  exportOkf: (input: BindingExportOkfInput) => Promise<BindingExportOkfOutput>;
}

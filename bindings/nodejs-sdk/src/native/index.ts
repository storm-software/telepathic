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
  BindingResult,
  BindingSearchGraphInput,
  BindingSearchGraphOutput,
  BindingSession,
  BindingTraceGraphInput,
  BindingTraceGraphOutput,
  BindingWriteGraphInput,
  BindingWriteGraphOutput
} from "../bindings.cjs";
import {
  BindingSDK,
  shutdownAsyncRuntime,
  startAsyncRuntime
} from "../bindings.cjs";
import { isBindingFailureResult } from "./utils";

// @ts-expect-error TS2540: the polyfill of `asyncDispose`.
Symbol.asyncDispose ??= Symbol("Symbol.asyncDispose");

function unwrapBindingResult<T>(
  result: BindingResult<T>,
  operation: string
): T {
  if (isBindingFailureResult(result)) {
    throw new Error(
      `Power Plant - ${operation} failed with errors: ${result.errors
        .map(error => error.field0.message)
        .join("\n")}`
    );
  }

  return result;
}

export class NativeBindingSDK {
  #isClosed = false;

  #binding: BindingSDK;

  #stopWorkers?: () => Promise<void>;

  protected static asyncRuntimeShutdown = false;

  public constructor(config: any) {
    this.#binding = new BindingSDK({
      logLevel: config.settings?.logLevel,
      cwd: config.cwd
    });
  }

  public get isClosed(): boolean {
    return this.#isClosed;
  }

  private async prepareBinding(): Promise<void> {
    await this.#stopWorkers?.();
    if (NativeBindingSDK.asyncRuntimeShutdown) {
      startAsyncRuntime();
    }
  }

  public async getSession(): Promise<BindingSession> {
    await this.prepareBinding();

    const result = await this.#binding.getSession();
    const output = unwrapBindingResult(result, "Get Session");

    return output.session;
  }

  public async getSchema(): Promise<BindingGetSchemaOutput> {
    await this.prepareBinding();

    const result = await this.#binding.getSchema();

    return unwrapBindingResult(result, "Get Schema");
  }

  public async listRepositories(): Promise<BindingListRepositoriesOutput> {
    await this.prepareBinding();

    const result = await this.#binding.listRepositories();

    return unwrapBindingResult(result, "List Repositories");
  }

  public async indexRepository(): Promise<BindingIndexRepositoryOutput> {
    await this.prepareBinding();

    const result = await this.#binding.indexRepository({
      rootPath: undefined,
      force: undefined
    });

    return unwrapBindingResult(result, "Index Repository");
  }

  public async listProjects(
    input: BindingListProjectsInput
  ): Promise<BindingListProjectsOutput> {
    await this.prepareBinding();

    const result = await this.#binding.listProjects(input);

    return unwrapBindingResult(result, "List Projects");
  }

  public async writeGraph(
    input: BindingWriteGraphInput
  ): Promise<BindingWriteGraphOutput> {
    await this.prepareBinding();

    const result = await this.#binding.writeGraph(input);

    return unwrapBindingResult(result, "Write Graph");
  }

  public async readGraph(
    input: BindingReadGraphInput
  ): Promise<BindingReadGraphOutput> {
    await this.prepareBinding();

    const result = await this.#binding.readGraph(input);

    return unwrapBindingResult(result, "Read Graph");
  }

  public async queryGraph(
    input: BindingQueryGraphInput
  ): Promise<BindingQueryGraphOutput> {
    await this.prepareBinding();

    const result = await this.#binding.queryGraph(input);

    return unwrapBindingResult(result, "Query Graph");
  }

  public async searchGraph(
    input: BindingSearchGraphInput
  ): Promise<BindingSearchGraphOutput> {
    await this.prepareBinding();

    const result = await this.#binding.searchGraph(input);

    return unwrapBindingResult(result, "Search Graph");
  }

  public async traceGraph(
    input: BindingTraceGraphInput
  ): Promise<BindingTraceGraphOutput> {
    await this.prepareBinding();

    const result = await this.#binding.traceGraph(input);

    return unwrapBindingResult(result, "Trace Graph");
  }

  public async exportOkf(
    input: BindingExportOkfInput
  ): Promise<BindingExportOkfOutput> {
    await this.prepareBinding();

    const result = await this.#binding.exportOkf(input);

    return unwrapBindingResult(result, "Export OKF");
  }

  public async close(): Promise<void> {
    await this.#stopWorkers?.();
    await this.#binding.close();

    shutdownAsyncRuntime();

    NativeBindingSDK.asyncRuntimeShutdown = true;
    this.#stopWorkers = undefined;
    this.#isClosed = true;
  }

  public async [Symbol.asyncDispose](): Promise<void> {
    await this.close();
  }
}

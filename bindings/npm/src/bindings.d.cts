export type MaybePromise<T> = T | Promise<T>;
export type Nullable<T> = T | null | undefined;
type VoidNullable<T = void> = T | null | undefined | void;
export type BindingStringOrRegex = string | RegExp;
type BindingResult<T> =
  { errors: BindingError[]; isBindingErrors: boolean } | T;

export declare class BindingEngine {
  constructor(options: BindingOptions);
  getSettings(): Promise<BindingResult<BindingGetSettingsOutput>>;
  getSession(): Promise<BindingResult<BindingGetSessionOutput>>;
  getSchema(): Promise<BindingResult<BindingGetSchemaOutput>>;
  listRepositories(): Promise<BindingResult<BindingListRepositoriesOutput>>;
  indexRepository(): Promise<BindingResult<BindingIndexRepositoryOutput>>;
  listProjects(
    input: BindingListProjectsInput
  ): Promise<BindingResult<BindingListProjectsOutput>>;
  writeGraph(
    input: BindingWriteGraphInput
  ): Promise<BindingResult<BindingWriteGraphOutput>>;
  readGraph(
    input: BindingReadGraphInput
  ): Promise<BindingResult<BindingReadGraphOutput>>;
  queryGraph(
    input: BindingQueryGraphInput
  ): Promise<BindingResult<BindingQueryGraphOutput>>;
  searchGraph(
    input: BindingSearchGraphInput
  ): Promise<BindingResult<BindingSearchGraphOutput>>;
  traceGraph(
    input: BindingTraceGraphInput
  ): Promise<BindingResult<BindingTraceGraphOutput>>;
  exportOkf(
    input: BindingExportOkfInput
  ): Promise<BindingResult<BindingExportOkfOutput>>;
  close(): Promise<undefined>;
  get isClosed(): boolean;
}

export declare class TraceSubscriberGuard {
  close(): void;
}

export interface BindingDefinition {
  name: string;
  qualifiedName: string;
  label: string;
  filePath?: string;
  startLine: number;
  endLine: number;
  signature?: string;
  returnType?: string;
  parentClass?: string;
  decorators: Array<string>;
  baseClasses: Array<string>;
  paramNames: Array<string>;
  paramTypes: Array<string>;
  returnTypes: Array<string>;
  complexity: number;
  lines: number;
  isExported: boolean;
  isTest: boolean;
  isEntryPoint: boolean;
}

export interface BindingDevice {
  name: string;
  displayName: string;
  platform: string;
  distro: string;
  desktopEnv: string;
  cpuArch: string;
}

export interface BindingEnvPaths {
  /** Path to the cache directory. */
  cache: string;
  /** Path to the configuration directory. */
  config: string;
  /** Path to the data directory. */
  data: string;
  /** Path to the log directory. */
  logs: string;
  /** Path to the temporary directory. */
  temp: string;
  /** Path to the downloads directory. */
  downloads: string;
  /** Path to the executable directory. */
  executable: string;
}

export type BindingError =
  | { type: "JsError"; field0: Error }
  | { type: "NativeError"; field0: NativeError };

export interface BindingErrors {
  errors: Array<BindingError>;
  isBindingErrors: boolean;
}

export interface BindingExecutionSearchHit {
  /** The id of the matching execution. */
  executionId: string;
  /** Relevance score when provided by the search backend. */
  score?: number;
  /** Short excerpt from the matched metadata, when available. */
  snippet?: string;
}

export interface BindingExportOkfInput {
  /** The path to the output location the OKF files will be written to. */
  outputPath: string;
}

export interface BindingExportOkfOutput {
  /** Whether the export operation was successful. */
  success: boolean;
  /** Any errors encountered during the export operation. */
  errors: Array<string>;
}

export interface BindingGetSchemaOutput {
  /** The schema. */
  schema: string;
}

export interface BindingGetSessionOutput {
  /** The current session. */
  session: BindingSession;
}

export interface BindingGetSettingsOutput {
  /** The loaded settings. */
  settings: BindingSettings;
}

export interface BindingIndexRepositoryOutput {
  /** Whether the index repository operation was successful. */
  success: boolean;
  /** Any errors encountered during the index repository operation. */
  errors: Array<string>;
}

export interface BindingListProjectsInput {
  /** The id of the repository to list projects for. */
  repositoryId?: string;
  /** All returned projects must depend on the given project. */
  dependsOn?: string;
}

export interface BindingListProjectsOutput {
  /** The projects. */
  projects: Array<string>;
}

export interface BindingListRepositoriesOutput {
  /** The repositories. */
  repositories: Array<string>;
}

/** Represents a log entry in the Telepathic binding. */
export interface BindingLog {
  /** The log message. */
  message: string;
  /** The log code. */
  code?: string;
  /** Additional details about the log. */
  details?: string;
  /** The log level. */
  level: BindingLogLevel;
  /** The plugin that generated the log. */
  plugin?: string;
}

export declare const enum BindingLogLevel {
  Silent = 0,
  Error = 1,
  Warn = 2,
  Info = 3,
  Debug = 4
}

export declare const enum BindingMode {
  /** Development mode. */
  Development = 0,
  /** Production mode. */
  Production = 1,
  /** Test mode. */
  Test = 2
}

export interface BindingOptions {
  /** The mode. */
  mode?: "development" | "production" | "test";
  /** The username of the user currently using the application */
  username?: string;
  /** The log level. */
  logLevel?: "debug" | "info" | "warn" | "error" | "silent";
  /** Callback for log messages. */
  customLogger?: (
    logLevel: "debug" | "info" | "warn" | "error",
    log: BindingLog
  ) => Promise<void>;
  /** The current working directory. */
  cwd?: string;
  /** The repository root. */
  repositoryRoot?: string;
}

export interface BindingQueryGraphInput {
  /** The Cypher query to execute on the source code graph. */
  query: string;
  /** The params to bind to the query. */
  params?: Record<string, any>;
}

export interface BindingQueryGraphOutput {
  /** The query results. */
  results: Array<string>;
}

export interface BindingReadGraphInput {
  /** The name of the node to read. */
  name: string;
}

export interface BindingReadGraphOutput {
  /** The node. */
  node: string;
}

export interface BindingSearchGraphInput {
  /** Free-text query matched against indexed source code graph nodes. */
  query?: string;
  /** Filter by the user who last modified the node. */
  lastUserId?: string;
  /** Filter by the name of the node. */
  name: string;
  /** Filter by the fully qualified name of the node. */
  qualifiedName: string;
  /** Filter by the label of the node. */
  label: string;
  /** Filter by the file path of the node. */
  filePath?: string;
  /** Filter by labels; a node matches when any label is present. */
  labels?: Array<string>;
  /** Optional embedding vector for semantic similarity search. */
  embedding?: Array<number>;
  /** Maximum number of results to return. */
  limit?: number;
}

export interface BindingSearchGraphOutput {
  /** The search results. */
  results: Array<BindingExecutionSearchHit>;
}

export interface BindingSession {
  sessionId: string;
  startedAt: number;
  user: BindingUser;
  device: BindingDevice;
}

export interface BindingSettings {
  /** The app mode to use. */
  mode: "development" | "production" | "test";
  /** The default username to use. */
  defaultUser: string;
  /** The paths to use.`` */
  paths: {
    cache: string;
    config: string;
    data: string;
    logs: string;
    temp: string;
    downloads: string;
    executable: string;
  };
  /** The log level to use. */
  logLevel: "debug" | "info" | "warn" | "error" | "silent";
  /** Whether to skip storage. */
  skipStorage: boolean;
}

export interface BindingTraceGraphInput {
  /** The name of the call site to trace. */
  callSiteName: string;
  /** The fully qualified name of the call site to trace. */
  qualifiedName: string;
  /** The strategy of the call site to trace. */
  strategy: string;
  /** The confidence of the call site to trace. */
  confidence: number;
}

export interface BindingTraceGraphOutput {
  /** The trace results. */
  results: Array<string>;
}

export interface BindingUser {
  name: string;
  displayName: string;
  languagePreferences: Array<string>;
}

export interface BindingWriteGraphInput {
  /** The node to write. */
  node: BindingDefinition;
  /** The properties to write. */
  properties?: Record<string, any>;
}

export interface BindingWriteGraphOutput {
  /** Whether the write graph operation was successful. */
  success: boolean;
  /** Any errors encountered during the write graph operation. */
  errors: Array<string>;
}

export declare function createTokioRuntime(
  blockingThreads?: number | undefined | null
): void;

export declare function initTraceSubscriber(): TraceSubscriberGuard | null;

/** Error emitted from native side, it only contains kind and message, no stack trace. */
export interface NativeError {
  kind: string;
  message: string;
}

/**
 * Shutdown the tokio runtime manually.
 *
 * This is required for the wasm target with `tokio_unstable` cfg.
 * In the wasm runtime, the `park` threads will hang there until the tokio::Runtime is shutdown.
 */
export declare function shutdownAsyncRuntime(): void;

/**
 * Start the async runtime manually.
 *
 * This is required when the async runtime is shutdown manually.
 * Usually it's used in test.
 */
export declare function startAsyncRuntime(): void;

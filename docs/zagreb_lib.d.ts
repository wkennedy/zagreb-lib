/* tslint:disable */
/* eslint-disable */
/**
 * Make a JS-compatible string list of low connectivity validators
 */
export function get_low_connectivity_validators(graph: WasmGraph): Uint32Array;
/**
 * Graph analysis result to be returned to JavaScript
 */
export class GraphAnalysisResult {
  private constructor();
  free(): void;
  readonly vertex_count: number;
  readonly edge_count: number;
  readonly zagreb_index: number;
  readonly min_degree: number;
  readonly max_degree: number;
  readonly is_likely_hamiltonian: boolean;
  readonly is_likely_traceable: boolean;
  readonly independence_number: number;
  readonly zagreb_upper_bound: number;
}
/**
 * A simple error type for WASM interfaces
 */
export class WasmError {
  free(): void;
  constructor(message: string);
  readonly message: string;
}
/**
 * WASM bindings for creating and manipulating graphs
 */
export class WasmGraph {
  free(): void;
  /**
   * Create a new empty graph with n vertices
   */
  constructor(n: number);
  /**
   * Add an edge between vertices u and v
   */
  add_edge(u: number, v: number): void;
  /**
   * Get the degree of a vertex
   */
  degree(v: number): number;
  /**
   * Calculate the first Zagreb index of the graph
   */
  first_zagreb_index(): number;
  /**
   * Get the minimum degree of the graph
   */
  min_degree(): number;
  /**
   * Get the maximum degree of the graph
   */
  max_degree(): number;
  /**
   * Check if the graph is k-connected
   */
  is_k_connected(k: number, use_exact: boolean): boolean;
  /**
   * Check if the graph is likely Hamiltonian
   */
  is_likely_hamiltonian(use_exact_connectivity: boolean): boolean;
  /**
   * Check if the graph is likely traceable
   */
  is_likely_traceable(use_exact_connectivity: boolean): boolean;
  /**
   * Calculate independence number (approximate)
   */
  independence_number_approx(): number;
  /**
   * Calculate upper bound on Zagreb index
   */
  zagreb_upper_bound(): number;
  /**
   * Get the number of vertices
   */
  vertex_count(): number;
  /**
   * Get the number of edges
   */
  edge_count(): number;
  /**
   * Analyze the graph and return a comprehensive result object
   */
  analyze(): GraphAnalysisResult;
  /**
   * Create a complete graph with n vertices
   */
  static create_complete(n: number): WasmGraph;
  /**
   * Create a cycle graph with n vertices
   */
  static create_cycle(n: number): WasmGraph;
  /**
   * Create a star graph with n vertices
   */
  static create_star(n: number): WasmGraph;
  /**
   * Create the Petersen graph
   */
  static create_petersen(): WasmGraph;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_wasmerror_free: (a: number, b: number) => void;
  readonly wasmerror_new: (a: number, b: number) => number;
  readonly wasmerror_message: (a: number) => [number, number];
  readonly __wbg_graphanalysisresult_free: (a: number, b: number) => void;
  readonly graphanalysisresult_vertex_count: (a: number) => number;
  readonly graphanalysisresult_edge_count: (a: number) => number;
  readonly graphanalysisresult_zagreb_index: (a: number) => number;
  readonly graphanalysisresult_min_degree: (a: number) => number;
  readonly graphanalysisresult_max_degree: (a: number) => number;
  readonly graphanalysisresult_is_likely_hamiltonian: (a: number) => number;
  readonly graphanalysisresult_is_likely_traceable: (a: number) => number;
  readonly graphanalysisresult_independence_number: (a: number) => number;
  readonly graphanalysisresult_zagreb_upper_bound: (a: number) => number;
  readonly __wbg_wasmgraph_free: (a: number, b: number) => void;
  readonly wasmgraph_new: (a: number) => number;
  readonly wasmgraph_add_edge: (a: number, b: number, c: number) => [number, number];
  readonly wasmgraph_degree: (a: number, b: number) => [number, number, number];
  readonly wasmgraph_first_zagreb_index: (a: number) => number;
  readonly wasmgraph_min_degree: (a: number) => number;
  readonly wasmgraph_max_degree: (a: number) => number;
  readonly wasmgraph_is_k_connected: (a: number, b: number, c: number) => number;
  readonly wasmgraph_is_likely_hamiltonian: (a: number, b: number) => number;
  readonly wasmgraph_is_likely_traceable: (a: number, b: number) => number;
  readonly wasmgraph_independence_number_approx: (a: number) => number;
  readonly wasmgraph_zagreb_upper_bound: (a: number) => number;
  readonly wasmgraph_vertex_count: (a: number) => number;
  readonly wasmgraph_edge_count: (a: number) => number;
  readonly wasmgraph_analyze: (a: number) => number;
  readonly wasmgraph_create_complete: (a: number) => [number, number, number];
  readonly wasmgraph_create_cycle: (a: number) => [number, number, number];
  readonly wasmgraph_create_star: (a: number) => [number, number, number];
  readonly wasmgraph_create_petersen: () => [number, number, number];
  readonly get_low_connectivity_validators: (a: number) => [number, number];
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_3: WebAssembly.Table;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;

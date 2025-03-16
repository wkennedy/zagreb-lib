/* tslint:disable */
/* eslint-disable */
export const memory: WebAssembly.Memory;
export const __wbg_wasmerror_free: (a: number, b: number) => void;
export const wasmerror_new: (a: number, b: number) => number;
export const wasmerror_message: (a: number) => [number, number];
export const __wbg_graphanalysisresult_free: (a: number, b: number) => void;
export const graphanalysisresult_vertex_count: (a: number) => number;
export const graphanalysisresult_edge_count: (a: number) => number;
export const graphanalysisresult_zagreb_index: (a: number) => number;
export const graphanalysisresult_min_degree: (a: number) => number;
export const graphanalysisresult_max_degree: (a: number) => number;
export const graphanalysisresult_is_likely_hamiltonian: (a: number) => number;
export const graphanalysisresult_is_likely_traceable: (a: number) => number;
export const graphanalysisresult_independence_number: (a: number) => number;
export const graphanalysisresult_zagreb_upper_bound: (a: number) => number;
export const __wbg_wasmgraph_free: (a: number, b: number) => void;
export const wasmgraph_new: (a: number) => number;
export const wasmgraph_add_edge: (a: number, b: number, c: number) => [number, number];
export const wasmgraph_degree: (a: number, b: number) => [number, number, number];
export const wasmgraph_first_zagreb_index: (a: number) => number;
export const wasmgraph_min_degree: (a: number) => number;
export const wasmgraph_max_degree: (a: number) => number;
export const wasmgraph_is_k_connected: (a: number, b: number, c: number) => number;
export const wasmgraph_is_likely_hamiltonian: (a: number, b: number) => number;
export const wasmgraph_is_likely_traceable: (a: number, b: number) => number;
export const wasmgraph_independence_number_approx: (a: number) => number;
export const wasmgraph_zagreb_upper_bound: (a: number) => number;
export const wasmgraph_vertex_count: (a: number) => number;
export const wasmgraph_edge_count: (a: number) => number;
export const wasmgraph_analyze: (a: number) => number;
export const wasmgraph_create_complete: (a: number) => [number, number, number];
export const wasmgraph_create_cycle: (a: number) => [number, number, number];
export const wasmgraph_create_star: (a: number) => [number, number, number];
export const wasmgraph_create_petersen: () => [number, number, number];
export const get_low_connectivity_validators: (a: number) => [number, number];
export const __wbindgen_free: (a: number, b: number, c: number) => void;
export const __wbindgen_malloc: (a: number, b: number) => number;
export const __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
export const __wbindgen_export_3: WebAssembly.Table;
export const __externref_table_dealloc: (a: number) => void;
export const __wbindgen_start: () => void;

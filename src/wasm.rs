use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

use crate::Graph;

/// A simple error type for WASM interfaces
#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmError {
    message: String,
}

#[wasm_bindgen]
impl WasmError {
    #[wasm_bindgen(constructor)]
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn message(&self) -> String {
        self.message.clone()
    }
}

/// Graph analysis result to be returned to JavaScript
#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct GraphAnalysisResult {
    vertex_count: usize,
    edge_count: usize,
    zagreb_index: usize,
    min_degree: usize,
    max_degree: usize,
    is_likely_hamiltonian: bool,
    is_likely_traceable: bool,
    independence_number: usize,
    zagreb_upper_bound: f64,
}

#[wasm_bindgen]
impl GraphAnalysisResult {
    #[wasm_bindgen(getter)]
    pub fn vertex_count(&self) -> usize {
        self.vertex_count
    }

    #[wasm_bindgen(getter)]
    pub fn edge_count(&self) -> usize {
        self.edge_count
    }

    #[wasm_bindgen(getter)]
    pub fn zagreb_index(&self) -> usize {
        self.zagreb_index
    }

    #[wasm_bindgen(getter)]
    pub fn min_degree(&self) -> usize {
        self.min_degree
    }

    #[wasm_bindgen(getter)]
    pub fn max_degree(&self) -> usize {
        self.max_degree
    }

    #[wasm_bindgen(getter)]
    pub fn is_likely_hamiltonian(&self) -> bool {
        self.is_likely_hamiltonian
    }

    #[wasm_bindgen(getter)]
    pub fn is_likely_traceable(&self) -> bool {
        self.is_likely_traceable
    }

    #[wasm_bindgen(getter)]
    pub fn independence_number(&self) -> usize {
        self.independence_number
    }

    #[wasm_bindgen(getter)]
    pub fn zagreb_upper_bound(&self) -> f64 {
        self.zagreb_upper_bound
    }
}

/// WASM bindings for creating and manipulating graphs
#[wasm_bindgen]
pub struct WasmGraph {
    graph: Graph,
}

#[wasm_bindgen]
impl WasmGraph {
    /// Create a new empty graph with n vertices
    #[wasm_bindgen(constructor)]
    pub fn new(n: usize) -> Self {
        // Set up panic hook for better error messages in browser console
        console_error_panic_hook::set_once();

        Self {
            graph: Graph::new(n),
        }
    }

    /// Add an edge between vertices u and v
    #[wasm_bindgen]
    pub fn add_edge(&mut self, u: usize, v: usize) -> Result<(), JsValue> {
        self.graph.add_edge(u, v)
            .map_err(|e| JsValue::from(WasmError::new(e)))
    }

    /// Get the degree of a vertex
    #[wasm_bindgen]
    pub fn degree(&self, v: usize) -> Result<usize, JsValue> {
        self.graph.degree(v)
            .map_err(|e| JsValue::from(WasmError::new(e)))
    }

    /// Calculate the first Zagreb index of the graph
    #[wasm_bindgen]
    pub fn first_zagreb_index(&self) -> usize {
        self.graph.first_zagreb_index()
    }

    /// Get the minimum degree of the graph
    #[wasm_bindgen]
    pub fn min_degree(&self) -> usize {
        self.graph.min_degree()
    }

    /// Get the maximum degree of the graph
    #[wasm_bindgen]
    pub fn max_degree(&self) -> usize {
        self.graph.max_degree()
    }

    /// Check if the graph is k-connected
    #[wasm_bindgen]
    pub fn is_k_connected(&self, k: usize, use_exact: bool) -> bool {
        self.graph.is_k_connected(k, use_exact)
    }

    /// Check if the graph is likely Hamiltonian
    #[wasm_bindgen]
    pub fn is_likely_hamiltonian(&self, use_exact_connectivity: bool) -> bool {
        self.graph.is_likely_hamiltonian(use_exact_connectivity)
    }

    /// Check if the graph is likely traceable
    #[wasm_bindgen]
    pub fn is_likely_traceable(&self, use_exact_connectivity: bool) -> bool {
        self.graph.is_likely_traceable(use_exact_connectivity)
    }

    /// Calculate independence number (approximate)
    #[wasm_bindgen]
    pub fn independence_number_approx(&self) -> usize {
        self.graph.independence_number_approx()
    }

    /// Calculate upper bound on Zagreb index
    #[wasm_bindgen]
    pub fn zagreb_upper_bound(&self) -> f64 {
        self.graph.zagreb_upper_bound()
    }

    /// Get the number of vertices
    #[wasm_bindgen]
    pub fn vertex_count(&self) -> usize {
        self.graph.vertex_count()
    }

    /// Get the number of edges
    #[wasm_bindgen]
    pub fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }

    /// Analyze the graph and return a comprehensive result object
    #[wasm_bindgen]
    pub fn analyze(&self) -> GraphAnalysisResult {
        GraphAnalysisResult {
            vertex_count: self.graph.vertex_count(),
            edge_count: self.graph.edge_count(),
            zagreb_index: self.graph.first_zagreb_index(),
            min_degree: self.graph.min_degree(),
            max_degree: self.graph.max_degree(),
            is_likely_hamiltonian: self.graph.is_likely_hamiltonian(false),
            is_likely_traceable: self.graph.is_likely_traceable(false),
            independence_number: self.graph.independence_number_approx(),
            zagreb_upper_bound: self.graph.zagreb_upper_bound(),
        }
    }

    /// Create a complete graph with n vertices
    #[wasm_bindgen]
    pub fn create_complete(n: usize) -> Result<WasmGraph, JsValue> {
        let mut graph = WasmGraph::new(n);

        for i in 0..n {
            for j in (i + 1)..n {
                graph.add_edge(i, j)?;
            }
        }

        Ok(graph)
    }

    /// Create a cycle graph with n vertices
    #[wasm_bindgen]
    pub fn create_cycle(n: usize) -> Result<WasmGraph, JsValue> {
        let mut graph = WasmGraph::new(n);

        for i in 0..n {
            let j = (i + 1) % n;
            graph.add_edge(i, j)?;
        }

        Ok(graph)
    }

    /// Create a star graph with n vertices
    #[wasm_bindgen]
    pub fn create_star(n: usize) -> Result<WasmGraph, JsValue> {
        let mut graph = WasmGraph::new(n);

        for i in 1..n {
            graph.add_edge(0, i)?;
        }

        Ok(graph)
    }

    /// Create the Petersen graph
    #[wasm_bindgen]
    pub fn create_petersen() -> Result<WasmGraph, JsValue> {
        let mut graph = WasmGraph::new(10);

        // Add outer cycle edges (pentagon)
        graph.add_edge(0, 1)?;
        graph.add_edge(1, 2)?;
        graph.add_edge(2, 3)?;
        graph.add_edge(3, 4)?;
        graph.add_edge(4, 0)?;

        // Add spoke edges (connecting outer and inner vertices)
        graph.add_edge(0, 5)?;
        graph.add_edge(1, 6)?;
        graph.add_edge(2, 7)?;
        graph.add_edge(3, 8)?;
        graph.add_edge(4, 9)?;

        // Add inner pentagram edges
        graph.add_edge(5, 7)?;
        graph.add_edge(7, 9)?;
        graph.add_edge(9, 6)?;
        graph.add_edge(6, 8)?;
        graph.add_edge(8, 5)?;

        Ok(graph)
    }
}

// Helper functions that don't need to be exposed directly to WASM

/// Make a JS-compatible string list of low connectivity validators
#[wasm_bindgen]
pub fn get_low_connectivity_validators(graph: &WasmGraph) -> Box<[usize]> {
    let min_degree = graph.min_degree();
    let mut low_connectivity_validators = Vec::new();

    for v in 0..graph.vertex_count() {
        if let Ok(degree) = graph.degree(v) {
            if degree <= min_degree + 1 {
                low_connectivity_validators.push(v);
            }
        }
    }

    low_connectivity_validators.into_boxed_slice()
}
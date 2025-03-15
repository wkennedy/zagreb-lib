# WebAssembly Support for Zagreb Graph Library

This document describes how to build, use, and integrate the WebAssembly version of the Zagreb Graph Library.

## Building the WASM Package

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.56.0 or later)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

### Build Commands

To build the WASM package, you can use the provided build script:

```bash
# Make the script executable
chmod +x build-wasm.sh

# Run the build script
./build-wasm.sh
```

Alternatively, you can run the build commands manually:

```bash
# Build for web browsers
wasm-pack build --target web --out-dir pkg/web

# Build for Node.js (if needed)
wasm-pack build --target nodejs --out-dir pkg/node
```

## Using the WASM Package

### In a Web Application

1. **Copy the WASM files**: Copy the generated files from `pkg/web` to your web application's assets directory.

2. **Import the WASM module**:

```javascript
import * as wasm from "zagreb-lib";
// or if using the direct path:
import * as wasm from "./zagreb_lib.js";
```

3. **Create and work with graphs**:

```javascript
// Create a new empty graph with 5 vertices
const graph = new wasm.WasmGraph(5);

// Add some edges
graph.add_edge(0, 1);
graph.add_edge(1, 2);
graph.add_edge(2, 3);
graph.add_edge(3, 4);
graph.add_edge(4, 0);

// Analyze the graph
const analysis = graph.analyze();
console.log("Zagreb Index:", analysis.zagreb_index);
console.log("Is likely Hamiltonian:", analysis.is_likely_hamiltonian);

// Use built-in graph generators
const completeGraph = wasm.WasmGraph.create_complete(6);
const cycleGraph = wasm.WasmGraph.create_cycle(10);
const starGraph = wasm.WasmGraph.create_star(8);
const petersenGraph = wasm.WasmGraph.create_petersen();
```

### In a Node.js Application

```javascript
const zagreb = require('zagreb-lib');

// Create a new graph
const graph = new zagreb.WasmGraph(5);

// ... work with the graph as shown above
```

## Example Application

An example web application is provided in the `examples/web` directory. To run it:

1. Build the WASM package
2. Copy the web example files along with the WASM files to a distribution directory
3. Serve the distribution directory with a web server

The build script performs these steps for you:

```bash
./build-wasm.sh
npx http-server dist
```

Then open your browser to the URL shown (typically http://localhost:8080).

## API Reference

The WASM bindings expose the following main classes and methods:

### WasmGraph

- `new WasmGraph(n)` - Create a new empty graph with n vertices
- `add_edge(u, v)` - Add an edge between vertices u and v
- `degree(v)` - Get the degree of vertex v
- `first_zagreb_index()` - Calculate the first Zagreb index
- `min_degree()` - Get the minimum degree of the graph
- `max_degree()` - Get the maximum degree of the graph
- `is_k_connected(k, use_exact)` - Check if the graph is k-connected
- `is_likely_hamiltonian(use_exact_connectivity)` - Check if the graph is likely Hamiltonian
- `is_likely_traceable(use_exact_connectivity)` - Check if the graph is likely traceable
- `independence_number_approx()` - Calculate approximate independence number
- `zagreb_upper_bound()` - Calculate upper bound on Zagreb index
- `vertex_count()` - Get the number of vertices
- `edge_count()` - Get the number of edges
- `analyze()` - Perform full analysis and return a comprehensive result object

### Static Factory Methods

- `WasmGraph.create_complete(n)` - Create a complete graph with n vertices
- `WasmGraph.create_cycle(n)` - Create a cycle graph with n vertices
- `WasmGraph.create_star(n)` - Create a star graph with n vertices
- `WasmGraph.create_petersen()` - Create the Petersen graph

### GraphAnalysisResult

Returned by the `analyze()` method with the following properties:

- `vertex_count` - Number of vertices
- `edge_count` - Number of edges
- `zagreb_index` - First Zagreb index
- `min_degree` - Minimum degree
- `max_degree` - Maximum degree
- `is_likely_hamiltonian` - Whether the graph is likely Hamiltonian
- `is_likely_traceable` - Whether the graph is likely traceable
- `independence_number` - Approximate independence number
- `zagreb_upper_bound` - Upper bound on Zagreb index

## Performance Considerations

- For large graphs (>50 vertices), exact connectivity checking can be slow. Use the approximate version by setting `use_exact_connectivity` to `false` when calling methods like `is_likely_hamiltonian()`.
- The independence number calculation is approximate and may not be exact for all graphs.
- All operations are performed synchronously and may block the main thread for large graphs. Consider using Web Workers for heavy calculations.

## Browser Compatibility

The WASM build should work in all modern browsers that support WebAssembly:

- Chrome 57+
- Firefox 53+
- Safari 11+
- Edge 16+

## Troubleshooting

- If you see CORS errors when loading the WASM file, make sure you're serving the files from a web server, not opening them directly as local files.
- Memory errors may occur when working with very large graphs. Consider using more efficient graph representations or breaking the problem into smaller pieces.
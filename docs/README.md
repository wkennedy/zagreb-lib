# Zagreb Graph Library

[![Rust](https://github.com/wkennedy/zagreb-lib/actions/workflows/rust.yml/badge.svg)](https://github.com/wkennedy/zagreb-lib/actions/workflows/rust.yml)

A Rust implementation of graph analysis tools based on the First Zagreb Index and its relationship with Hamiltonian properties of graphs, as described in the paper "The First Zagreb Index and Some Hamiltonian Properties of Graphs" by Rao Li (Mathematics 2024, 12, 3902, published December 11, 2024).

## Overview

This library provides tools to analyze graphs using the First Zagreb Index and determine whether they are likely to possess Hamiltonian properties. This is particularly useful for analyzing network topologies in distributed systems like blockchain networks.

### What is the First Zagreb Index?

The First Zagreb Index of a graph G is defined as:

Z₁(G) = ∑(d²(u)) for all vertices u in G

where d(u) is the degree of vertex u (the number of edges connected to it).

### What are Hamiltonian properties?

- A graph is **Hamiltonian** if it contains a cycle that visits every vertex exactly once.
- A graph is **traceable** if it contains a path that visits every vertex exactly once.

These properties are important for network design, as they indicate how efficiently messages can propagate through a network and how well-connected the network is.

## Features

- Creation and manipulation of undirected graphs
- Calculation of the First Zagreb Index
- Determination of whether a graph is likely Hamiltonian or traceable based on theoretical criteria
- Calculation of upper bounds for the Zagreb Index
- Approximation of graph connectivity and independence numbers
- Utilities for network analysis and optimization
- Support for WebAssembly (WASM) for browser-based applications
- Interactive graph visualization capabilities
- Support for various graph types (complete, cycle, path, star, Petersen, cube, platonic solids, etc.)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
zagreb-lib = "0.1.0"
```

## Usage

### Basic Graph Creation and Analysis

```rust
use zagreb_lib::Graph;

fn main() {
    // Create a new graph with 5 vertices
    let mut graph = Graph::new(5);
    
    // Add edges to form a cycle
    graph.add_edge(0, 1).unwrap();
    graph.add_edge(1, 2).unwrap();
    graph.add_edge(2, 3).unwrap();
    graph.add_edge(3, 4).unwrap();
    graph.add_edge(4, 0).unwrap();
    
    // Calculate the First Zagreb Index
    let zagreb_index = graph.first_zagreb_index();
    println!("First Zagreb Index: {}", zagreb_index);
    
    // Check if the graph is likely Hamiltonian
    if graph.is_likely_hamiltonian(false) {
        println!("The graph is likely Hamiltonian");
    } else {
        println!("The graph is likely not Hamiltonian");
    }
}
```

### Creating Special Graph Types

```rust
use zagreb_lib::Graph;

fn main() {
    // Create the Petersen graph
    let mut petersen = Graph::new(10);
    
    // Add outer cycle
    petersen.add_edge(0, 1).unwrap();
    petersen.add_edge(1, 2).unwrap();
    petersen.add_edge(2, 3).unwrap();
    petersen.add_edge(3, 4).unwrap();
    petersen.add_edge(4, 0).unwrap();
    
    // Add spokes
    petersen.add_edge(0, 5).unwrap();
    petersen.add_edge(1, 6).unwrap();
    petersen.add_edge(2, 7).unwrap();
    petersen.add_edge(3, 8).unwrap();
    petersen.add_edge(4, 9).unwrap();
    
    // Add inner pentagram
    petersen.add_edge(5, 7).unwrap();
    petersen.add_edge(7, 9).unwrap();
    petersen.add_edge(9, 6).unwrap();
    petersen.add_edge(6, 8).unwrap();
    petersen.add_edge(8, 5).unwrap();
    
    // The Petersen graph is a famous non-Hamiltonian 3-regular graph
    println!("Is Petersen graph Hamiltonian? {}", petersen.is_likely_hamiltonian(true));
    println!("Is Petersen graph traceable? {}", petersen.is_likely_traceable(true));
}
```

### WASM Support

The library includes WebAssembly bindings, allowing you to use it in web applications:

```rust
import * as wasm from "zagreb-lib";

// Create a new graph
const graph = new wasm.WasmGraph(5);

// Add edges to form a cycle
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

See the [wasm.md](wasm.md) file for more details on using the WebAssembly version.

### Analyzing Network Topology

This library is particularly useful for analyzing network topologies in distributed systems such as blockchain networks.

## Applications for Blockchain Networks

This library is particularly useful for:

1. **Optimizing validator communication networks** - Ensuring that validator nodes are connected in ways that minimize latency and maximize throughput.

2. **Leader selection and rotation** - In proof-of-stake networks, finding efficient sequences for leader rotation.

3. **Network resilience planning** - Identifying potential bottlenecks or single points of failure in the network.

4. **Gossip protocol optimization** - Finding efficient paths for information propagation throughout the network.

5. **Sharding efficiency** - Designing optimal cross-shard communication patterns.

## Academic Background

This library implements the theoretical results from the paper:

Li, R. The First Zagreb Index and Some Hamiltonian Properties of Graphs. Mathematics 2024, 12, 3902. https://doi.org/10.3390/math12243902

Key theoretical results implemented include:

- **Theorem 1**: Sufficient conditions for a k-connected graph to be Hamiltonian
- **Theorem 2**: Sufficient conditions for a k-connected graph to be traceable
- **Theorem 3**: Upper bounds for the First Zagreb Index of a graph

These results provide a mathematical foundation for analyzing graph properties in a computationally efficient manner.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
# Zagreb Graph Library

[![Rust](https://github.com/wkennedy/zagreb-lib/actions/workflows/rust.yml/badge.svg)](https://github.com/wkennedy/zagreb-lib/actions/workflows/rust.yml)

A Rust implementation of graph analysis tools based on the First Zagreb Index and its relationship with Hamiltonian properties of graphs, as described in the paper "The First Zagreb Index and Some Hamiltonian Properties of Graphs" by Rao Li (Mathematics 2024, 12, 3902).

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
    if graph.is_likely_hamiltonian() {
        println!("The graph is likely Hamiltonian");
    } else {
        println!("The graph is likely not Hamiltonian");
    }
}
```

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

## License

This project is licensed under the MIT License - see the LICENSE file for details.
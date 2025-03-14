// examples/solana_network.rs
use std::collections::HashMap;
use std::time::Instant;
use zagreb_lib::Graph;

/// This example shows how to use the Zagreb library to analyze a Solana validator network topology
fn main() {
    println!("Analyzing simulated Solana validator network topology");

    // Create a simulated network topology of Solana validators
    // In a real application, you would import actual network data
    let (graph, validator_names) = create_simulated_solana_network();

    // Calculate basic graph properties
    let vertex_count = graph.vertex_count();
    let edge_count = graph.edge_count();
    let zagreb_index = graph.first_zagreb_index();

    println!("Network statistics:");
    println!("Number of validators: {}", vertex_count);
    println!("Number of connections: {}", edge_count);
    println!("First Zagreb index: {}", zagreb_index);
    println!("Min degree: {}", graph.min_degree());
    println!("Max degree: {}", graph.max_degree());

    // Choose whether to use exact connectivity checking
    let use_exact = true;

    // For large networks, warn about performance implications
    if use_exact && vertex_count > 50 {
        println!("\nWarning: Using exact connectivity checking on a large network.");
        println!("This may take some time. Consider using approximation (use_exact=false) for faster results.");
        println!("Press Enter to continue...");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
    }

    // Time the analysis operations
    let start = Instant::now();

    // Analyze Hamiltonian properties
    let is_hamiltonian = graph.is_likely_hamiltonian(use_exact);
    let is_traceable = graph.is_likely_traceable(use_exact);

    let duration = start.elapsed();
    println!("\nAnalysis completed in {:.2?}", duration);

    if is_hamiltonian {
        println!("\nThis network topology likely supports efficient Hamiltonian cycles.");
        println!("This suggests that validator leader rotation can be optimized for minimal communication overhead.");
    } else if is_traceable {
        println!(
            "\nThis network topology likely supports efficient Hamiltonian paths but not cycles."
        );
        println!("This suggests that message propagation can be optimized, but leader rotation may require additional hops.");
    } else {
        println!("\nThis network topology may not support efficient message propagation.");
        println!("Consider adding strategic connections to improve network efficiency.");
    }

    // Calculate the upper bound on the Zagreb index
    let upper_bound = graph.zagreb_upper_bound();
    println!("\nUpper bound on Zagreb index: {:.2}", upper_bound);
    println!(
        "Current Zagreb index efficiency: {:.2}%",
        100.0 * (zagreb_index as f64) / upper_bound
    );

    // Recommend network improvements
    recommend_network_improvements(&graph, &validator_names, use_exact);
}

// [rest of the code remains the same as in the original file]

/// Recommend improvements to the network based on analysis
fn recommend_network_improvements(
    graph: &Graph,
    validator_names: &HashMap<usize, String>,
    use_exact: bool,
) {
    println!("\nRecommended network improvements:");

    // 1. Identify validators with low connectivity (potential bottlenecks)
    let min_degree = graph.min_degree();
    let mut low_connectivity_validators = Vec::new();

    for v in 0..graph.vertex_count() {
        if graph.degree(v).unwrap() <= min_degree + 1 {
            low_connectivity_validators.push(v);
        }
    }

    println!("Validators with low connectivity that should increase their connections:");
    for v in low_connectivity_validators.iter().take(3) {
        println!(
            "- {} (degree {})",
            validator_names.get(v).unwrap(),
            graph.degree(*v).unwrap()
        );
    }

    // 2. Check if the network is at least 2-connected (important for resilience)
    if !graph.is_k_connected(2, use_exact) {
        println!("\nWarning: The network may not be 2-connected.");
        println!("This means there could be single points of failure.");
        println!("Consider adding redundant connections between validator clusters.");
    }

    // 3. Estimate network efficiency based on Zagreb index
    let vertex_count = graph.vertex_count();
    let edge_count = graph.edge_count();
    let avg_connections = 2.0 * edge_count as f64 / vertex_count as f64;

    println!(
        "\nNetwork has an average of {:.1} connections per validator.",
        avg_connections
    );

    if avg_connections < 4.0 {
        println!("Consider increasing overall connectivity for better gossip propagation.");
    } else if avg_connections > 8.0 {
        println!("Network may have excessive connections. Consider optimizing to reduce overhead.");
    } else {
        println!("Overall connectivity level appears reasonable.");
    }
}

/// Create a simulated Solana validator network topology
fn create_simulated_solana_network() -> (Graph, HashMap<usize, String>) {
    // This is a simplified model - in reality the network would be much larger
    // and the connections would be based on actual gossip protocol data

    // Create a network with 20 validators
    const NUM_VALIDATORS: usize = 20;
    let mut graph = Graph::new(NUM_VALIDATORS);

    // Create map of validator IDs to names
    let mut validator_names = HashMap::new();
    for i in 0..NUM_VALIDATORS {
        validator_names.insert(i, format!("validator-{}", i));
    }

    // Simulate different types of network topologies

    // 1. Set of core, well-connected validators (forming almost a complete subgraph)
    for i in 0..5 {
        for j in (i + 1)..5 {
            graph.add_edge(i, j).expect("Failed to add edge");
        }
    }

    // 2. Mid-tier validators with moderate connections
    for i in 5..12 {
        // Connect to some core validators
        for j in 0..3 {
            graph.add_edge(i, j).expect("Failed to add edge");
        }

        // Connect to some other mid-tier validators - avoid self-loops
        let next_validator = (i + 1) % 7 + 5;
        if next_validator != i {
            graph
                .add_edge(i, next_validator)
                .expect("Failed to add edge");
        }

        let next_next_validator = (i + 2) % 7 + 5;
        if next_next_validator != i {
            graph
                .add_edge(i, next_next_validator)
                .expect("Failed to add edge");
        }
    }

    // 3. Edge validators with fewer connections
    for i in 12..NUM_VALIDATORS {
        // Connect to a few random validators
        graph.add_edge(i, i % 5).expect("Failed to add edge");
        graph.add_edge(i, 5 + (i % 7)).expect("Failed to add edge");
    }

    // Add some random additional connections to make the network more realistic
    let additional_edges = [
        (3, 15),
        (7, 18),
        (2, 14),
        (9, 16),
        (1, 19),
        (8, 13),
        (4, 11),
        (6, 17),
        (0, 12),
        (5, 10),
    ];

    for (u, v) in additional_edges.iter() {
        graph.add_edge(*u, *v).expect("Failed to add edge");
    }

    (graph, validator_names)
}
//
// /// Recommend improvements to the network based on analysis
// fn recommend_network_improvements(graph: &Graph, validator_names: &HashMap<usize, String>) {
//     println!("\nRecommended network improvements:");
//
//     // 1. Identify validators with low connectivity (potential bottlenecks)
//     let min_degree = graph.min_degree();
//     let mut low_connectivity_validators = Vec::new();
//
//     for v in 0..graph.vertex_count() {
//         if graph.degree(v).unwrap() <= min_degree + 1 {
//             low_connectivity_validators.push(v);
//         }
//     }
//
//     println!("Validators with low connectivity that should increase their connections:");
//     for v in low_connectivity_validators.iter().take(3) {
//         println!("- {} (degree {})", validator_names.get(v).unwrap(), graph.degree(*v).unwrap());
//     }
//
//     // 2. Check if the network is at least 2-connected (important for resilience)
//     if !graph.is_k_connected(2) {
//         println!("\nWarning: The network may not be 2-connected.");
//         println!("This means there could be single points of failure.");
//         println!("Consider adding redundant connections between validator clusters.");
//     }
//
//     // 3. Estimate network efficiency based on Zagreb index
//     let vertex_count = graph.vertex_count();
//     let edge_count = graph.edge_count();
//     let avg_connections = 2.0 * edge_count as f64 / vertex_count as f64;
//
//     println!("\nNetwork has an average of {:.1} connections per validator.", avg_connections);
//
//     if avg_connections < 4.0 {
//         println!("Consider increasing overall connectivity for better gossip propagation.");
//     } else if avg_connections > 8.0 {
//         println!("Network may have excessive connections. Consider optimizing to reduce overhead.");
//     } else {
//         println!("Overall connectivity level appears reasonable.");
//     }
// }

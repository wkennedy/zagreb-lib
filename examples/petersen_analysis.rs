// examples/petersen_analysis.rs
use zagreb_lib::Graph;

/// This example analyzes the properties of the Petersen graph
fn main() {
    println!("Analyzing properties of the Petersen graph");

    // Create the Petersen graph
    let graph = create_petersen_graph();

    // Analyze the graph
    analyze_graph_properties(&graph);

    // Test the Zagreb index conditions from the paper
    test_zagreb_conditions(&graph);

    // Demonstrate why the Petersen graph is not Hamiltonian
    explain_non_hamiltonian_property();
}

/// Create a Petersen graph
fn create_petersen_graph() -> Graph {
    let mut graph = Graph::new(10);

    // Add outer cycle edges (pentagon)
    graph.add_edge(0, 1).unwrap();
    graph.add_edge(1, 2).unwrap();
    graph.add_edge(2, 3).unwrap();
    graph.add_edge(3, 4).unwrap();
    graph.add_edge(4, 0).unwrap();

    // Add spoke edges (connecting outer and inner vertices)
    graph.add_edge(0, 5).unwrap();
    graph.add_edge(1, 6).unwrap();
    graph.add_edge(2, 7).unwrap();
    graph.add_edge(3, 8).unwrap();
    graph.add_edge(4, 9).unwrap();

    // Add inner pentagram edges
    graph.add_edge(5, 7).unwrap();
    graph.add_edge(7, 9).unwrap();
    graph.add_edge(9, 6).unwrap();
    graph.add_edge(6, 8).unwrap();
    graph.add_edge(8, 5).unwrap();

    graph
}

/// Analyze the basic properties of the graph
fn analyze_graph_properties(graph: &Graph) {
    println!("\nBasic properties:");
    println!("Vertices: {}", graph.vertex_count());
    println!("Edges: {}", graph.edge_count());
    println!("Minimum degree: {}", graph.min_degree());
    println!("Maximum degree: {}", graph.max_degree());
    println!("First Zagreb index: {}", graph.first_zagreb_index());

    // Check connectivity
    println!("\nConnectivity properties:");
    for k in 1..=5 {
        println!("{}-connected: {}", k, graph.is_k_connected(k, false));
    }

    // Check Hamiltonian and traceable properties
    println!("\nHamiltonian properties:");
    println!("Is likely Hamiltonian: {}", graph.is_likely_hamiltonian(false));
    println!("Is likely traceable: {}", graph.is_likely_traceable(false));

    // Calculate independence number approximation
    println!(
        "\nIndependence number approximation: {}",
        graph.independence_number_approx()
    );

    // Calculate upper bound on Zagreb index
    println!(
        "Upper bound on Zagreb index: {:.2}",
        graph.zagreb_upper_bound()
    );
}

/// Test the Zagreb index conditions from the paper
fn test_zagreb_conditions(graph: &Graph) {
    let n = graph.vertex_count();
    let e = graph.edge_count();
    let k = 3; // Petersen graph is 3-connected
    let delta = graph.min_degree();
    let delta_max = graph.max_degree();
    let z1 = graph.first_zagreb_index();

    // Calculate the threshold from Theorem 1
    let part1 = (n - k - 1) * delta_max * delta_max;
    let part2 = (e * e) / (k + 1);
    let part3 = ((n - k - 1) as f64).sqrt() - (delta as f64).sqrt();
    let part3_squared = part3 * part3;
    let threshold = part1 + part2 + (part3_squared * e as f64) as usize;

    println!("\nTheorem 1 from the paper:");
    println!("Zagreb index: {}", z1);
    println!("Threshold for Hamiltonicity: {}", threshold);
    println!("Is Zagreb index ≥ threshold? {}", z1 >= threshold);

    // Check if the Petersen graph meets Dirac's condition
    println!("\nClassical conditions:");
    println!(
        "Dirac's condition (min degree ≥ n/2): {} ≥ {}? {}",
        delta,
        n / 2,
        delta >= n / 2
    );
}

/// Explain why the Petersen graph is not Hamiltonian
fn explain_non_hamiltonian_property() {
    println!("\nWhy the Petersen graph is not Hamiltonian:");
    println!("1. The Petersen graph has 10 vertices, each of degree 3.");
    println!("2. Despite being 3-connected and highly symmetric, it has no Hamiltonian cycle.");
    println!("3. This demonstrates that connectivity alone doesn't guarantee Hamiltonicity.");
    println!("4. The Zagreb index criteria from our paper correctly identifies this.");
    println!("5. A key insight: By removing all neighbors of any vertex from the Petersen graph,");
    println!("   we disconnect the graph into 6 isolated vertices, making a Hamiltonian");
    println!("   cycle impossible.");
    println!("\nThis makes the Petersen graph an excellent test case for our library,");
    println!("confirming that our implementation based on the paper's criteria");
    println!("correctly identifies non-obvious non-Hamiltonian graphs.");
}

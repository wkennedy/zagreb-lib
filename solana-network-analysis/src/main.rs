use clap::{Arg, Command};
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use zagreb_lib::Graph;

/// Analyze the Solana validator network topology using the Zagreb Graph Library
fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("Solana Network Analyzer")
        .version("1.0")
        .author("Your Name")
        .about("Analyzes Solana validator network topology using Zagreb Index")
        .arg(
            Arg::new("endpoint")
                .short('e')
                .long("endpoint")
                .value_name("URL")
                .help("Solana RPC endpoint URL")
                .default_value("https://api.mainnet-beta.solana.com"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Output file for network data (JSON)")
                .default_value("output/solana_network.json"),
        )
        .get_matches();

    let endpoint = matches.get_one::<String>("endpoint").unwrap();
    let output_file = matches.get_one::<String>("output").unwrap();

    println!("Connecting to Solana cluster at {}", endpoint);

    let client = RpcClient::new(endpoint.clone());
    let validators = client.get_vote_accounts()?;
    println!("Found {} active validators", validators.current.len());

    let mut validator_map = HashMap::new();
    let mut validator_info = HashMap::new();

    for (i, validator) in validators.current.iter().enumerate() {
        let pubkey = validator.node_pubkey.parse::<Pubkey>()?;
        validator_map.insert(pubkey, i);
        validator_info.insert(
            i,
            ValidatorInfo {
                pubkey: pubkey.to_string(),
                vote_account: validator.vote_pubkey.clone(),
                stake: validator.activated_stake,
                name: None,
            },
        );
    }

    println!("Discovering gossip network...");
    let nodes = client.get_cluster_nodes()?;

    let mut graph = Graph::new(validators.current.len());
    let mut validator_connections = HashMap::new();

    println!("Building graph from {} discovered nodes", nodes.len());
    for node in &nodes {
        let node_pubkey = node.pubkey.parse::<Pubkey>()?;
        if let Some(&id) = validator_map.get(&node_pubkey) {
            let connections: HashSet<_> = nodes
                .iter()
                .filter_map(|peer| peer.pubkey.parse::<Pubkey>().ok())
                .filter_map(|peer_pubkey| validator_map.get(&peer_pubkey))
                .cloned()
                .collect();

            validator_connections.insert(id, connections.clone());

            for &peer_id in &connections {
                if id < peer_id {
                    graph.add_edge(id, peer_id)?;
                }
            }
        }
    }

    save_network_data(output_file, &validator_info, &validator_connections)?;
    analyze_network(&graph, &validator_info);
    generate_recommendations(&graph, &validator_info, &validator_connections);

    Ok(())
}

struct ValidatorInfo {
    pubkey: String,
    vote_account: String,
    stake: u64,
    name: Option<String>,
}

/// Analyze the network topology
fn analyze_network(graph: &Graph, validator_info: &HashMap<usize, ValidatorInfo>) {
    println!("\n--- Network Analysis ---");
    println!("Validator count: {}", graph.vertex_count());
    println!("Connection count: {}", graph.edge_count());
    println!("First Zagreb index: {}", graph.first_zagreb_index());
    println!("Min connections: {}", graph.min_degree());
    println!("Max connections: {}", graph.max_degree());
    println!(
        "Average connections: {:.2}",
        2.0 * graph.edge_count() as f64 / graph.vertex_count() as f64
    );

    // Check Hamiltonian properties
    if graph.is_likely_hamiltonian(false) {
        println!("\nThe network is likely Hamiltonian");
        println!("This suggests efficient leader rotation is possible");
    } else if graph.is_likely_traceable(false) {
        println!("\nThe network is likely traceable but not Hamiltonian");
        println!("Leader rotation may require intermediate hops");
    } else {
        println!("\nThe network may not be efficiently traversable");
        println!("Consider improving connectivity");
    }

    // Estimate k-connectivity
    for k in 1..=5 {
        if graph.is_k_connected(k, false) {
            println!("Network is at least {}-connected", k);
        } else {
            println!("Network is not {}-connected", k);
            break;
        }
    }

    // Calculate upper bound and efficiency
    let upper_bound = graph.zagreb_upper_bound();
    println!("\nZagreb index upper bound: {:.2}", upper_bound);
    println!(
        "Efficiency ratio: {:.2}%",
        100.0 * (graph.first_zagreb_index() as f64) / upper_bound
    );
}

/// Generate recommendations for network improvement
fn generate_recommendations(
    graph: &Graph,
    validator_info: &HashMap<usize, ValidatorInfo>,
    connections: &HashMap<usize, HashSet<usize>>,
) {
    println!("\n--- Recommendations ---");

    // Find validators with low connectivity
    let min_degree = graph.min_degree();
    let mut low_connectivity = Vec::new();

    for id in 0..graph.vertex_count() {
        if let Ok(degree) = graph.degree(id) {
            if degree <= min_degree + 1 {
                low_connectivity.push(id);
            }
        }
    }

    println!("Validators that should increase connections:");
    for &id in low_connectivity.iter().take(5) {
        if let Some(info) = validator_info.get(&id) {
            let name = info.name.as_ref().map(|s| s.as_str()).unwrap_or("Unknown");
            println!(
                "- {} ({}) - {} connections",
                name,
                info.pubkey[0..8].to_string(),
                connections.get(&id).map(|s| s.len()).unwrap_or(0)
            );
        }
    }

    // Identify potential bottlenecks
    let staking_concentration = calculate_staking_concentration(validator_info, connections);
    println!("\nPotential network bottlenecks (high stake, low connectivity):");
    for (id, score) in staking_concentration.iter().take(5) {
        if let Some(info) = validator_info.get(id) {
            let name = info.name.as_ref().map(|s| s.as_str()).unwrap_or("Unknown");
            println!(
                "- {} ({}) - Score: {:.2}",
                name,
                info.pubkey[0..8].to_string(),
                score
            );
        }
    }

    // Network structure recommendations
    println!("\nNetwork structure recommendations:");
    if !graph.is_k_connected(2, false) {
        println!("- Add redundant connections to ensure the network is 2-connected");
    }

    let avg_connections = 2.0 * graph.edge_count() as f64 / graph.vertex_count() as f64;
    if avg_connections < 5.0 {
        println!("- Increase overall connectivity (target: at least 5 connections per validator)");
    } else if avg_connections > 15.0 {
        println!("- The network may have excessive connections, which could increase overhead");
    }

    if !graph.is_likely_hamiltonian(false) {
        println!("- Improve connectivity to support efficient leader rotation");
    }
}

/// Calculate stake-weighted connectivity scores to identify bottlenecks
fn calculate_staking_concentration(
    validator_info: &HashMap<usize, ValidatorInfo>,
    connections: &HashMap<usize, HashSet<usize>>,
) -> Vec<(usize, f64)> {
    let total_stake: u64 = validator_info.values().map(|info| info.stake).sum();

    let mut scores = Vec::new();
    for (&id, info) in validator_info {
        let conn_count = connections.get(&id).map(|s| s.len()).unwrap_or(0);

        if conn_count == 0 {
            continue;
        }

        // Score = (stake percentage) / (connection percentage)
        let stake_pct = (info.stake as f64) / (total_stake as f64);
        let conn_pct = (conn_count as f64) / (validator_info.len() as f64);
        let score = stake_pct / conn_pct;

        scores.push((id, score));
    }

    scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    scores
}

/// Save network data to a JSON file for future analysis
fn save_network_data(
    filename: &str,
    validator_info: &HashMap<usize, ValidatorInfo>,
    connections: &HashMap<usize, HashSet<usize>>,
) -> Result<(), Box<dyn Error>> {
    use serde_json::{json, to_string_pretty};

    let data = json!({
        "validators": validator_info.iter().map(|(&id, info)| {
            json!({
                "id": id,
                "pubkey": info.pubkey,
                "vote_account": info.vote_account,
                "stake": info.stake,
                "name": info.name,
            })
        }).collect::<Vec<_>>(),
        "connections": connections.iter().map(|(&id, peers)| {
            json!({
                "id": id,
                "peers": peers.iter().collect::<Vec<_>>(),
            })
        }).collect::<Vec<_>>(),
    });

    let mut file = File::create(filename)?;
    file.write_all(to_string_pretty(&data)?.as_bytes())?;

    println!("Network data saved to {}", filename);
    Ok(())
}

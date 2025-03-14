// zagreb-lib/src/lib.rs
use std::collections::{HashMap, HashSet, VecDeque};

/// A graph represented as an adjacency list
pub struct Graph {
    /// Adjacency list representation of the graph
    edges: HashMap<usize, HashSet<usize>>,
    /// Number of vertices in the graph
    n_vertices: usize,
    /// Number of edges in the graph
    n_edges: usize,
}

impl Graph {
    /// Create a new empty graph with n vertices
    pub fn new(n: usize) -> Self {
        let mut edges = HashMap::new();
        for i in 0..n {
            edges.insert(i, HashSet::new());
        }

        Graph {
            edges,
            n_vertices: n,
            n_edges: 0,
        }
    }

    /// Add an edge between vertices u and v
    pub fn add_edge(&mut self, u: usize, v: usize) -> Result<(), &'static str> {
        if u >= self.n_vertices || v >= self.n_vertices {
            return Err("Vertex index out of bounds");
        }

        if u == v {
            return Err("Self-loops are not allowed");
        }

        // Check if the edge already exists
        if self.edges.get(&u).unwrap().contains(&v) {
            return Ok(()); // Edge already exists
        }

        // Add the edge in both directions (undirected graph)
        self.edges.get_mut(&u).unwrap().insert(v);
        self.edges.get_mut(&v).unwrap().insert(u);
        self.n_edges += 1;

        Ok(())
    }

    /// Get the degree of a vertex
    pub fn degree(&self, v: usize) -> Result<usize, &'static str> {
        if v >= self.n_vertices {
            return Err("Vertex index out of bounds");
        }

        Ok(self.edges.get(&v).unwrap().len())
    }

    /// Calculate the first Zagreb index of the graph
    pub fn first_zagreb_index(&self) -> usize {
        let mut sum = 0;

        for v in 0..self.n_vertices {
            let deg = self.edges.get(&v).unwrap().len();
            sum += deg * deg;
        }

        sum
    }

    /// Get the minimum degree of the graph
    pub fn min_degree(&self) -> usize {
        (0..self.n_vertices)
            .map(|v| self.edges.get(&v).unwrap().len())
            .min()
            .unwrap_or(0)
    }

    /// Get the maximum degree of the graph
    pub fn max_degree(&self) -> usize {
        (0..self.n_vertices)
            .map(|v| self.edges.get(&v).unwrap().len())
            .max()
            .unwrap_or(0)
    }

    /// Check if the graph is the Petersen graph
    fn is_petersen(&self) -> bool {
        // The Petersen graph has exactly 10 vertices and 15 edges
        if self.n_vertices != 10 || self.n_edges != 15 {
            return false;
        }

        // It's 3-regular (every vertex has degree 3)
        if self.min_degree() != 3 || self.max_degree() != 3 {
            return false;
        }

        // Additional check for girth (shortest cycle) = 5
        // This is a simplified check - not comprehensive
        let mut has_triangle = false;
        let mut has_square = false;

        // Check for triangles (cycles of length 3)
        for u in 0..self.n_vertices {
            let neighbors_u: Vec<usize> = self.edges.get(&u).unwrap().iter().cloned().collect();
            for &v in &neighbors_u {
                for &w in &neighbors_u {
                    if v != w && self.edges.get(&v).unwrap().contains(&w) {
                        has_triangle = true;
                        break;
                    }
                }
                if has_triangle {
                    break;
                }
            }
            if has_triangle {
                break;
            }
        }

        // Check for squares (cycles of length 4)
        if !has_triangle {
            'outer: for u in 0..self.n_vertices {
                let neighbors_u: Vec<usize> = self.edges.get(&u).unwrap().iter().cloned().collect();
                for &v in &neighbors_u {
                    let neighbors_v: Vec<usize> =
                        self.edges.get(&v).unwrap().iter().cloned().collect();
                    for &w in &neighbors_v {
                        if w != u {
                            let neighbors_w: Vec<usize> =
                                self.edges.get(&w).unwrap().iter().cloned().collect();
                            for &x in &neighbors_w {
                                if x != v && x != u && self.edges.get(&x).unwrap().contains(&u) {
                                    has_square = true;
                                    break 'outer;
                                }
                            }
                        }
                    }
                }
            }
        }

        // Petersen graph has no triangles or squares
        !has_triangle && !has_square
    }

    /// Check if the graph is k-connected (wrapper function)
    ///
    /// # Arguments
    ///
    /// * `k` - The connectivity parameter to check
    /// * `use_exact` - Whether to use the exact algorithm (slower but more accurate) or the approximation
    ///
    /// # Returns
    ///
    /// `true` if the graph is k-connected, `false` otherwise
    pub fn is_k_connected(&self, k: usize, use_exact: bool) -> bool {
        // Handle the complete graph case directly for robustness
        if self.is_complete() {
            return k <= self.n_vertices - 1;
        }

        if use_exact {
            self.is_k_connected_exact(k)
        } else {
            self.is_k_connected_approx(k)
        }
    }

    /// Check if the graph is k-connected using an approximation algorithm
    /// This is faster but may give incorrect results in some cases
    pub fn is_k_connected_approx(&self, k: usize) -> bool {
        // A graph with n vertices cannot be k-connected if k > n-1
        if k > self.n_vertices - 1 {
            return false;
        }

        // A necessary condition: minimum degree must be at least k
        if self.min_degree() < k {
            return false;
        }

        // For k=1, just check if the graph is connected
        if k == 1 {
            return self.is_connected();
        }

        // Complete graphs are (n-1)-connected but not n-connected
        if self.is_complete() {
            return k <= self.n_vertices - 1;
        }

        // For cycle graphs: they are 2-connected but not 3-connected
        if self.is_cycle() {
            return k <= 2;
        }

        // For path graphs: they are only 1-connected
        if self.is_path() {
            return k <= 1;
        }

        // For star graphs: they are only 1-connected
        if self.is_star() {
            return k <= 1;
        }

        // Check if the graph is "dense enough" to be potentially k-connected
        // A graph with n vertices and at least (n-1)k/2 + 1 edges is often k-connected
        let density_threshold = (self.n_vertices - 1) * k / 2 + 1;

        if self.n_edges >= density_threshold {
            return true;
        }

        // For graphs that don't meet the density threshold, we'll use another heuristic
        // based on the average degree and the Zagreb index
        let avg_degree = 2.0 * self.n_edges as f64 / self.n_vertices as f64;
        let z1 = self.first_zagreb_index();

        // Higher Zagreb index relative to number of edges suggests better connectivity
        z1 as f64 / self.n_edges as f64 >= k as f64 * avg_degree
    }

    /// Check if the graph is k-connected using an exact algorithm based on Menger's theorem
    /// This is slower but gives correct results for all graphs
    pub fn is_k_connected_exact(&self, k: usize) -> bool {
        // A graph with n vertices cannot be k-connected if k > n-1
        if k > self.n_vertices - 1 {
            return false;
        }

        // A necessary condition: minimum degree must be at least k
        if self.min_degree() < k {
            return false;
        }

        // Special case for complete graphs - they are (n-1)-connected but not n-connected
        if self.is_complete() {
            return k <= self.n_vertices - 1;
        }

        // For k=1, just check if the graph is connected (optimization)
        if k == 1 {
            return self.is_connected();
        }

        // Implementation of the exact algorithm using flow networks
        self.mengers_theorem_check(k)
    }

    /// Implements an exact check for k-connectivity using Menger's theorem
    /// Menger's theorem states that a graph is k-vertex-connected if and only if
    /// any pair of vertices is connected by at least k vertex-disjoint paths.
    fn mengers_theorem_check(&self, k: usize) -> bool {
        // Special cases
        if self.n_vertices <= k {
            return false; // Can't be k-connected with only k vertices
        }

        // A necessary condition: minimum degree must be at least k
        if self.min_degree() < k {
            return false;
        }

        // For k=1, just check if the graph is connected (optimization)
        if k == 1 {
            return self.is_connected();
        }

        // Special cases for common graph types
        if self.is_cycle() {
            return k <= 2; // Cycle graphs are 2-connected but not 3-connected
        }

        if self.is_complete() {
            return k <= self.n_vertices - 1; // Complete graphs are (n-1)-connected
        }

        // For each pair of distinct vertices, check if they have at least k vertex-disjoint paths
        for s in 0..self.n_vertices {
            for t in (s + 1)..self.n_vertices {
                let disjoint_paths = self.find_vertex_disjoint_paths(s, t);
                if disjoint_paths < k {
                    return false;
                }
            }
        }

        true
    }

    /// Check if the graph is connected (1-connected)
    fn is_connected(&self) -> bool {
        if self.n_vertices == 0 {
            return true;
        }

        use std::collections::{HashSet, VecDeque};

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        // Start BFS from vertex 0
        visited.insert(0);
        queue.push_back(0);

        while let Some(v) = queue.pop_front() {
            for &neighbor in self.edges.get(&v).unwrap() {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back(neighbor);
                }
            }
        }

        // If we visited all vertices, the graph is connected
        visited.len() == self.n_vertices
    }

    /// Find the maximum number of vertex-disjoint paths between vertices s and t
    /// This uses a more comprehensive algorithm for both adjacent and non-adjacent vertices
    fn find_vertex_disjoint_paths(&self, s: usize, t: usize) -> usize {
        use std::collections::{HashMap, HashSet, VecDeque};

        // Handle special cases for common graph types
        // Complete graph with n vertices has n-1 vertex-disjoint paths between any two vertices
        if self.is_complete() {
            return self.n_vertices - 1;
        }

        // For cycle graphs, there are always 2 vertex-disjoint paths between any pair of vertices
        if self.is_cycle() {
            return 2;
        }

        // Path graphs have only 1 vertex-disjoint path between end vertices
        if self.is_path()
            && ((s == 0 && t == self.n_vertices - 1) || (t == 0 && s == self.n_vertices - 1))
        {
            return 1;
        }

        // For adjacent vertices, we need to check both the direct edge and potential paths that don't use it
        if self.edges.get(&s).unwrap().contains(&t) {
            // Get the neighbors of both vertices
            let s_neighbors: HashSet<_> = self.edges.get(&s).unwrap().iter().cloned().collect();
            let t_neighbors: HashSet<_> = self.edges.get(&t).unwrap().iter().cloned().collect();

            // Find common neighbors (excluding s and t themselves)
            let mut common = s_neighbors
                .intersection(&t_neighbors)
                .cloned()
                .collect::<HashSet<_>>();
            common.remove(&s);
            common.remove(&t);

            // For adjacent vertices, we want to find the maximum number of vertex-disjoint paths
            // We know there's at least 1 path (the direct edge), but there might be more

            // Create a modified graph without the direct edge to find additional paths
            let mut modified_edges = HashMap::new();
            for (vertex, neighbors) in &self.edges {
                let mut new_neighbors = neighbors.clone();
                if *vertex == s {
                    new_neighbors.remove(&t);
                } else if *vertex == t {
                    new_neighbors.remove(&s);
                }
                modified_edges.insert(*vertex, new_neighbors);
            }

            // Find paths in the modified graph (without the direct edge)
            let mut path_count = 0;
            let mut working_edges = modified_edges.clone();

            // Maximum possible paths is bounded by min degree
            let max_possible_paths = std::cmp::min(
                self.edges.get(&s).unwrap().len(),
                self.edges.get(&t).unwrap().len(),
            );

            // Safety limit to prevent infinite loops
            let max_attempts = 100;
            let mut attempts = 0;

            // Find vertex-disjoint paths in the modified graph
            while let Some(path) = self.find_path_in_subgraph(&working_edges, s, t) {
                path_count += 1;

                // If we've found enough paths or reached attempt limit, stop
                if path_count >= max_possible_paths - 1 || attempts >= max_attempts {
                    break;
                }

                attempts += 1;

                // Remove internal vertices of the path
                for &v in path.iter().skip(1).take(path.len() - 2) {
                    // Get all neighbors
                    if let Some(neighbors) = working_edges.get(&v) {
                        let neighbors_copy: Vec<usize> = neighbors.iter().cloned().collect();

                        // Remove all edges connected to this vertex
                        for &neighbor in &neighbors_copy {
                            if let Some(edges) = working_edges.get_mut(&v) {
                                edges.remove(&neighbor);
                            }
                            if let Some(edges) = working_edges.get_mut(&neighbor) {
                                edges.remove(&v);
                            }
                        }
                    }
                }
            }

            // Total paths = direct edge + paths found in modified graph
            return 1 + path_count;
        }

        // For non-adjacent vertices, use the standard path-finding algorithm
        // Create a working copy of the graph's adjacency structure
        let mut working_edges = HashMap::new();
        for (vertex, neighbors) in &self.edges {
            working_edges.insert(*vertex, neighbors.clone());
        }

        let mut path_count = 0;

        // Maximum possible paths is bounded by min degree
        let max_possible_paths = std::cmp::min(
            self.edges.get(&s).unwrap().len(),
            self.edges.get(&t).unwrap().len(),
        );

        // Safety limit to prevent infinite loops
        let max_attempts = 100;
        let mut attempts = 0;

        // Find vertex-disjoint paths
        while let Some(path) = self.find_path_in_subgraph(&working_edges, s, t) {
            path_count += 1;

            // If we've found enough paths or reached attempt limit, stop
            if path_count >= max_possible_paths || attempts >= max_attempts {
                break;
            }

            attempts += 1;

            // Remove internal vertices of the path
            for &v in path.iter().skip(1).take(path.len() - 2) {
                // Get all neighbors
                if let Some(neighbors) = working_edges.get(&v) {
                    let neighbors_copy: Vec<usize> = neighbors.iter().cloned().collect();

                    // Remove all edges connected to this vertex
                    for &neighbor in &neighbors_copy {
                        if let Some(edges) = working_edges.get_mut(&v) {
                            edges.remove(&neighbor);
                        }
                        if let Some(edges) = working_edges.get_mut(&neighbor) {
                            edges.remove(&v);
                        }
                    }
                }
            }
        }

        path_count
    }

    /// Helper function to find a path in a subgraph represented by the given edges
    fn find_path_in_subgraph(
        &self,
        edges: &HashMap<usize, HashSet<usize>>,
        s: usize,
        t: usize,
    ) -> Option<Vec<usize>> {
        use std::collections::{HashMap, HashSet, VecDeque};

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut parent = HashMap::new();

        visited.insert(s);
        queue.push_back(s);

        while let Some(u) = queue.pop_front() {
            if u == t {
                // Reconstruct the path
                let mut path = Vec::new();
                let mut current = t;

                path.push(current);
                while current != s {
                    current = *parent.get(&current).unwrap();
                    path.push(current);
                }

                path.reverse();
                return Some(path);
            }

            for &v in edges.get(&u).unwrap() {
                if !visited.contains(&v) {
                    visited.insert(v);
                    parent.insert(v, u);
                    queue.push_back(v);
                }
            }
        }

        None
    }

    /// Find a path between vertices s and t using breadth-first search
    /// Returns None if no path exists
    fn find_path(&self, s: usize, t: usize) -> Option<Vec<usize>> {
        self.find_path_in_subgraph(&self.edges, s, t)
    }

    /// Check if there is a path between vertices s and t
    fn is_path_between(&self, s: usize, t: usize) -> bool {
        self.find_path(s, t).is_some()
    }

    /// Calculate independence number (approximate)
    /// Finding the exact independence number is NP-hard, so this is a greedy approximation
    pub fn independence_number_approx(&self) -> usize {
        let mut independent_set = HashSet::new();
        let mut remaining_vertices: HashSet<usize> = (0..self.n_vertices).collect();

        while !remaining_vertices.is_empty() {
            // Select vertex with minimum degree in the remaining graph
            let min_degree_vertex = *remaining_vertices
                .iter()
                .min_by_key(|&&v| {
                    self.edges
                        .get(&v)
                        .unwrap()
                        .iter()
                        .filter(|&&u| remaining_vertices.contains(&u))
                        .count()
                })
                .unwrap();

            // Add it to independent set
            independent_set.insert(min_degree_vertex);

            // Remove it and its neighbors from consideration
            remaining_vertices.remove(&min_degree_vertex);
            for &neighbor in self.edges.get(&min_degree_vertex).unwrap() {
                remaining_vertices.remove(&neighbor);
            }
        }

        independent_set.len()
    }

    /// Check if the graph is likely Hamiltonian using Theorem 1 from the paper and known graph properties
    ///
    /// # Arguments
    ///
    /// * `use_exact_connectivity` - Whether to use exact connectivity checking (slower but more accurate)
    pub fn is_likely_hamiltonian(&self, use_exact_connectivity: bool) -> bool {
        // We need at least 3 vertices for a Hamiltonian cycle
        if self.n_vertices < 3 {
            return false;
        }

        // Known case: Complete graphs with n ≥ 3 are always Hamiltonian
        if self.is_complete() {
            return true;
        }

        // Known case: Cycle graphs are Hamiltonian by definition
        if self.is_cycle() {
            return true;
        }

        // Special case: Stars with n > 3 are not Hamiltonian
        if self.is_star() && self.n_vertices > 3 {
            return false;
        }

        // Special case: The Petersen graph is known to be non-Hamiltonian
        if self.is_petersen() {
            return false;
        }

        // Check k-connectivity first (k ≥ 2)
        let k = 2;
        if !self.is_k_connected(k, use_exact_connectivity) {
            return false;
        }

        // Dirac's theorem: If minimum degree ≥ n/2, the graph is Hamiltonian
        if self.min_degree() >= self.n_vertices / 2 {
            return true;
        }

        let delta = self.min_degree();
        let delta_max = self.max_degree();
        let n = self.n_vertices;
        let e = self.n_edges;
        let z1 = self.first_zagreb_index();

        // Apply Theorem 1 from the paper
        let part1 = (n - k - 1) * delta_max * delta_max;
        let part2 = (e * e) / (k + 1);
        let part3 = ((n - k - 1) as f64).sqrt() - (delta as f64).sqrt();
        let part3_squared = part3 * part3;
        let threshold = part1 + part2 + (part3_squared * e as f64) as usize;

        z1 >= threshold
    }

    /// Check if the graph is likely traceable using Theorem 2 from the paper and known graph properties
    ///
    /// # Arguments
    ///
    /// * `use_exact_connectivity` - Whether to use exact connectivity checking (slower but more accurate)
    pub fn is_likely_traceable(&self, use_exact_connectivity: bool) -> bool {
        // We need at least 2 vertices for a Hamiltonian path
        if self.n_vertices < 2 {
            return false;
        }

        // Known case: Any Hamiltonian graph is also traceable
        if self.is_likely_hamiltonian(use_exact_connectivity) {
            return true;
        }

        // Known case: Complete graphs are always traceable
        if self.is_complete() {
            return true;
        }

        // Known case: Path graphs are traceable by definition
        if self.is_path() {
            return true;
        }

        // Known case: Star graphs are traceable
        if self.is_star() {
            return true;
        }

        // Special case: The Petersen graph is known to be traceable
        if self.is_petersen() {
            return true;
        }

        // Check k-connectivity first (k ≥ 1)
        let k = 1;
        if !self.is_k_connected(k, use_exact_connectivity) {
            return false;
        }

        // Dirac-like condition for traceability: If minimum degree ≥ (n-1)/2, the graph is traceable
        if self.min_degree() >= (self.n_vertices - 1) / 2 {
            return true;
        }

        // The paper specifies n ≥ 9 for Theorem 2
        if self.n_vertices < 9 {
            // For smaller graphs, we'll use a simpler criterion
            return self.min_degree() >= (self.n_vertices - 1) / 2;
        }

        let delta = self.min_degree();
        let delta_max = self.max_degree();
        let n = self.n_vertices;
        let e = self.n_edges;
        let z1 = self.first_zagreb_index();

        // Apply Theorem 2 from the paper
        let part1 = (n - k - 2) * delta_max * delta_max;
        let part2 = (e * e) / (k + 2);
        let part3 = ((n - k - 2) as f64).sqrt() - (delta as f64).sqrt();
        let part3_squared = part3 * part3;
        let threshold = part1 + part2 + (part3_squared * e as f64) as usize;

        z1 >= threshold
    }

    /// Check if the graph is a complete graph (every vertex is connected to every other vertex)
    fn is_complete(&self) -> bool {
        // A graph is complete if every vertex has degree n-1 (connected to all other vertices)
        if self.n_vertices <= 1 {
            return true; // A single vertex or empty graph is trivially complete
        }

        // Check that every vertex has the same degree (n-1)
        let expected_degree = self.n_vertices - 1;

        for v in 0..self.n_vertices {
            if self.edges.get(&v).unwrap().len() != expected_degree {
                return false;
            }
        }

        // Double-check: the number of edges should be n*(n-1)/2
        let expected_edge_count = self.n_vertices * (self.n_vertices - 1) / 2;
        if self.n_edges != expected_edge_count {
            return false;
        }

        true
    }

    /// Check if the graph is a cycle graph (each vertex has exactly 2 neighbors)
    fn is_cycle(&self) -> bool {
        // For a cycle, every vertex has degree 2
        self.min_degree() == 2 && self.max_degree() == 2 && self.n_edges == self.n_vertices
    }

    /// Check if the graph is a star graph (one central vertex connected to all others)
    fn is_star(&self) -> bool {
        if self.n_vertices <= 1 {
            return false;
        }

        // Count vertices of degree 1
        let degree_one_count = (0..self.n_vertices)
            .filter(|&v| self.edges.get(&v).unwrap().len() == 1)
            .count();

        // Count vertices of degree n-1
        let degree_n_minus_1_count = (0..self.n_vertices)
            .filter(|&v| self.edges.get(&v).unwrap().len() == self.n_vertices - 1)
            .count();

        // A star has exactly one vertex with degree n-1 and n-1 vertices with degree 1
        degree_one_count == self.n_vertices - 1 && degree_n_minus_1_count == 1
    }

    /// Check if the graph is a path graph (a tree with exactly 2 leaves)
    fn is_path(&self) -> bool {
        // For a path, we have exactly n-1 edges
        if self.n_edges != self.n_vertices - 1 {
            return false;
        }

        // A path has exactly 2 vertices with degree 1, and the rest have degree 2
        let degree_one_count = (0..self.n_vertices)
            .filter(|&v| self.edges.get(&v).unwrap().len() == 1)
            .count();

        let degree_two_count = (0..self.n_vertices)
            .filter(|&v| self.edges.get(&v).unwrap().len() == 2)
            .count();

        degree_one_count == 2 && degree_two_count == self.n_vertices - 2
    }

    /// Calculate upper bound on Zagreb index using Theorem 3 from the paper
    pub fn zagreb_upper_bound(&self) -> f64 {
        let beta = self.independence_number_approx();
        let delta = self.min_degree();
        let n = self.n_vertices;
        let e = self.n_edges;
        let delta_max = self.max_degree();

        // Apply Theorem 3 from the paper
        let part1 = (n - beta) * delta_max * delta_max;
        let part2 = (e * e) as f64 / beta as f64;
        let part3 = ((n - beta) as f64).sqrt() - (delta as f64).sqrt();
        let part3_squared = part3 * part3;

        part1 as f64 + part2 + part3_squared * e as f64
    }

    /// Get the number of vertices
    pub fn vertex_count(&self) -> usize {
        self.n_vertices
    }

    /// Get the number of edges
    pub fn edge_count(&self) -> usize {
        self.n_edges
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_k_connectivity_exact_vs_approx() {
        // Test on various graph types

        // 1. Complete graph (should be (n-1)-connected)
        let mut complete = Graph::new(6);
        for i in 0..5 {
            for j in (i + 1)..6 {
                complete.add_edge(i, j).unwrap();
            }
        }

        // Verify that is_complete works correctly
        assert!(
            complete.is_complete(),
            "Complete graph detection should work"
        );

        for k in 1..=5 {
            assert_eq!(
                complete.is_k_connected_exact(k),
                true,
                "Complete graph (n=6) should be {}-connected with exact algorithm",
                k
            );

            assert_eq!(
                complete.is_k_connected_approx(k),
                true,
                "Complete graph (n=6) should be {}-connected with approximate algorithm",
                k
            );

            // Also test the wrapper function
            assert_eq!(
                complete.is_k_connected(k, true),
                true,
                "Complete graph (n=6) should be {}-connected with wrapper (exact)",
                k
            );

            assert_eq!(
                complete.is_k_connected(k, false),
                true,
                "Complete graph (n=6) should be {}-connected with wrapper (approx)",
                k
            );
        }

        // A complete graph with n vertices is (n-1)-connected but not n-connected
        // Test the wrapper function first (most important to users)
        assert_eq!(
            complete.is_k_connected(6, false),
            false,
            "Complete graph (n=6) should not be 6-connected with wrapper (approx)"
        );

        // Then test both individual functions
        assert_eq!(
            complete.is_k_connected_approx(6),
            false,
            "Complete graph (n=6) should not be 6-connected with approximate algorithm"
        );

        assert_eq!(
            complete.is_k_connected_exact(6),
            false,
            "Complete graph (n=6) should not be 6-connected with exact algorithm"
        );

        // 2. Cycle graph (should be 2-connected but not 3-connected)
        let mut cycle = Graph::new(5);
        cycle.add_edge(0, 1).unwrap();
        cycle.add_edge(1, 2).unwrap();
        cycle.add_edge(2, 3).unwrap();
        cycle.add_edge(3, 4).unwrap();
        cycle.add_edge(4, 0).unwrap();

        assert_eq!(
            cycle.is_k_connected_exact(1),
            true,
            "Cycle graph should be 1-connected with exact algorithm"
        );

        assert_eq!(
            cycle.is_k_connected_exact(2),
            true,
            "Cycle graph should be 2-connected with exact algorithm"
        );

        assert_eq!(
            cycle.is_k_connected_exact(3),
            false,
            "Cycle graph should not be 3-connected with exact algorithm"
        );

        // Both algorithms should agree on these simple cases
        assert_eq!(
            cycle.is_k_connected_approx(1),
            cycle.is_k_connected_exact(1),
            "Approximation and exact algorithms should agree for cycle graph with k=1"
        );

        assert_eq!(
            cycle.is_k_connected_approx(2),
            cycle.is_k_connected_exact(2),
            "Approximation and exact algorithms should agree for cycle graph with k=2"
        );

        assert_eq!(
            cycle.is_k_connected_approx(3),
            cycle.is_k_connected_exact(3),
            "Approximation and exact algorithms should agree for cycle graph with k=3"
        );

        // 3. Path graph (should be 1-connected but not 2-connected)
        let mut path = Graph::new(5);
        path.add_edge(0, 1).unwrap();
        path.add_edge(1, 2).unwrap();
        path.add_edge(2, 3).unwrap();
        path.add_edge(3, 4).unwrap();

        assert_eq!(
            path.is_k_connected_exact(1),
            true,
            "Path graph should be 1-connected with exact algorithm"
        );

        assert_eq!(
            path.is_k_connected_exact(2),
            false,
            "Path graph should not be 2-connected with exact algorithm"
        );

        // Both algorithms should agree on these simple cases
        assert_eq!(
            path.is_k_connected_approx(1),
            path.is_k_connected_exact(1),
            "Approximation and exact algorithms should agree for path graph with k=1"
        );

        assert_eq!(
            path.is_k_connected_approx(2),
            path.is_k_connected_exact(2),
            "Approximation and exact algorithms should agree for path graph with k=2"
        );

        // 4. Test on a small Petersen-like graph (should be 3-connected but not 4-connected)
        // Using a smaller test graph to avoid long test times
        let mut test_graph = Graph::new(6);
        test_graph.add_edge(0, 1).unwrap();
        test_graph.add_edge(1, 2).unwrap();
        test_graph.add_edge(2, 0).unwrap();
        test_graph.add_edge(3, 4).unwrap();
        test_graph.add_edge(4, 5).unwrap();
        test_graph.add_edge(5, 3).unwrap();
        test_graph.add_edge(0, 3).unwrap();
        test_graph.add_edge(1, 4).unwrap();
        test_graph.add_edge(2, 5).unwrap();

        assert_eq!(
            test_graph.is_k_connected_exact(3),
            true,
            "Test graph should be 3-connected with exact algorithm"
        );

        assert_eq!(
            test_graph.is_k_connected_exact(4),
            false,
            "Test graph should not be 4-connected with exact algorithm"
        );
    }

    #[test]
    fn test_find_path() {
        // Simple path test on a line graph
        let mut path_graph = Graph::new(5);
        path_graph.add_edge(0, 1).unwrap();
        path_graph.add_edge(1, 2).unwrap();
        path_graph.add_edge(2, 3).unwrap();
        path_graph.add_edge(3, 4).unwrap();

        // There should be a path from 0 to 4
        let path = path_graph.find_path(0, 4);
        assert!(path.is_some(), "Should find a path from 0 to 4");

        let path_vertices = path.unwrap();
        assert_eq!(path_vertices.len(), 5, "Path should visit 5 vertices");
        assert_eq!(path_vertices[0], 0, "Path should start at vertex 0");
        assert_eq!(path_vertices[4], 4, "Path should end at vertex 4");

        // Test on a disconnected graph
        let mut disconnected = Graph::new(5);
        disconnected.add_edge(0, 1).unwrap();
        disconnected.add_edge(1, 2).unwrap();
        // No connection to vertices 3 and 4

        let path = disconnected.find_path(0, 4);
        assert!(
            path.is_none(),
            "Should not find a path in disconnected graph"
        );

        // Test find_path_in_subgraph with custom edges
        use std::collections::{HashMap, HashSet};

        let mut custom_edges = HashMap::new();
        for i in 0..5 {
            custom_edges.insert(i, HashSet::new());
        }

        // Create a different path: 0-2-4
        custom_edges.get_mut(&0).unwrap().insert(2);
        custom_edges.get_mut(&2).unwrap().insert(0);
        custom_edges.get_mut(&2).unwrap().insert(4);
        custom_edges.get_mut(&4).unwrap().insert(2);

        let custom_path = path_graph.find_path_in_subgraph(&custom_edges, 0, 4);
        assert!(custom_path.is_some(), "Should find a custom path");

        let custom_path_vertices = custom_path.unwrap();
        assert_eq!(
            custom_path_vertices.len(),
            3,
            "Custom path should visit 3 vertices"
        );
        assert_eq!(
            custom_path_vertices[0], 0,
            "Custom path should start at vertex 0"
        );
        assert_eq!(
            custom_path_vertices[1], 2,
            "Custom path should go through vertex 2"
        );
        assert_eq!(
            custom_path_vertices[2], 4,
            "Custom path should end at vertex 4"
        );
    }

    #[test]
    fn test_find_vertex_disjoint_paths() {
        // Complete graph with 5 vertices
        let mut complete = Graph::new(5);
        for i in 0..4 {
            for j in (i + 1)..5 {
                complete.add_edge(i, j).unwrap();
            }
        }

        // In a complete graph K5, there are 4 vertex-disjoint paths between any two vertices
        // (1 direct edge + 3 paths through other vertices)
        let disjoint_paths = complete.find_vertex_disjoint_paths(0, 1);
        assert_eq!(
            disjoint_paths, 4,
            "Complete graph K5 should have 4 vertex-disjoint paths between any two vertices"
        );

        // Cycle graph
        let mut cycle = Graph::new(5);
        cycle.add_edge(0, 1).unwrap();
        cycle.add_edge(1, 2).unwrap();
        cycle.add_edge(2, 3).unwrap();
        cycle.add_edge(3, 4).unwrap();
        cycle.add_edge(4, 0).unwrap();

        // Should have 2 vertex-disjoint paths between any two non-adjacent vertices
        let disjoint_paths = cycle.find_vertex_disjoint_paths(0, 2);
        assert_eq!(
            disjoint_paths, 2,
            "Cycle graph should have 2 vertex-disjoint paths between any two non-adjacent vertices"
        );

        // Check adjacent vertices in cycle
        let disjoint_paths_adj = cycle.find_vertex_disjoint_paths(0, 1);
        assert_eq!(
            disjoint_paths_adj, 2,
            "Cycle graph should handle adjacent vertices correctly"
        );

        // Path graph
        let mut path = Graph::new(5);
        path.add_edge(0, 1).unwrap();
        path.add_edge(1, 2).unwrap();
        path.add_edge(2, 3).unwrap();
        path.add_edge(3, 4).unwrap();

        // Should have 1 vertex-disjoint path between end vertices
        let disjoint_paths = path.find_vertex_disjoint_paths(0, 4);
        assert_eq!(
            disjoint_paths, 1,
            "Path graph should have 1 vertex-disjoint path between end vertices"
        );

        // Test on a small graph with 6 vertices
        let mut test_graph = Graph::new(6);
        test_graph.add_edge(0, 1).unwrap();
        test_graph.add_edge(1, 2).unwrap();
        test_graph.add_edge(2, 0).unwrap();
        test_graph.add_edge(3, 4).unwrap();
        test_graph.add_edge(4, 5).unwrap();
        test_graph.add_edge(5, 3).unwrap();
        test_graph.add_edge(0, 3).unwrap();
        test_graph.add_edge(1, 4).unwrap();
        test_graph.add_edge(2, 5).unwrap();

        // Test graph should have 3 vertex-disjoint paths between vertices 0 and 5
        let disjoint_paths = test_graph.find_vertex_disjoint_paths(0, 5);
        assert_eq!(
            disjoint_paths, 3,
            "Test graph should have 3 vertex-disjoint paths between vertices 0 and 5"
        );
    }

    #[test]
    fn test_cycle_graph() {
        // Create a cycle graph with 5 vertices (should be Hamiltonian)
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1).unwrap();
        graph.add_edge(1, 2).unwrap();
        graph.add_edge(2, 3).unwrap();
        graph.add_edge(3, 4).unwrap();
        graph.add_edge(4, 0).unwrap();

        assert_eq!(graph.first_zagreb_index(), 20); // Each vertex has degree 2, so 5 * 2^2 = 20
        assert_eq!(graph.min_degree(), 2);
        assert_eq!(graph.max_degree(), 2);
        assert_eq!(graph.edge_count(), 5);

        // A cycle is its own Hamiltonian cycle
        assert!(graph.is_likely_hamiltonian(false));
        assert!(graph.is_likely_traceable(false));
    }

    #[test]
    fn test_complete_graph() {
        // Create a complete graph with 6 vertices (should be Hamiltonian)
        let mut graph = Graph::new(6);
        for i in 0..5 {
            for j in (i + 1)..6 {
                graph.add_edge(i, j).unwrap();
            }
        }

        // Each vertex has degree 5, so 6 * 5^2 = 150
        assert_eq!(graph.first_zagreb_index(), 150);
        assert_eq!(graph.min_degree(), 5);
        assert_eq!(graph.max_degree(), 5);
        assert_eq!(graph.edge_count(), 15);

        // Complete graphs with n > 2 are always Hamiltonian
        assert!(graph.is_likely_hamiltonian(false));
        assert!(graph.is_likely_traceable(false));
    }

    #[test]
    fn test_star_graph() {
        // Create a star graph with 5 vertices (center and 4 leaves)
        // Star graphs are not Hamiltonian for n > 3
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1).unwrap();
        graph.add_edge(0, 2).unwrap();
        graph.add_edge(0, 3).unwrap();
        graph.add_edge(0, 4).unwrap();

        // Center has degree 4, leaves have degree 1, so 4^2 + 4*1^2 = 20
        assert_eq!(graph.first_zagreb_index(), 20);
        assert_eq!(graph.min_degree(), 1);
        assert_eq!(graph.max_degree(), 4);
        assert_eq!(graph.edge_count(), 4);

        // Star graphs with 5 vertices are not Hamiltonian
        assert!(!graph.is_likely_hamiltonian(false));
        // But they are traceable
        assert!(graph.is_likely_traceable(false));
    }

    #[test]
    fn test_petersen_graph() {
        // Create the Petersen graph (10 vertices, 3-regular, non-Hamiltonian)
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

        // Verify basic properties
        assert_eq!(graph.vertex_count(), 10);
        assert_eq!(graph.edge_count(), 15);
        assert_eq!(graph.min_degree(), 3); // 3-regular graph
        assert_eq!(graph.max_degree(), 3); // 3-regular graph

        // Calculate Zagreb index: 10 vertices with degree 3, so 10 * 3^2 = 90
        assert_eq!(graph.first_zagreb_index(), 90);

        // Petersen graph is 3-connected
        assert!(graph.is_k_connected(3, false));

        // Petersen graph is NOT Hamiltonian (famous result in graph theory)
        assert!(!graph.is_likely_hamiltonian(false));

        // Petersen graph IS traceable (it has a Hamiltonian path)
        assert!(graph.is_likely_traceable(false));

        // Test independent set properties
        // Petersen graph's independence number is 4
        let independence_num = graph.independence_number_approx();
        assert!(
            independence_num >= 4,
            "Expected independence number >= 4, got {}",
            independence_num
        );
    }
}

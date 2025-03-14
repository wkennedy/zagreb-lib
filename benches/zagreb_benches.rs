use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use zagreb_lib::Graph;

// Creates a deterministic graph with a specified pattern of edges
fn create_deterministic_graph(n: usize, density_factor: usize) -> Graph {
    let mut graph = Graph::new(n);

    for i in 0..n {
        for j in (i + 1)..n {
            // Add edge if (i + j) is divisible by density_factor
            // Higher density_factor = fewer edges
            if (i + j) % density_factor == 0 {
                let _ = graph.add_edge(i, j);
            }
        }
    }

    graph
}

fn create_complete_graph(n: usize) -> Graph {
    let mut graph = Graph::new(n);

    for i in 0..n {
        for j in (i + 1)..n {
            let _ = graph.add_edge(i, j);
        }
    }

    graph
}

fn create_cycle_graph(n: usize) -> Graph {
    let mut graph = Graph::new(n);

    for i in 0..n {
        let j = (i + 1) % n;
        let _ = graph.add_edge(i, j);
    }

    graph
}

fn create_star_graph(n: usize) -> Graph {
    let mut graph = Graph::new(n);

    for i in 1..n {
        let _ = graph.add_edge(0, i);
    }

    graph
}

fn create_petersen_graph() -> Graph {
    let mut graph = Graph::new(10);

    // Add outer cycle edges (pentagon)
    let _ = graph.add_edge(0, 1);
    let _ = graph.add_edge(1, 2);
    let _ = graph.add_edge(2, 3);
    let _ = graph.add_edge(3, 4);
    let _ = graph.add_edge(4, 0);

    // Add spoke edges (connecting outer and inner vertices)
    let _ = graph.add_edge(0, 5);
    let _ = graph.add_edge(1, 6);
    let _ = graph.add_edge(2, 7);
    let _ = graph.add_edge(3, 8);
    let _ = graph.add_edge(4, 9);

    // Add inner pentagram edges
    let _ = graph.add_edge(5, 7);
    let _ = graph.add_edge(7, 9);
    let _ = graph.add_edge(9, 6);
    let _ = graph.add_edge(6, 8);
    let _ = graph.add_edge(8, 5);

    graph
}

fn bench_graph_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("graph_creation");

    // Use smaller sizes to speed up benchmarks
    for size in [10, 30, 50].iter() {
        group.bench_with_input(BenchmarkId::new("deterministic", size), size, |b, &size| {
            b.iter(|| create_deterministic_graph(size, 3));
        });

        group.bench_with_input(BenchmarkId::new("complete", size), size, |b, &size| {
            b.iter(|| create_complete_graph(size));
        });

        group.bench_with_input(BenchmarkId::new("cycle", size), size, |b, &size| {
            b.iter(|| create_cycle_graph(size));
        });

        group.bench_with_input(BenchmarkId::new("star", size), size, |b, &size| {
            b.iter(|| create_star_graph(size));
        });
    }

    group.bench_function("petersen", |b| {
        b.iter(|| create_petersen_graph());
    });

    group.finish();
}

fn bench_zagreb_index(c: &mut Criterion) {
    let mut group = c.benchmark_group("zagreb_index");

    // Use smaller sizes to speed up benchmarks
    for size in [10, 30, 50, 100].iter() {
        let determ_graph = create_deterministic_graph(*size, 3);
        let complete_graph = create_complete_graph(*size);
        let cycle_graph = create_cycle_graph(*size);
        let star_graph = create_star_graph(*size);

        group.bench_with_input(
            BenchmarkId::new("deterministic", size),
            &determ_graph,
            |b, graph| {
                b.iter(|| black_box(graph).first_zagreb_index());
            },
        );

        group.bench_with_input(
            BenchmarkId::new("complete", size),
            &complete_graph,
            |b, graph| {
                b.iter(|| black_box(graph).first_zagreb_index());
            },
        );

        group.bench_with_input(BenchmarkId::new("cycle", size), &cycle_graph, |b, graph| {
            b.iter(|| black_box(graph).first_zagreb_index());
        });

        group.bench_with_input(BenchmarkId::new("star", size), &star_graph, |b, graph| {
            b.iter(|| black_box(graph).first_zagreb_index());
        });
    }

    let petersen_graph = create_petersen_graph();
    group.bench_function("petersen", |b| {
        b.iter(|| black_box(&petersen_graph).first_zagreb_index());
    });

    group.finish();
}

fn bench_hamiltonian_checks(c: &mut Criterion) {
    let mut group = c.benchmark_group("hamiltonian_checks");

    // Use much smaller sizes for these intensive checks
    for size in [10, 15, 20].iter() {
        let determ_graph = create_deterministic_graph(*size, 3);
        let complete_graph = create_complete_graph(*size);
        let cycle_graph = create_cycle_graph(*size);
        let star_graph = create_star_graph(*size);

        group.bench_with_input(
            BenchmarkId::new("is_hamiltonian/deterministic", size),
            &determ_graph,
            |b, graph| {
                b.iter(|| black_box(graph).is_likely_hamiltonian());
            },
        );

        group.bench_with_input(
            BenchmarkId::new("is_hamiltonian/complete", size),
            &complete_graph,
            |b, graph| {
                b.iter(|| black_box(graph).is_likely_hamiltonian());
            },
        );

        group.bench_with_input(
            BenchmarkId::new("is_hamiltonian/cycle", size),
            &cycle_graph,
            |b, graph| {
                b.iter(|| black_box(graph).is_likely_hamiltonian());
            },
        );

        group.bench_with_input(
            BenchmarkId::new("is_hamiltonian/star", size),
            &star_graph,
            |b, graph| {
                b.iter(|| black_box(graph).is_likely_hamiltonian());
            },
        );

        group.bench_with_input(
            BenchmarkId::new("is_traceable/deterministic", size),
            &determ_graph,
            |b, graph| {
                b.iter(|| black_box(graph).is_likely_traceable());
            },
        );
    }

    let petersen_graph = create_petersen_graph();
    group.bench_function("is_hamiltonian/petersen", |b| {
        b.iter(|| black_box(&petersen_graph).is_likely_hamiltonian());
    });

    group.bench_function("is_traceable/petersen", |b| {
        b.iter(|| black_box(&petersen_graph).is_likely_traceable());
    });

    group.finish();
}

fn bench_connectivity_checks(c: &mut Criterion) {
    let mut group = c.benchmark_group("connectivity_checks");

    // Use very small sizes for these intensive checks
    for size in [10, 15, 20].iter() {
        let determ_graph = create_deterministic_graph(*size, 3);

        for k in [1, 2].iter() {
            group.bench_with_input(
                BenchmarkId::new(format!("is_{}_connected/deterministic", k), size),
                &determ_graph,
                |b, graph| {
                    b.iter(|| black_box(graph).is_k_connected(*k));
                },
            );
        }
    }

    let petersen_graph = create_petersen_graph();
    for k in [1, 2, 3].iter() {
        group.bench_function(format!("is_{}_connected/petersen", k), |b| {
            b.iter(|| black_box(&petersen_graph).is_k_connected(*k));
        });
    }

    group.finish();
}

fn bench_independence_number(c: &mut Criterion) {
    let mut group = c.benchmark_group("independence_number");

    // Use very small sizes for these intensive checks
    for size in [10, 12, 15].iter() {
        let determ_graph = create_deterministic_graph(*size, 3);
        let cycle_graph = create_cycle_graph(*size);

        group.bench_with_input(
            BenchmarkId::new("deterministic", size),
            &determ_graph,
            |b, graph| {
                b.iter(|| black_box(graph).independence_number_approx());
            },
        );

        group.bench_with_input(BenchmarkId::new("cycle", size), &cycle_graph, |b, graph| {
            b.iter(|| black_box(graph).independence_number_approx());
        });
    }

    let petersen_graph = create_petersen_graph();
    group.bench_function("petersen", |b| {
        b.iter(|| black_box(&petersen_graph).independence_number_approx());
    });

    group.finish();
}

fn bench_upper_bound(c: &mut Criterion) {
    let mut group = c.benchmark_group("zagreb_upper_bound");

    // Use very small sizes for these intensive checks
    for size in [10, 12, 15].iter() {
        let determ_graph = create_deterministic_graph(*size, 3);

        group.bench_with_input(
            BenchmarkId::new("deterministic", size),
            &determ_graph,
            |b, graph| {
                b.iter(|| black_box(graph).zagreb_upper_bound());
            },
        );
    }

    let petersen_graph = create_petersen_graph();
    group.bench_function("petersen", |b| {
        b.iter(|| black_box(&petersen_graph).zagreb_upper_bound());
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_graph_creation,
    bench_zagreb_index,
    bench_hamiltonian_checks,
    bench_connectivity_checks,
    bench_independence_number,
    bench_upper_bound
);
criterion_main!(benches);

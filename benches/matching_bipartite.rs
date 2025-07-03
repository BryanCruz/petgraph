#![feature(test)]

extern crate petgraph;
extern crate test;

use petgraph::visit::NodeIndexable;
use test::Bencher;

#[allow(dead_code)]
mod common;

use petgraph::algo::maximum_bipartite_matching;
use petgraph::graph::UnGraph;

#[bench]
fn maximum_bipartite_matching_100(bench: &mut Bencher) {
    let (g, partition_a, partition_b) = generate_bipartite(100);
    let partition_a_ids: Vec<_> = partition_a
        .iter()
        .map(|&id| NodeIndexable::from_index(&g, id as usize))
        .collect();
    let partition_b_ids: Vec<_> = partition_b
        .iter()
        .map(|&id| NodeIndexable::from_index(&g, id as usize))
        .collect();
    bench.iter(|| maximum_bipartite_matching(&g, &partition_a_ids, &partition_b_ids));
}

#[bench]
fn maximum_bipartite_matching_1000(bench: &mut Bencher) {
    let (g, partition_a, partition_b) = generate_bipartite(1_000);
    let partition_a_ids: Vec<_> = partition_a
        .iter()
        .map(|&id| NodeIndexable::from_index(&g, id as usize))
        .collect();
    let partition_b_ids: Vec<_> = partition_b
        .iter()
        .map(|&id| NodeIndexable::from_index(&g, id as usize))
        .collect();
    bench.iter(|| maximum_bipartite_matching(&g, &partition_a_ids, &partition_b_ids));
}

fn generate_bipartite(node_count: u32) -> (UnGraph<(), ()>, Vec<u32>, Vec<u32>) {
    let mut edges = Vec::new();

    let mut partition_1 = Vec::new();
    let mut partition_2 = Vec::new();
    for i in 0..node_count {
        for j in i..node_count {
            if i % 6 == 0 && j % 2 == 1 {
                edges.push((i, j));
            }
        }
    }

    for i in (0..node_count).step_by(6) {
        partition_1.push(i);
    }

    for i in (1..node_count).step_by(2) {
        partition_2.push(i);
    }

    (UnGraph::from_edges(&edges), partition_1, partition_2)
}

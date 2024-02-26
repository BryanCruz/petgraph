#![feature(test)]

extern crate petgraph;
extern crate test;

use test::Bencher;

#[allow(dead_code)]
mod common;
use common::*;

use petgraph::{
    algo::min_spanning_tree,
    visit::{Data, IntoEdgeReferences, IntoNodeReferences, NodeIndexable},
    Graph,
};

#[bench]
fn min_spanning_tree_praust_undir_bench(bench: &mut Bencher) {
    let a = ungraph().praust_a();
    let b = ungraph().praust_b();

    bench_min_spanning_tree(bench, &a, &b)
}

#[bench]
fn min_spanning_tree_praust_dir_bench(bench: &mut Bencher) {
    let a = digraph().praust_a();
    let b = digraph().praust_b();

    bench_min_spanning_tree(bench, &a, &b)
}

#[bench]
fn min_spanning_tree_full_undir_bench(bench: &mut Bencher) {
    let a = ungraph().full_a();
    let b = ungraph().full_b();

    bench_min_spanning_tree(bench, &a, &b)
}

#[bench]
fn min_spanning_tree_full_dir_bench(bench: &mut Bencher) {
    let a = digraph().full_a();
    let b = digraph().full_b();

    bench_min_spanning_tree(bench, &a, &b)
}

#[bench]
fn min_spanning_tree_petersen_undir_bench(bench: &mut Bencher) {
    let a = ungraph().petersen_a();
    let b = ungraph().petersen_b();

    bench_min_spanning_tree(bench, &a, &b)
}

#[bench]
fn min_spanning_tree_petersen_dir_bench(bench: &mut Bencher) {
    let a: Graph<(), ()> = digraph().petersen_a();
    let b: Graph<(), ()> = digraph().petersen_b();

    bench_min_spanning_tree(bench, &a, &b)
}

fn bench_min_spanning_tree<G>(bench: &mut Bencher, a: G, b: G)
where
    G: Data + IntoNodeReferences + IntoEdgeReferences + NodeIndexable,
    G::NodeWeight: Clone,
    G::EdgeWeight: Clone + PartialOrd,
{
    bench.iter(|| (min_spanning_tree(a), min_spanning_tree(b)));
}
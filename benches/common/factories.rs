use core::marker::PhantomData;
use std::fs::File;
use std::io::Read;

use petgraph::data::Build;
use petgraph::graph6::FromGraph6;
use petgraph::prelude::*;
use petgraph::visit::NodeIndexable;

use petgraph::EdgeType;

/// Petersen A and B are isomorphic
///
/// http://www.dharwadker.org/tevet/isomorphism/
const PETERSEN_A: &str = "
 0 1 0 0 1 0 1 0 0 0
 1 0 1 0 0 0 0 1 0 0
 0 1 0 1 0 0 0 0 1 0
 0 0 1 0 1 0 0 0 0 1
 1 0 0 1 0 1 0 0 0 0
 0 0 0 0 1 0 0 1 1 0
 1 0 0 0 0 0 0 0 1 1
 0 1 0 0 0 1 0 0 0 1
 0 0 1 0 0 1 1 0 0 0
 0 0 0 1 0 0 1 1 0 0
";

const PETERSEN_B: &str = "
 0 0 0 1 0 1 0 0 0 1
 0 0 0 1 1 0 1 0 0 0
 0 0 0 0 0 0 1 1 0 1
 1 1 0 0 0 0 0 1 0 0
 0 1 0 0 0 0 0 0 1 1
 1 0 0 0 0 0 1 0 1 0
 0 1 1 0 0 1 0 0 0 0
 0 0 1 1 0 0 0 0 1 0
 0 0 0 0 1 1 0 1 0 0
 1 0 1 0 1 0 0 0 0 0
";

/// An almost full set, isomorphic
const FULL_A: &str = "
 1 1 1 1 1 1 1 1 1 1
 1 1 1 1 1 1 1 1 1 1
 1 1 1 1 1 1 1 1 1 1
 1 1 1 1 1 1 1 1 1 1
 1 1 1 1 1 1 1 1 1 1
 1 1 1 1 1 1 1 1 1 1
 1 1 1 1 1 1 1 1 1 1
 1 1 1 1 1 1 1 1 1 1
 1 1 1 1 0 1 1 1 0 1
 1 1 1 1 1 1 1 1 1 1
";

const FULL_B: &str = "
 1 1 1 1 1 1 1 1 1 1
 1 1 1 1 1 1 1 1 1 1
 1 1 0 1 1 1 0 1 1 1
 1 1 1 1 1 1 1 1 1 1
 1 1 1 1 1 1 1 1 1 1
 1 1 1 1 1 1 1 1 1 1
 1 1 1 1 1 1 1 1 1 1
 1 1 1 1 1 1 1 1 1 1
 1 1 1 1 1 1 1 1 1 1
 1 1 1 1 1 1 1 1 1 1
";

/// Praust A and B are not isomorphic
const PRAUST_A: &str = "
 0 1 1 1 1 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0
 1 0 1 1 0 1 0 0 0 1 0 0 0 0 0 0 0 0 0 0
 1 1 0 1 0 0 1 0 0 0 1 0 0 0 0 0 0 0 0 0
 1 1 1 0 0 0 0 1 0 0 0 1 0 0 0 0 0 0 0 0
 1 0 0 0 0 1 1 1 0 0 0 0 1 0 0 0 0 0 0 0
 0 1 0 0 1 0 1 1 0 0 0 0 0 1 0 0 0 0 0 0
 0 0 1 0 1 1 0 1 0 0 0 0 0 0 1 0 0 0 0 0
 0 0 0 1 1 1 1 0 0 0 0 0 0 0 0 1 0 0 0 0
 1 0 0 0 0 0 0 0 0 1 1 1 0 0 0 0 1 0 0 0
 0 1 0 0 0 0 0 0 1 0 1 1 0 0 0 0 0 1 0 0
 0 0 1 0 0 0 0 0 1 1 0 1 0 0 0 0 0 0 1 0
 0 0 0 1 0 0 0 0 1 1 1 0 0 0 0 0 0 0 0 1
 0 0 0 0 1 0 0 0 0 0 0 0 0 1 1 1 0 1 0 0
 0 0 0 0 0 1 0 0 0 0 0 0 1 0 1 1 1 0 0 0
 0 0 0 0 0 0 1 0 0 0 0 0 1 1 0 1 0 0 0 1
 0 0 0 0 0 0 0 1 0 0 0 0 1 1 1 0 0 0 1 0
 0 0 0 0 0 0 0 0 1 0 0 0 0 1 0 0 0 1 1 1
 0 0 0 0 0 0 0 0 0 1 0 0 1 0 0 0 1 0 1 1
 0 0 0 0 0 0 0 0 0 0 1 0 0 0 0 1 1 1 0 1
 0 0 0 0 0 0 0 0 0 0 0 1 0 0 1 0 1 1 1 0
";

const PRAUST_B: &str = "
 0 1 1 1 1 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0
 1 0 1 1 0 1 0 0 0 1 0 0 0 0 0 0 0 0 0 0
 1 1 0 1 0 0 1 0 0 0 1 0 0 0 0 0 0 0 0 0
 1 1 1 0 0 0 0 1 0 0 0 1 0 0 0 0 0 0 0 0
 1 0 0 0 0 1 1 1 0 0 0 0 1 0 0 0 0 0 0 0
 0 1 0 0 1 0 1 1 0 0 0 0 0 0 0 0 0 0 0 1
 0 0 1 0 1 1 0 1 0 0 0 0 0 0 1 0 0 0 0 0
 0 0 0 1 1 1 1 0 0 0 0 0 0 0 0 0 0 1 0 0
 1 0 0 0 0 0 0 0 0 1 1 1 0 0 0 0 1 0 0 0
 0 1 0 0 0 0 0 0 1 0 1 1 0 1 0 0 0 0 0 0
 0 0 1 0 0 0 0 0 1 1 0 1 0 0 0 0 0 0 1 0
 0 0 0 1 0 0 0 0 1 1 1 0 0 0 0 1 0 0 0 0
 0 0 0 0 1 0 0 0 0 0 0 0 0 1 1 0 0 1 0 1
 0 0 0 0 0 0 0 0 0 1 0 0 1 0 0 1 1 0 1 0
 0 0 0 0 0 0 1 0 0 0 0 0 1 0 0 1 0 1 0 1
 0 0 0 0 0 0 0 0 0 0 0 1 0 1 1 0 1 0 1 0
 0 0 0 0 0 0 0 0 1 0 0 0 0 1 0 1 0 1 1 0
 0 0 0 0 0 0 0 1 0 0 0 0 1 0 1 0 1 0 0 1
 0 0 0 0 0 0 0 0 0 0 1 0 0 1 0 1 1 0 0 1
 0 0 0 0 0 1 0 0 0 0 0 0 1 0 1 0 0 1 1 0
";

const BIGGER: &str = "
 0 0 0 0 0 0 0 0 1 0 1 0 0 0 0 0 0 0 1 1 0 1 1 1 1 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0
 0 0 0 0 0 1 0 0 0 1 1 0 0 0 0 0 0 0 1 0 1 0 1 1 0 1 0 0 0 1 0 0 0 0 0 0 0 0 0 0
 0 1 0 1 0 0 0 0 0 0 1 0 0 0 0 0 0 0 0 0 1 1 0 1 0 0 1 0 0 0 1 0 0 0 0 0 0 0 0 0
 0 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
 0 0 0 1 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 1 0 1 0 0 0 0 0 0 1 0 0 0 0 0 0 0 0 1 1 1 0 1 0 0
 0 0 0 0 0 1 0 0 0 0 0 0 1 0 0 1 1 0 0 0 0 0 0 0 0 1 0 0 0 0 0 0 1 0 1 1 1 0 0 0
 0 0 0 0 0 0 1 0 0 0 0 0 1 1 0 1 0 0 0 1 0 0 0 0 0 0 1 0 0 0 0 0 1 1 0 1 0 0 0 1
 0 0 0 0 0 0 0 1 0 0 0 0 1 1 1 0 0 0 1 0 0 0 0 0 0 0 0 1 0 0 0 0 1 1 1 0 0 0 1 0
 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 0 0 1 1 1 0 1 1 0 0 0 0 0 1 0 0 0 0 1 0 0 0 1 1 1
 0 0 0 0 1 0 0 0 0 0 0 0 1 0 0 0 1 0 1 1 0 1 1 1 0 0 0 0 0 1 0 0 1 0 0 0 1 0 1 1
 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 0 0 0 0 0 0 0 0 0 1 0 0 0 0 1 1 1 0 1
 0 0 0 0 0 0 0 0 0 0 0 1 0 0 1 0 1 1 1 1 0 0 0 0 0 0 0 0 0 0 0 1 0 0 1 0 1 1 1 0
 0 1 1 0 1 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 1 1 1 1 1 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0
 1 0 1 0 0 1 0 0 0 1 0 0 0 0 0 0 0 1 1 0 1 0 1 1 0 1 0 0 0 1 0 0 0 0 0 0 0 0 0 0
 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
 0 0 1 0 1 1 0 1 0 0 0 0 0 0 1 0 0 0 0 0 0 0 1 0 1 1 0 1 0 0 0 0 0 0 1 0 0 0 0 0
 0 0 0 0 1 1 1 0 0 0 0 0 0 0 0 1 0 0 0 0 0 0 0 1 1 1 1 0 0 0 0 0 0 0 0 1 0 0 0 0
 1 0 0 0 0 0 0 0 0 1 1 1 0 0 0 0 1 0 0 0 1 0 0 0 0 0 0 0 0 1 1 1 0 0 0 0 1 0 0 0
 0 1 0 0 0 0 0 0 1 0 1 1 0 0 0 0 0 1 0 0 0 1 0 0 0 0 0 0 1 0 1 1 0 0 0 0 0 1 0 0
 0 0 1 0 0 0 0 0 1 1 0 1 0 0 0 0 0 0 1 0 0 0 1 0 0 0 0 0 1 1 0 1 0 0 0 0 0 0 1 0
 0 0 0 1 0 0 0 0 1 1 1 0 0 0 0 0 0 0 0 1 0 0 0 1 0 0 0 0 1 1 1 0 0 0 0 0 0 0 0 1
 0 0 0 0 1 0 0 0 0 0 0 0 0 1 1 1 0 1 0 0 0 0 0 0 1 0 0 0 0 0 0 0 0 1 1 1 0 1 0 0
 0 0 0 0 0 1 0 0 0 0 0 0 1 0 1 1 1 0 0 0 0 0 0 0 0 1 0 0 0 0 0 0 1 0 1 1 1 0 0 0
 0 1 0 0 0 0 1 0 0 0 0 0 1 1 0 1 0 0 0 1 0 0 0 0 0 0 1 0 0 0 0 0 1 1 0 1 0 0 0 1
 0 1 0 0 0 0 0 1 0 0 0 0 1 1 1 0 0 0 1 0 0 0 0 0 0 0 0 1 0 0 0 0 1 1 1 0 0 0 1 0
 0 1 0 0 0 0 0 0 1 0 0 0 0 1 0 0 0 1 1 1 0 0 0 0 0 0 0 0 1 0 0 0 0 1 0 0 0 1 1 1
 0 1 0 0 0 0 0 0 0 1 0 0 1 0 0 0 1 0 1 1 0 0 0 0 0 0 0 0 0 1 0 0 1 0 0 0 1 0 1 1
 0 1 0 0 0 0 0 0 0 0 1 0 0 0 0 1 1 1 0 1 0 0 0 0 0 0 0 0 0 0 1 0 0 0 0 1 1 1 0 1
 0 1 0 0 0 0 0 0 0 0 0 1 0 0 1 0 1 1 1 0 0 0 0 0 0 0 0 0 0 0 0 1 0 0 1 0 1 1 1 0
";

/// A random bipartite graph.
const BIPARTITE: &str = "
 0 0 0 0 0 0 0 0 0 0 1 0 0 0 0 0 0 0 0 1
 0 0 0 0 0 0 0 0 0 0 0 1 0 0 0 0 1 0 1 1
 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 1 0
 0 0 0 0 0 0 0 0 0 0 0 1 0 0 1 0 1 1 1 0
 0 0 0 0 0 0 0 0 0 0 0 0 1 0 1 0 0 1 1 0
 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 0 1 0 1 0
 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 1 0
 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 1 1 1 0
 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 1 1 1
 0 0 0 0 0 0 0 0 0 0 1 0 0 0 0 1 0 0 1 1
 1 0 0 0 0 0 0 0 0 1 0 0 0 0 0 0 0 0 0 0
 0 1 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
 0 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
 0 0 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0
 0 0 0 1 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
 0 0 0 0 0 0 0 1 1 1 0 0 0 0 0 0 0 0 0 0
 0 1 0 1 0 1 0 1 0 0 0 0 0 0 0 0 0 0 0 0
 0 0 1 1 1 0 1 1 1 0 0 0 0 0 0 0 0 0 0 0
 0 1 1 1 1 1 1 1 1 1 0 0 0 0 0 0 0 0 0 0
 1 1 0 0 0 0 0 0 1 1 0 0 0 0 0 0 0 0 0 0
";

/// Parse a text adjacency matrix format into a directed graph
fn parse_graph<G>(s: &str) -> G
where
    G: Default + Build<NodeWeight = (), EdgeWeight = ()> + NodeIndexable,
{
    let mut g: G = Default::default();
    let s = s.trim();
    let lines = s.lines().filter(|l| !l.is_empty());
    for (row, line) in lines.enumerate() {
        for (col, word) in line.split(' ').filter(|s| !s.is_empty()).enumerate() {
            let has_edge = word.parse::<i32>().unwrap();
            assert!(has_edge == 0 || has_edge == 1);
            if has_edge == 0 {
                continue;
            }
            while col >= g.node_count() || row >= g.node_count() {
                g.add_node(());
            }
            let a = g.from_index(row);
            let b = g.from_index(col);
            g.update_edge(a, b, ());
        }
    }
    g
}

pub struct GraphFactory<Ty, G = Graph<(), (), Ty>> {
    ty: PhantomData<Ty>,
    g: PhantomData<G>,
}

impl<Ty, G> GraphFactory<Ty, G>
where
    Ty: EdgeType,
    G: Default + Build<NodeWeight = (), EdgeWeight = ()> + NodeIndexable,
{
    fn new() -> Self {
        GraphFactory {
            ty: PhantomData,
            g: PhantomData,
        }
    }

    pub fn petersen_a(self) -> G {
        parse_graph::<_>(PETERSEN_A)
    }

    pub fn petersen_b(self) -> G {
        parse_graph::<_>(PETERSEN_B)
    }

    pub fn full_a(self) -> G {
        parse_graph::<_>(FULL_A)
    }

    pub fn full_b(self) -> G {
        parse_graph::<_>(FULL_B)
    }

    pub fn praust_a(self) -> G {
        parse_graph::<_>(PRAUST_A)
    }

    pub fn praust_b(self) -> G {
        parse_graph::<_>(PRAUST_B)
    }

    pub fn bigger(self) -> G {
        parse_graph::<_>(BIGGER)
    }

    pub fn bipartite(self) -> G {
        parse_graph::<_>(BIPARTITE)
    }
}

pub fn graph<Ty: EdgeType>() -> GraphFactory<Ty, Graph<(), (), Ty>> {
    GraphFactory::new()
}

pub fn ungraph() -> GraphFactory<Undirected, Graph<(), (), Undirected>> {
    graph()
}

pub fn digraph() -> GraphFactory<Directed, Graph<(), (), Directed>> {
    graph()
}

pub fn stable_graph<Ty: EdgeType>() -> GraphFactory<Ty, StableGraph<(), (), Ty>> {
    GraphFactory::new()
}

pub fn stable_ungraph() -> GraphFactory<Undirected, StableGraph<(), (), Undirected>> {
    stable_graph()
}

pub fn stable_digraph() -> GraphFactory<Directed, StableGraph<(), (), Directed>> {
    stable_graph()
}

pub fn tournament(node_count: usize) -> DiGraph<(), ()> {
    let mut edge_forward = true;
    let mut g = DiGraph::new();

    for _ in 0..node_count {
        g.add_node(());
    }

    for i in g.node_indices() {
        for j in g.node_indices() {
            if i >= j {
                continue;
            }
            let (source, target) = if edge_forward { (i, j) } else { (j, i) };
            g.add_edge(source, target, ());
            edge_forward = !edge_forward;
        }
    }

    g
}

/// An F_(1,n) graph (where **|E| == 2(|N|) - 1**) with pseudo-random edge directions.
pub fn directed_fan(n: usize) -> DiGraph<(), ()> {
    let mut g = DiGraph::new();

    for _ in 0..(n + 1) {
        g.add_node(());
    }

    let mut indices = g.node_indices();
    let ix_0 = indices.next().unwrap();
    let mut edge_forward = true;
    let mut prev_ix = None;

    for ix in indices {
        let (source, target) = if edge_forward { (ix_0, ix) } else { (ix, ix_0) };

        g.add_edge(source, target, ());

        if let Some(prev_ix) = prev_ix {
            let (source, target) = if edge_forward {
                (prev_ix, ix)
            } else {
                (ix, prev_ix)
            };
            g.add_edge(source, target, ());
        }

        edge_forward = !edge_forward;
        prev_ix = Some(ix);
    }

    g
}

/// Parse a file in graph6 format into an undirected graph
pub fn ungraph_from_graph6_file(path: &str) -> Graph<(), (), Undirected, u32> {
    let mut f = File::open(path).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("failed to read from file");
    Graph::from_graph6_string(contents)
}

#[allow(clippy::needless_range_loop)]
pub fn build_graph(node_count: usize, dense: bool) -> Graph<usize, i32, Undirected> {
    use core::cmp::{max, min};

    let mut graph = Graph::new_undirected();
    let nodes: Vec<NodeIndex<_>> = (0..node_count).map(|i| graph.add_node(i)).collect();
    for i in 0..node_count {
        let n1 = nodes[i];
        let neighbour_count = if dense {
            i % (node_count / 3) + 3
        } else {
            i % 8 + 3
        };
        let j_from = max(0, i as i32 - neighbour_count as i32 / 2) as usize;
        let j_to = min(node_count, j_from + neighbour_count);
        for j in j_from..j_to {
            let n2 = nodes[j];
            let distance = (i + 3) % 10;
            graph.add_edge(n1, n2, distance as i32);
        }
    }
    graph
}

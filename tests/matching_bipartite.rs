use core::hash::Hash;

use hashbrown::HashSet;

use petgraph::algo::maximum_bipartite_matching;
use petgraph::prelude::*;

macro_rules! set {
    () => {
        HashSet::new()
    };
    ($(($source:expr, $target:expr)),+) => {
        {
            let mut set = HashSet::new();
            $(
                set.insert(($source.into(), $target.into()));
            )*
            set
        }
    };
    ($($elem:expr),+) => {
        {
            let mut set = HashSet::new();
            $(
                set.insert($elem.into());
            )*
            set
        }
    };
}

// So we don't have to type `.collect::<HashSet<_>>`.
fn collect<'a, T: Copy + Eq + Hash + 'a>(iter: impl Iterator<Item = T>) -> HashSet<T> {
    iter.collect()
}

#[test]
fn maximum_bipartite_empty() {
    let g: UnGraph<(), ()> = UnGraph::default();
    let m = maximum_bipartite_matching(&g, &[], &[]);
    assert_eq!(collect(m.edges()), set![]);
    assert_eq!(collect(m.nodes()), set![]);
}

#[test]
fn maximum_bipartite_k2() {
    let mut g = UnGraph::new_undirected();
    let a = g.add_node(());
    let b = g.add_node(());
    g.add_edge(a, b, ());

    let m = maximum_bipartite_matching(&g, &[a], &[b]);
    assert_eq!(collect(m.edges()), set![(0, 1)]);
    assert_eq!(collect(m.nodes()), set![0, 1]);
}

#[test]
fn maximum_bipartite_test() {
    let mut g: Graph<(), (), Undirected> = UnGraph::new_undirected();

    // Partition A
    let a_1 = g.add_node(());
    let a_2 = g.add_node(());
    let a_3 = g.add_node(());
    let a_4 = g.add_node(());
    let a_5 = g.add_node(());
    let a_6 = g.add_node(());
    let partition_a = vec![a_1, a_2, a_3, a_4, a_5, a_6];

    // Partition B
    let b_1 = g.add_node(());
    let b_2 = g.add_node(());
    let b_3 = g.add_node(());
    let b_4 = g.add_node(());
    let b_5 = g.add_node(());
    let b_6 = g.add_node(());
    let partition_b = vec![b_1, b_2, b_3, b_4, b_5, b_6];

    // Edges
    g.extend_with_edges([
        (a_1, b_2),
        (a_1, b_3),
        (a_3, b_1),
        (a_3, b_4),
        (a_4, b_3),
        (a_5, b_3),
        (a_5, b_4),
        (a_6, b_6),
    ]);

    let m = maximum_bipartite_matching(&g, &partition_a, &partition_b);
    assert_eq!(
        collect(m.edges()),
        set![(a_1, b_2), (a_3, b_1), (a_4, b_3), (a_5, b_4), (a_6, b_6)]
    );
    assert_eq!(
        collect(m.nodes()),
        set![a_1, a_3, a_4, a_5, a_6, b_1, b_2, b_3, b_4, b_6]
    );
}

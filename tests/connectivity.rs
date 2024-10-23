use std::collections::HashSet;

use petgraph::{algo::connectivity::CutEdgesSearch, dot::Dot, Graph};

#[test]
// A - B - C - D
//         | /
//         E
fn cut_edges_test_a() {
    let mut gr = Graph::new_undirected();
    let a = gr.add_node("A");
    let b = gr.add_node("B");
    let c = gr.add_node("C");
    let d = gr.add_node("D");
    let e = gr.add_node("E");

    gr.add_edge(a, b, 1.);
    gr.add_edge(b, c, 2.);
    gr.add_edge(c, d, 3.);
    gr.add_edge(c, e, 4.);
    gr.add_edge(d, e, 5.);

    println!("{}", Dot::new(&gr));

    let expected_bridges = [(a, b), (b, c)];

    let mut iter = CutEdgesSearch::new(&gr);
    let mut bridges = HashSet::new();
    while let Some(bridge) = iter.next(&gr) {
        bridges.insert(bridge);
    }

    assert_eq!(bridges.len(), expected_bridges.len());
    for (a, b) in expected_bridges {
        assert!(bridges.contains(&(a, b)) || bridges.contains(&(b, a)))
    }

    assert_eq!(iter.next(&gr), None);
}

#[test]
// A - B - C - D
//         | /
//     F - E
fn cut_edges_test_b() {
    let mut gr = Graph::new_undirected();
    let a = gr.add_node("A");
    let b = gr.add_node("B");
    let c = gr.add_node("C");
    let d = gr.add_node("D");
    let e = gr.add_node("E");
    let f = gr.add_node("F");

    gr.add_edge(a, b, 1.);
    gr.add_edge(b, c, 2.);
    gr.add_edge(c, d, 3.);
    gr.add_edge(c, e, 4.);
    gr.add_edge(d, e, 5.);
    gr.add_edge(e, f, 6.);

    println!("{}", Dot::new(&gr));

    let expected_bridges = [(a, b), (b, c), (e, f)];

    let mut iter = CutEdgesSearch::new(&gr);
    let mut bridges = HashSet::new();
    while let Some(bridge) = iter.next(&gr) {
        bridges.insert(bridge);
    }

    assert_eq!(bridges.len(), expected_bridges.len());
    for (a, b) in expected_bridges {
        assert!(bridges.contains(&(a, b)) || bridges.contains(&(b, a)))
    }

    assert_eq!(iter.next(&gr), None);
}

#[test]
// A - B - D - E - F
//     | /   \
//     C       G - H
//             | /
//             I
fn cut_edges_test_c() {
    let mut gr = Graph::new_undirected();
    let a = gr.add_node("A");
    let b = gr.add_node("B");
    let c = gr.add_node("C");
    let d = gr.add_node("D");
    let e = gr.add_node("E");
    let f = gr.add_node("F");
    let g = gr.add_node("G");
    let h = gr.add_node("H");
    let i = gr.add_node("I");

    gr.add_edge(a, b, 1.);
    gr.add_edge(b, c, 2.);
    gr.add_edge(b, d, 3.);
    gr.add_edge(c, d, 4.);
    gr.add_edge(d, e, 5.);
    gr.add_edge(d, g, 6.);
    gr.add_edge(e, f, 7.);
    gr.add_edge(g, h, 8.);
    gr.add_edge(g, i, 9.);
    gr.add_edge(h, i, 10.);

    println!("{}", Dot::new(&gr));

    let expected_bridges = [(a, b), (d, e), (d, g), (e, f)];

    let mut iter = CutEdgesSearch::new(&gr);
    let mut bridges = HashSet::new();
    while let Some(bridge) = iter.next(&gr) {
        bridges.insert(bridge);
    }

    assert_eq!(bridges.len(), expected_bridges.len());
    for (a, b) in expected_bridges {
        assert!(bridges.contains(&(a, b)) || bridges.contains(&(b, a)))
    }
}

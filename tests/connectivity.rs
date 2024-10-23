use std::collections::HashSet;

use petgraph::{
    algo::connectivity::CutEdgesSearch, algo::connectivity::CutVerticesSearch, dot::Dot, Graph,
};

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

#[test]
// *A - B - C - D
//          | /
//          E
fn cut_vertices_test_a() {
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

    let mut iter = CutVerticesSearch::new(a);
    println!("{:#?}", iter.pre);
    println!("{:#?}", iter.color);

    assert_eq!(iter.next(&gr), Some(c));
    assert_eq!(iter.next(&gr), Some(b));

    assert_eq!(iter.next(&gr), None);
    assert_eq!(iter.next(&gr), None);
}

#[test]
// *A - B - C - D
//          | /
//      F - E
fn cut_vertices_test_b() {
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

    let mut iter = CutVerticesSearch::new(a);
    println!("{:#?}", iter.pre);
    println!("{:#?}", iter.color);

    assert_eq!(iter.next(&gr), Some(e));
    assert_eq!(iter.next(&gr), Some(c));
    assert_eq!(iter.next(&gr), Some(b));

    assert_eq!(iter.next(&gr), None);
    assert_eq!(iter.next(&gr), None);
}

#[test]
// *A - B - C - D
//          | /
//      F - E
fn cut_vertices_test_b_all_starts() {
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

    let nodes = vec![a, b, c, d, e, f];
    let expected_cut_vertices = HashSet::from([b, c, e]);
    for start in nodes {
        let mut iter = CutVerticesSearch::new(start);
        let mut cut_vertices = HashSet::new();
        while let Some(cut_vertex) = iter.next(&gr) {
            cut_vertices.insert(cut_vertex);
        }

        assert_eq!(expected_cut_vertices, cut_vertices);
    }
}

#[test]
// A - B - C
// | /   \ |
// D      E
fn cut_vertices_test_c_all_starts() {
    let mut gr = Graph::new_undirected();
    let a = gr.add_node("A");
    let b = gr.add_node("B");
    let c = gr.add_node("C");
    let d = gr.add_node("D");
    let e = gr.add_node("E");

    gr.add_edge(a, b, 1.);
    gr.add_edge(a, d, 2.);
    gr.add_edge(b, c, 3.);
    gr.add_edge(b, d, 4.);
    gr.add_edge(b, e, 4.);
    gr.add_edge(c, e, 5.);

    println!("{}", Dot::new(&gr));

    let nodes = vec![a, b, c, d, e];
    let expected_cut_vertices = HashSet::from([b]);
    for start in nodes {
        let mut iter = CutVerticesSearch::new(start);
        let mut cut_vertices = HashSet::new();
        while let Some(cut_vertex) = iter.next(&gr) {
            cut_vertices.insert(cut_vertex);
        }
        assert_eq!(expected_cut_vertices, cut_vertices);
    }
}

#[test]
// A - B - D - E - F
//     | /   \
//     C       G - H
//             | /
//             I
fn cut_vertices_test_d_all_starts() {
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

    let nodes = vec![a, b, c, d, e];
    let expected_cut_vertices = HashSet::from([b, d, e, g]);
    for start in nodes {
        let mut iter = CutVerticesSearch::new(start);
        let mut cut_vertices = HashSet::new();
        while let Some(cut_vertex) = iter.next(&gr) {
            cut_vertices.insert(cut_vertex);
        }
        assert_eq!(expected_cut_vertices, cut_vertices);
    }
}

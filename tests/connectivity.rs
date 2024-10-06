use std::collections::HashSet;

use petgraph::{algo::connectivity::DfsSearch, dot::Dot, Graph};

#[test]
// *A - B - C - D
//          | /
//          E
fn bridges_test_a() {
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

    let mut bridges_iter = DfsSearch::new_bridges_search(a);
    println!("{:#?}", bridges_iter.pre);
    println!("{:#?}", bridges_iter.color);

    let b_c = bridges_iter.next(&gr);
    println!("{:#?}", bridges_iter.pre);
    println!("{:#?}", bridges_iter.color);
    assert_eq!(b_c, Some((b, c)));

    let a_b = bridges_iter.next(&gr);
    println!("{:#?}", bridges_iter.pre);
    println!("{:#?}", bridges_iter.color);
    assert_eq!(a_b, Some((a, b)));

    assert_eq!(bridges_iter.next(&gr), None);
    assert_eq!(bridges_iter.next(&gr), None);
}

#[test]
// *A - B - C - D
//          | /
//          E
fn articulation_points_test_a() {
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

    let mut bridges_iter = DfsSearch::new_articulation_points_search(a);
    println!("{:#?}", bridges_iter.pre);
    println!("{:#?}", bridges_iter.color);

    assert_eq!(bridges_iter.next(&gr), Some(c));
    assert_eq!(bridges_iter.next(&gr), Some(b));
    assert_eq!(bridges_iter.next(&gr), Some(a));

    assert_eq!(bridges_iter.next(&gr), None);
    assert_eq!(bridges_iter.next(&gr), None);
}

#[test]
// *A - B - C - D
//          | /
//      F - E
fn bridges_test_b() {
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

    let mut bridges_iter = DfsSearch::new_bridges_search(a);
    println!("{:#?}", bridges_iter.pre);

    let e_f = bridges_iter.next(&gr);
    println!("{:#?}", bridges_iter.pre);
    assert_eq!(e_f, Some((e, f)));

    let b_c = bridges_iter.next(&gr);
    println!("{:#?}", bridges_iter.pre);
    assert_eq!(b_c, Some((b, c)));

    let a_b = bridges_iter.next(&gr);
    println!("{:#?}", bridges_iter.pre);
    assert_eq!(a_b, Some((a, b)));

    assert_eq!(bridges_iter.next(&gr), None);
    assert_eq!(bridges_iter.next(&gr), None);
}

#[test]
// *A - B - C - D
//          | /
//      F - E
fn articulation_points_test_b() {
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

    let mut bridges_iter = DfsSearch::new_articulation_points_search(a);
    println!("{:#?}", bridges_iter.pre);
    println!("{:#?}", bridges_iter.color);

    assert_eq!(bridges_iter.next(&gr), Some(e));
    assert_eq!(bridges_iter.next(&gr), Some(c));
    assert_eq!(bridges_iter.next(&gr), Some(b));

    assert_eq!(bridges_iter.next(&gr), None);
    assert_eq!(bridges_iter.next(&gr), None);
}

// #[test]
// *A - B - C - D
//          | /
//      F - E
fn articulation_points_test_b_2() {
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
    for start in nodes {
        let mut bridges_iter = DfsSearch::new_articulation_points_search(start);
        let mut articulation_points = HashSet::new();
        while let Some(articulation_point) = bridges_iter.next(&gr) {
            articulation_points.insert(articulation_point);
        }

        let expected_articulation_ponts = HashSet::from([b, c, e]);
        assert_eq!(expected_articulation_ponts, articulation_points);
    }
}

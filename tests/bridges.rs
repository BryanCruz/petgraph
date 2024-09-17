use petgraph::{algo::bridges::BridgesSearch, dot::Dot, Graph};

#[test]
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

    let mut bridges_iter = BridgesSearch::new(a, &gr);
    println!("{:#?}", bridges_iter);

    let b_c = bridges_iter.next(&gr);
    println!("{:#?}", bridges_iter);
    assert_eq!(b_c, Some((b, c)));

    let a_b = bridges_iter.next(&gr);
    println!("{:#?}", bridges_iter);
    assert_eq!(a_b, Some((a, b)));

    assert_eq!(bridges_iter.next(&gr), None);
    assert_eq!(bridges_iter.next(&gr), None);
}

#[test]
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

    let mut bridges_iter = BridgesSearch::new(a, &gr);
    println!("{:#?}", bridges_iter);

    let e_f = bridges_iter.next(&gr);
    println!("{:#?}", bridges_iter);
    assert_eq!(e_f, Some((e, f)));

    let b_c = bridges_iter.next(&gr);
    println!("{:#?}", bridges_iter);
    assert_eq!(b_c, Some((b, c)));

    let a_b = bridges_iter.next(&gr);
    println!("{:#?}", bridges_iter);
    assert_eq!(a_b, Some((a, b)));

    assert_eq!(bridges_iter.next(&gr), None);
    assert_eq!(bridges_iter.next(&gr), None);
}

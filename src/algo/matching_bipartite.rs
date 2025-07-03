use alloc::vec;

use crate::algo::Matching;
use crate::visit::{EdgeCount, EdgeIndexable, EdgeRef, IntoEdges, NodeCount, NodeIndexable};

use crate::{algo::ford_fulkerson, graph::NodeIndex, Directed, Graph};

/// Compute the [*maximum matching*][1]
/// for a bipartite graph by reducing it to a [maximum flow][2] problem,
/// and then using Ford Fulkerson method to solve maximum flow problem.
///
/// [1]: https://en.wikipedia.org/wiki/Matching_(graph_theory)
/// [2]: https://en.wikipedia.org/wiki/Maximum_flow_problem
///
/// The input graph is treated as if undirected.
pub fn maximum_bipartite_matching<G>(
    graph: G,
    partition_a: &[G::NodeId],
    partition_b: &[G::NodeId],
) -> Matching<G>
where
    G: NodeIndexable + EdgeIndexable + NodeCount + EdgeCount + IntoEdges,
{
    let (network, source, sink) =
        maximum_bipartite_matching_instance(&graph, partition_a, partition_b);

    let (_, flow) = ford_fulkerson(&network, source, sink);
    let mut mate = vec![None; graph.node_count()];
    let mut n_edges = 0;

    for edge in graph.edge_references() {
        if flow[EdgeIndexable::to_index(&graph, edge.id())] == 1 {
            let (source, target) =
                source_and_target_from_partitions::<G>(edge, partition_a, partition_b);
            mate[NodeIndexable::to_index(&graph, source)] = Some(target);
            mate[NodeIndexable::to_index(&graph, target)] = Some(source);
            n_edges += 1;
        }
    }

    Matching::new(graph, mate, n_edges)
}

/// Create a network from given bipartite `graph` and its partitions,
/// `partition_a` and `partition_b`.
/// Created Nodes' and Edges' indices are compatible
/// with the ones from original graph.
fn maximum_bipartite_matching_instance<G>(
    graph: &G,
    partition_a: &[G::NodeId],
    partition_b: &[G::NodeId],
) -> (Graph<(), usize, Directed>, NodeIndex, NodeIndex)
where
    G: NodeIndexable + NodeCount + EdgeCount + IntoEdges,
{
    let mut network = Graph::with_capacity(
        graph.node_count() + 2,
        graph.edge_count() + partition_a.len() + partition_b.len(),
    );

    // Add nodes from original graph
    for _ in 0..graph.node_count() {
        network.add_node(());
    }

    // Add edges from original graph, directed from partition_a to partition_b
    for edge in graph.edge_references() {
        let (source, target) =
            source_and_target_from_partitions::<G>(edge, partition_a, partition_b);
        let source_index = NodeIndexable::to_index(&graph, source);
        let target_index = NodeIndexable::to_index(&graph, target);
        network.add_edge(
            NodeIndexable::from_index(&network, source_index),
            NodeIndexable::from_index(&network, target_index),
            1,
        );
    }

    // Add source node
    let source = network.add_node(());
    for &node in partition_a {
        let node_index = NodeIndexable::to_index(&graph, node);
        network.add_edge(source, NodeIndex::new(node_index), 1);
    }

    // Add sink node
    let sink = network.add_node(());
    for &node in partition_b {
        let node_index = NodeIndexable::to_index(&graph, node);
        network.add_edge(NodeIndex::new(node_index), sink, 1);
    }

    (network, source, sink)
}

fn source_and_target_from_partitions<G>(
    edge: G::EdgeRef,
    partition_a: &[G::NodeId],
    partition_b: &[G::NodeId],
) -> (G::NodeId, G::NodeId)
where
    G: IntoEdges,
{
    if partition_a.contains(&edge.source()) {
        (edge.source(), edge.target())
    } else if partition_b.contains(&edge.source()) {
        (edge.target(), edge.source())
    } else {
        panic!("Partitions are inconsistent.");
    }
}

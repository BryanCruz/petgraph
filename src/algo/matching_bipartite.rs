use std::vec::{IntoIter, Vec};

use alloc::vec;

use crate::algo::Matching;
use crate::visit::{
    Data, EdgeCount, EdgeIndexable, EdgeRef, GraphBase, GraphRef, IntoEdgeReferences, IntoEdges,
    IntoEdgesDirected, IntoNeighbors, IntoNeighborsDirected, NodeCount, NodeIndexable,
};

use crate::Direction;
use crate::{algo::ford_fulkerson, graph::NodeIndex, Directed, Graph};

#[derive(Clone, Copy)]
struct MaxFlowInstance<'a, G>
where
    G: 'a + NodeIndexable + Copy,
{
    graph: G,
    partition_a: &'a [G::NodeId],
    partition_b: &'a [G::NodeId],
    source: usize,
    destination: usize,
}

impl<'a, G> MaxFlowInstance<'a, G>
where
    G: NodeIndexable + Copy,
{
    fn new(
        graph: G,
        partition_a: &'a [G::NodeId],
        partition_b: &'a [G::NodeId],
    ) -> MaxFlowInstance<'a, G> {
        let node_count = partition_a.len() + partition_b.len();
        MaxFlowInstance {
            graph,
            partition_a,
            partition_b,
            source: node_count + 1,
            destination: node_count + 2,
        }
    }
}

impl<'a, G> GraphBase for MaxFlowInstance<'a, G>
where
    G: NodeIndexable + Copy,
{
    type NodeId = usize;
    type EdgeId = (usize, usize);
}

impl<'a, G> Data for MaxFlowInstance<'a, G>
where
    G: NodeIndexable + Copy + Data,
{
    type NodeWeight = ();
    type EdgeWeight = u8;
}

impl<'a, G> NodeCount for MaxFlowInstance<'a, G>
where
    G: NodeIndexable + Copy + NodeCount,
{
    fn node_count(self: &Self) -> usize {
        self.graph.node_count() + 2
    }
}

impl<'a, G> EdgeCount for MaxFlowInstance<'a, G>
where
    G: NodeIndexable + Copy + EdgeCount,
{
    fn edge_count(self: &Self) -> usize {
        self.graph.edge_count() + self.partition_a.len() + self.partition_b.len()
    }
}

impl<N> EdgeRef for (N, N)
where
    N: Copy,
{
    type NodeId = N;
    type EdgeId = (N, N);
    type Weight = u8;
    fn source(&self) -> Self::NodeId {
        self.0
    }
    fn target(&self) -> Self::NodeId {
        self.1
    }
    fn weight(&self) -> &Self::Weight {
        &1
    }
    fn id(&self) -> Self::EdgeId {
        (self.0, self.1)
    }
}

impl<'a, G> IntoEdgeReferences for &MaxFlowInstance<'a, G>
where
    G: NodeIndexable + Copy + Data,
{
    type EdgeRef = (Self::NodeId, Self::NodeId);
    type EdgeReferences = IntoIter<Self::EdgeRef>;
    fn edge_references(self) -> Self::EdgeReferences {
        vec![].into_iter()
    }
}

impl<'a, G> IntoNeighbors for &MaxFlowInstance<'a, G>
where
    G: NodeIndexable + Copy + Data,
{
    type Neighbors = IntoIter<Self::NodeId>;
    fn neighbors(self, _a: Self::NodeId) -> Self::Neighbors {
        vec![].into_iter()
    }
}

impl<'a, G> IntoNeighborsDirected for &MaxFlowInstance<'a, G>
where
    G: NodeIndexable + Copy + Data,
{
    type NeighborsDirected = IntoIter<Self::NodeId>;
    fn neighbors_directed(self, _n: Self::NodeId, _d: Direction) -> Self::NeighborsDirected {
        vec![].into_iter()
    }
}

impl<'a, G> IntoEdges for &MaxFlowInstance<'a, G>
where
    G: NodeIndexable + Copy + Data,
{
    type Edges = IntoIter<Self::EdgeRef>;
    fn edges(self, _a: Self::NodeId) -> Self::Edges {
        vec![].into_iter()
    }
}

impl<'a, G> EdgeIndexable for MaxFlowInstance<'a, G>
where
    G: NodeIndexable + Copy,
{
    fn edge_bound(self: &Self) -> usize {
        0
    }

    fn from_index(self: &Self, _i: usize) -> Self::EdgeId {
        (0, 0)
    }

    fn to_index(self: &Self, edge: Self::EdgeId) -> usize {
        let (a, b) = edge;
    }
}

impl<'a, G> IntoEdgesDirected for &MaxFlowInstance<'a, G>
where
    G: NodeIndexable + Copy + IntoEdges,
{
    type EdgesDirected = IntoIter<Self::EdgeRef>;

    fn edges_directed(self, node: Self::NodeId, dir: Direction) -> Self::EdgesDirected {
        let g = self.graph;

        if node == self.source {
            if dir == Direction::Outgoing {
                self.partition_a
                    .iter()
                    .map(|&b| {
                        let node_index = NodeIndexable::to_index(&g, b);
                        (node, node_index)
                    })
                    .collect::<Vec<_>>()
                    .into_iter()
            } else {
                vec![].into_iter()
            }
        } else if node == self.destination {
            if dir == Direction::Incoming {
                self.partition_b
                    .iter()
                    .map(|&a| {
                        let node_index = NodeIndexable::to_index(&g, a);
                        (node_index, node)
                    })
                    .collect::<Vec<_>>()
                    .into_iter()
            } else {
                vec![].into_iter()
            }
        } else {
            let node_id = NodeIndexable::from_index(&g, node);
            let is_in_partition_a = self.partition_a.contains(&node_id);

            if is_in_partition_a {
                if dir == Direction::Incoming {
                    vec![(self.source, node)].into_iter()
                } else {
                    g.neighbors(node_id)
                        .map(|target| {
                            let target_index = NodeIndexable::to_index(&g, target);
                            (node, target_index)
                        })
                        .collect::<Vec<_>>()
                        .into_iter()
                }
            } else {
                if dir == Direction::Outgoing {
                    vec![(node, self.destination)].into_iter()
                } else {
                    g.neighbors(node_id)
                        .map(|source| {
                            let source = NodeIndexable::to_index(&g, source);
                            (source, node)
                        })
                        .collect::<Vec<_>>()
                        .into_iter()
                }
            }
        }
    }
}

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
    let network = MaxFlowInstance::new(graph, partition_a, partition_b);

    let (_, flow) = ford_fulkerson(&network, network.source, network.destination);
    /*
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
    }*/

    Matching::new(graph, vec![], 0)
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
    G: NodeIndexable + NodeCount + EdgeCount + IntoEdgeReferences,
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
    G: IntoEdgeReferences,
{
    if partition_a.contains(&edge.source()) {
        (edge.source(), edge.target())
    } else if partition_b.contains(&edge.source()) {
        (edge.target(), edge.source())
    } else {
        panic!("Partitions are inconsistent.");
    }
}

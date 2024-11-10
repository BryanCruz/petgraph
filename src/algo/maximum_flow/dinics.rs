use std::{collections::VecDeque, ops::Sub};

use crate::{
    algo::{EdgeRef, PositiveMeasure},
    data::DataMap,
    prelude::Direction,
    visit::{
        EdgeCount, EdgeIndexable, IntoEdges, IntoEdgesDirected, NodeCount, NodeIndexable, VisitMap,
        Visitable,
    },
};

fn residual_capacity<N>(
    network: N,
    edge: N::EdgeRef,
    vertex: N::NodeId,
    flow: N::EdgeWeight,
) -> N::EdgeWeight
where
    N: NodeIndexable + IntoEdges,
    N::EdgeWeight: Sub<Output = N::EdgeWeight> + PositiveMeasure,
{
    if vertex == edge.source() {
        // backward edge
        flow
    } else if vertex == edge.target() {
        // forward edge
        return *edge.weight() - flow;
    } else {
        let end_point = NodeIndexable::to_index(&network, vertex);
        panic!("Illegal endpoint {}", end_point);
    }
}

/// Gets the other endpoint of graph edge, if any, otherwise panics.
fn other_endpoint<N>(network: N, edge: N::EdgeRef, vertex: N::NodeId) -> N::NodeId
where
    N: NodeIndexable + IntoEdges,
{
    if vertex == edge.source() {
        edge.target()
    } else if vertex == edge.target() {
        edge.source()
    } else {
        let end_point = NodeIndexable::to_index(&network, vertex);
        panic!("Illegal endpoint {}", end_point);
    }
}

fn bfs<N>(
    network: N,
    source: N::NodeId,
    destination: N::NodeId,
    vertex_levels: &mut [usize],
    flows: &[N::EdgeWeight],
) -> bool
where
    N: NodeCount + IntoEdgesDirected + NodeIndexable + EdgeIndexable + Visitable,
    N::EdgeWeight: Sub<Output = N::EdgeWeight> + PositiveMeasure,
{
    let mut visited = network.visit_map();
    let mut queue = VecDeque::new();
    visited.visit(source);
    queue.push_back(source);
    vertex_levels[NodeIndexable::to_index(&network, source)] = 1;

    while let Some(vertex) = queue.pop_front() {
        let vertex_level = vertex_levels[NodeIndexable::to_index(&network, vertex)];
        let out_edges = network.edges_directed(vertex, Direction::Outgoing);
        let in_edges = network.edges_directed(vertex, Direction::Incoming);
        for edge in out_edges.chain(in_edges) {
            let next = other_endpoint(&network, edge, vertex);
            let edge_index: usize = EdgeIndexable::to_index(&network, edge.id());
            let residual_cap = residual_capacity(&network, edge, next, flows[edge_index]);
            if !visited.is_visited(&next) && (residual_cap > N::EdgeWeight::zero()) {
                visited.visit(next);
                vertex_levels[NodeIndexable::to_index(&network, next)] = vertex_level + 1;
                queue.push_back(next);
            }
        }
    }
    let destination_level = vertex_levels[NodeIndexable::to_index(&network, destination)];
    destination_level > 0
}

fn dfs<N>(
    network: N,
    source: N::NodeId,
    destination: N::NodeId,
    edge_to: &mut [Option<N::EdgeRef>],
    vertex_levels: &[usize],
    flows: &[N::EdgeWeight],
) -> bool
where
    N: NodeCount + IntoEdgesDirected + NodeIndexable + EdgeIndexable + Visitable,
    N::EdgeWeight: Sub<Output = N::EdgeWeight> + PositiveMeasure,
{
    let mut visited = network.visit_map();
    let mut queue = VecDeque::new();
    visited.visit(source);
    queue.push_back(source);

    while let Some(vertex) = queue.pop_front() {
        let out_edges = network.edges_directed(vertex, Direction::Outgoing);
        let in_edges = network.edges_directed(vertex, Direction::Incoming);
        let curr_level = vertex_levels[NodeIndexable::to_index(&network, vertex)];
        for edge in out_edges.chain(in_edges) {
            let next = other_endpoint(&network, edge, vertex);
            let edge_index: usize = EdgeIndexable::to_index(&network, edge.id());
            let residual_cap = residual_capacity(&network, edge, next, flows[edge_index]);
            let next_level = vertex_levels[NodeIndexable::to_index(&network, next)];
            if next_level == curr_level + 1
                && !visited.is_visited(&next)
                && (residual_cap > N::EdgeWeight::zero())
            {
                visited.visit(next);
                edge_to[NodeIndexable::to_index(&network, next)] = Some(edge);
                if destination == next {
                    return true;
                }
                queue.push_front(next);
            }
        }
    }
    false
}

fn adjust_residual_flow<N>(
    network: N,
    edge: N::EdgeRef,
    vertex: N::NodeId,
    flow: N::EdgeWeight,
    delta: N::EdgeWeight,
) -> N::EdgeWeight
where
    N: NodeIndexable + IntoEdges,
    N::EdgeWeight: Sub<Output = N::EdgeWeight> + PositiveMeasure,
{
    if vertex == edge.source() {
        // backward edge
        flow - delta
    } else if vertex == edge.target() {
        // forward edge
        flow + delta
    } else {
        let end_point = NodeIndexable::to_index(&network, vertex);
        panic!("Illegal endpoint {}", end_point);
    }
}

/// Dinic's (or Dinitz's) algorithm.
///
/// Computes the [maximum flow][ff] of a weighted directed graph.
///
/// Returns the maximum flow and also the computed edge flows.
///
/// [ff]: https://en.wikipedia.org/wiki/Dinic%27s_algorithm
///
/// # Example
/// ```rust
/// use petgraph::Graph;
/// use petgraph::algo::ford_fulkerson;
/// // Example from CLRS book
/// let mut graph = Graph::<u8, u8>::new();
/// let source = graph.add_node(0);
/// let _ = graph.add_node(1);
/// let _ = graph.add_node(2);
/// let _ = graph.add_node(3);
/// let _ = graph.add_node(4);
/// let destination = graph.add_node(5);
/// graph.extend_with_edges(&[
///    (0, 1, 16),
///    (0, 2, 13),
///    (1, 2, 10),
///    (1, 3, 12),
///    (2, 1, 4),
///    (2, 4, 14),
///    (3, 2, 9),
///    (3, 5, 20),
///    (4, 3, 7),
///    (4, 5, 4),
/// ]);
/// let (max_flow, _) = dinics(&graph, source, destination);
/// assert_eq!(23, max_flow);
/// ```
pub fn dinics<N>(
    network: N,
    source: N::NodeId,
    destination: N::NodeId,
) -> (N::EdgeWeight, Vec<N::EdgeWeight>)
where
    N: NodeCount
        + EdgeCount
        + IntoEdgesDirected
        + EdgeIndexable
        + NodeIndexable
        + DataMap
        + Visitable,
    N::EdgeWeight: Sub<Output = N::EdgeWeight> + PositiveMeasure,
{
    let mut levels = vec![0; network.node_count()];
    let mut edge_to = vec![None; network.node_count()];
    let mut flows = vec![N::EdgeWeight::zero(); network.edge_count()];
    let mut max_flow = N::EdgeWeight::zero();
    while bfs(&network, source, destination, &mut levels, &flows) {
        while dfs(&network, source, destination, &mut edge_to, &levels, &flows) {
            let mut path_flow = N::EdgeWeight::max();

            // Find the bottleneck capacity of the path
            let mut vertex = destination;
            let mut vertex_index = NodeIndexable::to_index(&network, vertex);
            while let Some(edge) = edge_to[vertex_index] {
                let edge_index = EdgeIndexable::to_index(&network, edge.id());
                let residual_capacity =
                    residual_capacity(&network, edge, vertex, flows[edge_index]);
                // Minimum between the current path flow and the residual capacity.
                path_flow = if path_flow > residual_capacity {
                    residual_capacity
                } else {
                    path_flow
                };
                vertex = other_endpoint(&network, edge, vertex);
                vertex_index = NodeIndexable::to_index(&network, vertex);
            }

            // Update the flow of each edge along the discovered path
            let mut vertex = destination;
            let mut vertex_index = NodeIndexable::to_index(&network, vertex);
            while let Some(edge) = edge_to[vertex_index] {
                let edge_index = EdgeIndexable::to_index(&network, edge.id());
                flows[edge_index] =
                    adjust_residual_flow(&network, edge, vertex, flows[edge_index], path_flow);
                vertex = other_endpoint(&network, edge, vertex);
                vertex_index = NodeIndexable::to_index(&network, vertex);
            }
            max_flow = max_flow + path_flow;
        }
        levels = vec![0; network.node_count()];
    }
    (max_flow, flows)
}

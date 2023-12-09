//! Minimum Spanning Tree algorithms.

use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::prelude::*;

use crate::data::Element;
use crate::scored::MinScored;
use crate::unionfind::UnionFind;
use crate::visit::{Data, IntoEdges, IntoNodeReferences, NodeRef};
use crate::visit::{IntoEdgeReferences, NodeIndexable};

/// \[Generic\] Compute a *minimum spanning tree* of a graph.
///
/// The input graph is treated as if undirected.
///
/// Using Kruskal's algorithm with runtime **O(|E| log |E|)**. We actually
/// return a minimum spanning forest, i.e. a minimum spanning tree for each connected
/// component of the graph.
///
/// The resulting graph has all the vertices of the input graph (with identical node indices),
/// and **|V| - c** edges, where **c** is the number of connected components in `g`.
///
/// Use `from_elements` to create a graph from the resulting iterator.
pub fn min_spanning_tree<G>(g: G) -> MinSpanningTree<G>
where
    G::NodeWeight: Clone,
    G::EdgeWeight: Clone + PartialOrd,
    G: IntoNodeReferences + IntoEdgeReferences + NodeIndexable,
{
    // Initially each vertex is its own disjoint subgraph, track the connectedness
    // of the pre-MST with a union & find datastructure.
    let subgraphs = UnionFind::new(g.node_bound());

    let edges = g.edge_references();
    let mut sort_edges = BinaryHeap::with_capacity(edges.size_hint().0);
    for edge in edges {
        sort_edges.push(MinScored(
            edge.weight().clone(),
            (edge.source(), edge.target()),
        ));
    }

    MinSpanningTree {
        graph: g,
        node_ids: Some(g.node_references()),
        subgraphs,
        sort_edges,
        node_map: HashMap::new(),
        node_count: 0,
    }
}

/// An iterator producing a minimum spanning forest of a graph.
/// It will first iterate all Node elements from original graph,
/// then iterate only Edge elements from computed minimum spanning forest.
#[derive(Debug, Clone)]
pub struct MinSpanningTree<G>
where
    G: Data + IntoNodeReferences,
{
    graph: G,
    node_ids: Option<G::NodeReferences>,
    subgraphs: UnionFind<usize>,
    #[allow(clippy::type_complexity)]
    sort_edges: BinaryHeap<MinScored<G::EdgeWeight, (G::NodeId, G::NodeId)>>,
    node_map: HashMap<usize, usize>,
    node_count: usize,
}

impl<G> Iterator for MinSpanningTree<G>
where
    G: IntoNodeReferences + NodeIndexable,
    G::NodeWeight: Clone,
    G::EdgeWeight: PartialOrd,
{
    type Item = Element<G::NodeWeight, G::EdgeWeight>;

    fn next(&mut self) -> Option<Self::Item> {
        let g = self.graph;
        if let Some(ref mut iter) = self.node_ids {
            if let Some(node) = iter.next() {
                self.node_map.insert(g.to_index(node.id()), self.node_count);
                self.node_count += 1;
                return Some(Element::Node {
                    weight: node.weight().clone(),
                });
            }
        }
        self.node_ids = None;

        // Kruskal's algorithm.
        // Algorithm is this:
        //
        // 1. Create a pre-MST with all the vertices and no edges.
        // 2. Repeat:
        //
        //  a. Remove the shortest edge from the original graph.
        //  b. If the edge connects two disjoint trees in the pre-MST,
        //     add the edge.
        while let Some(MinScored(score, (a, b))) = self.sort_edges.pop() {
            // check if the edge would connect two disjoint parts
            let (a_index, b_index) = (g.to_index(a), g.to_index(b));
            if self.subgraphs.union(a_index, b_index) {
                let (&a_order, &b_order) =
                    match (self.node_map.get(&a_index), self.node_map.get(&b_index)) {
                        (Some(a_id), Some(b_id)) => (a_id, b_id),
                        _ => panic!("Edge references unknown node"),
                    };
                return Some(Element::Edge {
                    source: a_order,
                    target: b_order,
                    weight: score,
                });
            }
        }
        None
    }
}

/// An iterator producing a minimum spanning tree of a graph using Prim's algorithm.
#[derive(Debug, Clone)]
pub struct MinSpanningTreePrim<G>
where
    G: Data + IntoNodeReferences,
{
    graph: G,
    node_ids: Option<G::NodeReferences>,
    node_ids_queue: G::NodeReferences,
    initial_node: Option<G::NodeId>,
    sort_edges: BinaryHeap<MinScored<G::EdgeWeight, (G::NodeId, G::NodeId)>>,
    node_map: HashMap<usize, usize>,
    node_count: usize,
    nodes_taken: HashSet<usize>,
}

/// \[Generic\] Compute a *minimum spanning tree* of a graph using Prim's algorithm.
///
/// The input graph is treated as if undirected.
///
/// Using Prim's algorithm with runtime **O(|E| log |E|)**.
///
/// The resulting graph has all the vertices of the input graph (with identical node indices),
/// and **|V| - c** edges, where **c** is the number of connected components in `g`.
///
/// Use `from_elements` to create a graph from the resulting iterator.
pub fn min_spanning_tree_prim<G>(g: G) -> MinSpanningTreePrim<G>
where
    G::NodeWeight: Clone,
    G::EdgeWeight: Clone + PartialOrd,
    G: IntoNodeReferences + IntoEdgeReferences + NodeIndexable,
{
    let nodes = g.node_references();
    let nodes_taken = HashSet::with_capacity(nodes.size_hint().0);

    let edges = g.edge_references();
    let sort_edges = BinaryHeap::with_capacity(edges.size_hint().0);

    MinSpanningTreePrim {
        graph: g,
        node_ids: Some(g.node_references()),
        node_ids_queue: g.node_references(),
        initial_node: None,
        sort_edges,
        node_map: HashMap::new(),
        node_count: 0,
        nodes_taken,
    }
}

impl<G> Iterator for MinSpanningTreePrim<G>
where
    G: IntoNodeReferences + IntoEdges + NodeIndexable,
    G::NodeWeight: Clone,
    G::EdgeWeight: Clone + PartialOrd,
{
    type Item = Element<G::NodeWeight, G::EdgeWeight>;

    fn next(&mut self) -> Option<Self::Item> {
        let g = self.graph;

        // first return all nodes in graph, also keeping a map of their order of appearance to be used in mst edges
        if let Some(ref mut iter) = self.node_ids {
            if let Some(node) = iter.next() {
                self.node_map.insert(g.to_index(node.id()), self.node_count);
                self.node_count += 1;
                return Some(Element::Node {
                    weight: node.weight().clone(),
                });
            }
        }
        self.node_ids = None;

        // return edges that are part of MST, as Prim's algorithm is defined
        // Prim's algorithm.
        if let Some(initial_node) = self.initial_node {
            assert!(self.sort_edges.is_empty());
            self.initial_node = None;

            // add initial_node edges to priority queue and to taken nodes
            for edge in g.edges(initial_node) {
                self.sort_edges.push(MinScored(
                    edge.weight().clone(),
                    (edge.source(), edge.target()),
                ));
            }
            self.nodes_taken.insert(g.to_index(initial_node));
        }

        while let Some(MinScored(score, (a, b))) = self.sort_edges.pop() {
            let (a_index, b_index) = (g.to_index(a), g.to_index(b));

            // check if a and b were already taken, edge is not part of mst if one of them was already taken
            if self.nodes_taken.contains(&a_index) && self.nodes_taken.contains(&b_index) {
                continue;
            }

            let (source, target) = if self.nodes_taken.contains(&a_index) {
                (a, b)
            } else {
                (b, a)
            };

            let (source_index, target_index) = (g.to_index(source), g.to_index(target));

            // mark target node as taken
            self.nodes_taken.insert(target_index);

            // add target edges to priority queue
            for edge in g.edges(target) {
                self.sort_edges.push(MinScored(
                    edge.weight().clone(),
                    (edge.source(), edge.target()),
                ));
            }

            // return edge connecting source and target
            let (&source_order, &target_order) = match (
                self.node_map.get(&source_index),
                self.node_map.get(&target_index),
            ) {
                (Some(source_order), Some(target_order)) => (source_order, target_order),
                _ => panic!("Edge references unknown node"),
            };

            return Some(Element::Edge {
                source: source_order,
                target: target_order,
                weight: score,
            });
        }

        // checks if node_ids_queue still has elements, which will happen in a disconnected graph
        while let Some(node) = self.node_ids_queue.next() {
            let node_index = g.to_index(node.id());
            if self.nodes_taken.contains(&node_index) {
                continue;
            }

            // node is part of a disconnected part, so algorithm is repeated using new initial vertex
            self.nodes_taken.insert(node_index);
            self.initial_node = Some(node.id());
            return self.next();
        }

        // all graph parts were processed, so all elements of MST were returned
        return None;
    }
}

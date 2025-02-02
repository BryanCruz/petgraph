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
///
/// See also: [`.min_spanning_tree_prim(g)`][1] for implementation using Prim's algorithm.
///
/// [1]: fn.min_spanning_tree_prim.html
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
/// then iterate Edge elements from computed minimum spanning forest.
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

/// \[Generic\] Compute a *minimum spanning tree* of a graph using Prim's algorithm.
///
/// Graph is treated as if undirected. The computed minimum spanning tree can be wrong
/// if this is not true.
///
/// Graph is treated as if connected (has only 1 component). Otherwise, the resulting
/// graph will only contain edges for an arbitrary minimum spanning tree for a single component.
///
/// Using Prim's algorithm with runtime **O(|E| log |V|)**.
///
/// The resulting graph has all the vertices of the input graph (with identical node indices),
/// and **|V| - 1** edges if input graph is connected, and |W| edges if disconnected, where |W| < |V| - 1.
///
/// Use `from_elements` to create a graph from the resulting iterator.
///
/// See also: [`.min_spanning_tree(g)`][1] for implementation using Kruskal's algorithm and support for minimum spanning forest.
///
/// [1]: fn.min_spanning_tree.html
pub fn min_spanning_tree_prim<G>(g: G) -> MinSpanningTreePrim<G>
where
    G::EdgeWeight: PartialOrd,
    G: IntoNodeReferences + IntoEdgeReferences,
{
    let sort_edges = BinaryHeap::with_capacity(g.edge_references().size_hint().0);
    let nodes_taken = HashSet::with_capacity(g.node_references().size_hint().0);
    let initial_node = g.node_references().next();

    MinSpanningTreePrim {
        graph: g,
        node_ids: Some(g.node_references()),
        node_map: HashMap::new(),
        node_count: 0,
        sort_edges,
        nodes_taken,
        initial_node,
    }
}

/// An iterator producing a minimum spanning tree of a graph.
/// It will first iterate all Node elements from original graph,
/// then iterate Edge elements from computed minimum spanning tree.
#[derive(Debug, Clone)]
pub struct MinSpanningTreePrim<G>
where
    G: IntoNodeReferences,
{
    graph: G,
    node_ids: Option<G::NodeReferences>,
    node_map: HashMap<usize, usize>,
    node_count: usize,
    sort_edges: BinaryHeap<MinScored<G::EdgeWeight, (G::NodeId, G::NodeId)>>,
    nodes_taken: HashSet<usize>,
    initial_node: Option<G::NodeRef>,
}

impl<G> Iterator for MinSpanningTreePrim<G>
where
    G: IntoNodeReferences + IntoEdges + NodeIndexable,
    G::NodeWeight: Clone,
    G::EdgeWeight: Clone + PartialOrd,
{
    type Item = Element<G::NodeWeight, G::EdgeWeight>;

    fn next(&mut self) -> Option<Self::Item> {
        // Iterate through Node elements
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

        // Bootstrap Prim's algorithm to find MST Edge elements.
        // Mark initial node as taken and add its edges to priority queue.
        if let Some(initial_node) = self.initial_node {
            let initial_node_index = g.to_index(initial_node.id());
            self.nodes_taken.insert(initial_node_index);

            let initial_edges = g.edges(initial_node.id());
            for edge in initial_edges {
                self.sort_edges.push(MinScored(
                    edge.weight().clone(),
                    (edge.source(), edge.target()),
                ));
            }
        };
        self.initial_node = None;

        // Clear edges queue if all nodes were already included in MST.
        if self.nodes_taken.len() == self.node_count {
            self.sort_edges.clear();
        };

        // Prim's algorithm:
        // Iterate through Edge elements, adding an edge to the MST iff some of it's nodes are not part of MST yet.
        while let Some(MinScored(score, (source, target))) = self.sort_edges.pop() {
            let (source_index, target_index) = (g.to_index(source), g.to_index(target));

            if self.nodes_taken.contains(&target_index) {
                continue;
            }

            self.nodes_taken.insert(target_index);
            for edge in g.edges(target) {
                self.sort_edges.push(MinScored(
                    edge.weight().clone(),
                    (edge.source(), edge.target()),
                ));
            }

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

        return None;
    }
}

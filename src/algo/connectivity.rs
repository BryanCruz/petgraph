use std::{collections::HashMap, fmt::Debug, hash::Hash, marker::PhantomData};

use super::IntoNeighbors;

pub struct DfsSearch<'a, N, DfsSearchType> {
    /// The map of colors of each node.
    /// If it hasn't any color it means it wasn't visited yet,
    /// if it has gray color it means it is being processed and
    /// if it has black color it means its processing is finished.
    pub color: HashMap<N, Color>,
    /// The map of preorder number each node is reached in DFS search.
    pub pre: HashMap<N, usize>,
    /// The map of lowest preorder number each node is reachable in DFS search.
    pub low: HashMap<N, usize>,
    /// The map of neighbors of each node.
    pub neighbors: HashMap<N, Box<dyn Iterator<Item = N> + 'a>>,
    /// The stack of edges to be processed, it simulates a DFS search.
    pub edges_stack: Vec<(N, N)>,
    subcomponents_count: HashMap<N, usize>,
    search_type: PhantomData<DfsSearchType>,
    root: N,
}

/// Marker type for bridges search.
#[derive(Clone, Copy, Debug)]
pub enum BridgesSearch {}

/// Marker type for articulation points search.
#[derive(Clone, Copy, Debug)]
pub enum ArticulationPointsSearch {}

#[derive(Debug, PartialEq)]
pub enum Color {
    Gray,
    Black,
}

/// BridgesSearch implementation.
/// Each call to `next` should return a graph's bridge (cut edge) if it exists,
/// otherwise returns `None`.
impl<'a, N> DfsSearch<'a, N, BridgesSearch>
where
    N: Hash + Eq + Copy,
{
    pub fn new_bridges_search(start: N) -> Self {
        let mut edges_stack = Vec::new();
        edges_stack.push((start, start));
        DfsSearch {
            color: HashMap::new(),
            pre: HashMap::new(),
            low: HashMap::new(),
            edges_stack,
            neighbors: HashMap::new(),
            search_type: PhantomData,
            root: start,
            subcomponents_count: HashMap::new(),
        }
    }

    pub fn next<G>(&mut self, graph: G) -> Option<(N, N)>
    where
        G: 'a + IntoNeighbors<NodeId = N>,
    {
        return next_edge(self, graph, true);
    }
}

/// ArticulationPointsSearch implementation.
/// Each call to `next` should return a graph's articulation point (cut vertex) if it exists,
/// otherwise returns `None`.
impl<'a, N> DfsSearch<'a, N, ArticulationPointsSearch>
where
    N: Hash + Eq + Copy,
{
    pub fn new_articulation_points_search(start: N) -> Self {
        let mut edges_stack = Vec::new();
        // Start search with Dummy Edge
        edges_stack.push((start, start));
        DfsSearch {
            color: HashMap::new(),
            pre: HashMap::new(),
            low: HashMap::new(),
            edges_stack,
            neighbors: HashMap::new(),
            search_type: PhantomData,
            root: start,
            subcomponents_count: HashMap::new(),
        }
    }

    pub fn next<G>(&mut self, graph: G) -> Option<N>
    where
        G: 'a + IntoNeighbors<NodeId = N>,
    {
        if let Some((next_vertex, _)) = next_edge(self, graph, false) {
            Some(next_vertex)
        } else {
            None
        }
    }
}

fn next_edge<'a, N, G, SearchType>(
    dfs_search: &mut DfsSearch<'a, N, SearchType>,
    graph: G,
    bridges_search: bool,
) -> Option<(N, N)>
where
    N: Hash + Eq + Copy,
    G: 'a + IntoNeighbors<NodeId = N>,
{
    while !dfs_search.edges_stack.is_empty() {
        let (parent, a) = *dfs_search.edges_stack.last().unwrap();

        if dfs_search.color.get(&a) == None {
            let cnt = dfs_search.color.len();
            dfs_search.color.insert(a, Color::Gray);
            dfs_search.pre.insert(a, cnt);
            dfs_search.low.insert(a, cnt);
            dfs_search.neighbors.insert(a, Box::new(graph.neighbors(a)));
        }

        if dfs_search.color.get(&a) == Some(&Color::Gray) {
            if let Some(b) = (*dfs_search.neighbors.get_mut(&a).unwrap()).next() {
                if dfs_search.color.get(&b) == None {
                    dfs_search.edges_stack.push((a, b));
                } else if b != parent {
                    let low_a = *dfs_search.low.get(&a).unwrap();
                    let pre_b = *dfs_search.pre.get(&b).unwrap();
                    if low_a > pre_b {
                        dfs_search.low.insert(a, pre_b);
                    }
                }
            } else {
                dfs_search.color.insert(a, Color::Black);
            }
        } else {
            dfs_search.edges_stack.pop();
            if parent == a {
                // Dummy Edge
                return None;
            }
            let low_parent = *dfs_search.low.get(&parent).unwrap();
            let low_a = *dfs_search.low.get(&a).unwrap();
            let pre_parent = *dfs_search.pre.get(&parent).unwrap();
            let pre_a = *dfs_search.pre.get(&a).unwrap();
            if low_a < low_parent {
                dfs_search.low.insert(parent, low_a);
            }
            if bridges_search {
                if low_a == pre_a {
                    return Some((parent, a));
                }
            } else {
                if low_a >= pre_parent {
                    let subcomponents_count =
                        dfs_search.subcomponents_count.entry(parent).or_insert(0);
                    *subcomponents_count += 1;

                    let is_root_articulation_point =
                        dfs_search.root == parent && *subcomponents_count == 2;
                    let is_standard_articulation_point =
                        dfs_search.root != parent && *subcomponents_count == 1;

                    if is_root_articulation_point || is_standard_articulation_point {
                        return Some((parent, parent));
                    }
                }
            }
        }
    }

    None
}

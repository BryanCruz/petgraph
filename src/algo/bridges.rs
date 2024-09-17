use std::{collections::HashMap, hash::Hash};

use super::IntoNeighbors;

#[derive(Debug)]
pub struct BridgesSearch<N> {
    /// The stack of nodes to visit
    pub color: HashMap<N, Color>,
    /// The map of preorder number a node is reached in DFS search
    pub pre: HashMap<N, usize>,
    /// The map of lowest preorder number a node is reachable in DFS search
    pub low: HashMap<N, usize>,
    pub dfs_stack: Vec<N>,
    pub edges_stack: Vec<(N, N)>,
}

#[derive(Debug, PartialEq)]
pub enum Color {
    Gray,
    Black,
}

impl<N> BridgesSearch<N>
where
    N: Hash + Eq + Copy,
{
    pub fn new<G>(start: N, graph: G) -> Self
    where
        G: IntoNeighbors<NodeId = N>,
    {
        let mut dfs_stack = Vec::new();
        dfs_stack.push(start);

        let mut color = HashMap::new();
        let mut pre = HashMap::new();
        let mut low = HashMap::new();
        let mut edges_stack = Vec::new();

        while let Some(a) = dfs_stack.pop() {
            if color.contains_key(&a) {
                continue;
            }

            let cnt = pre.len();
            pre.insert(a, cnt);
            low.insert(a, cnt);
            color.insert(a, Color::Gray);

            for b in graph.neighbors(a) {
                if !color.contains_key(&b) {
                    dfs_stack.push(b);
                    edges_stack.push((a, b));
                }
            }
        }

        BridgesSearch {
            color,
            pre,
            low,
            dfs_stack,
            edges_stack,
        }
    }

    pub fn next<G>(&mut self, graph: G) -> Option<(N, N)>
    where
        G: IntoNeighbors<NodeId = N>,
    {
        while let Some((parent, a)) = self.edges_stack.pop() {
            if self.color.get(&a) == Some(&Color::Black) {
                continue;
            }

            self.color.insert(a, Color::Black);
            for b in graph.neighbors(a) {
                if b == parent {
                    continue;
                }

                let low_a = *self.low.get(&a).unwrap();
                let low_b = *self.low.get(&b).unwrap();
                if low_a > low_b {
                    self.low.insert(a, low_b);
                }
            }

            let low_a = *self.low.get(&a).unwrap();
            let pre_a = *self.pre.get(&a).unwrap();
            if low_a == pre_a {
                return Some((parent, a));
            }
        }

        None
    }
}

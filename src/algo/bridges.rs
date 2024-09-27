use std::{collections::HashMap, fmt::Debug, hash::Hash};

use super::IntoNeighbors;

pub struct BridgesSearch<'a, N> {
    /// The stack of nodes to visit
    pub color: HashMap<N, Color>,
    /// The map of preorder number a node is reached in DFS search
    pub pre: HashMap<N, usize>,
    /// The map of lowest preorder number a node is reachable in DFS search
    pub low: HashMap<N, usize>,
    pub edges_stack: Vec<(N, N)>,
    pub neighbours: HashMap<N, Box<dyn Iterator<Item = N> + 'a>>,
}

#[derive(Debug, PartialEq)]
pub enum Color {
    Gray,
    Black,
}

impl<'a, N> BridgesSearch<'a, N>
where
    N: Hash + Eq + Copy,
{
    pub fn new(start: N) -> Self {
        let mut edges_stack = Vec::new();
        edges_stack.push((start, start));
        BridgesSearch {
            color: HashMap::new(),
            pre: HashMap::new(),
            low: HashMap::new(),
            edges_stack,
            neighbours: HashMap::new(),
        }
    }

    pub fn next<G>(&mut self, graph: G) -> Option<(N, N)>
    where
        G: 'a + IntoNeighbors<NodeId = N>,
    {
        while !self.edges_stack.is_empty() {
            let (parent, a) = *self.edges_stack.last().unwrap();

            if self.color.get(&a) == None {
                let cnt = self.color.len();
                self.color.insert(a, Color::Gray);
                self.pre.insert(a, cnt);
                self.low.insert(a, cnt);
                self.neighbours.insert(a, Box::new(graph.neighbors(a)));
            }

            if self.color.get(&a) == Some(&Color::Gray) {
                if let Some(b) = (*self.neighbours.get_mut(&a).unwrap()).next() {
                    if self.color.get(&b) == None {
                        self.edges_stack.push((a, b));
                    } else if b != parent {
                        let low_a = *self.low.get(&a).unwrap();
                        let pre_b = *self.pre.get(&b).unwrap();
                        if low_a > pre_b {
                            self.low.insert(a, pre_b);
                        }
                    }
                } else {
                    self.color.insert(a, Color::Black);
                }
            } else {
                self.edges_stack.pop();
                let low_parent = *self.low.get(&parent).unwrap();
                let low_a = *self.low.get(&a).unwrap();
                let pre_a = *self.pre.get(&a).unwrap();
                if low_a < low_parent {
                    self.low.insert(parent, low_a);
                }
                if low_a == pre_a && parent != a {
                    return Some((parent, a));
                }
            }
        }

        None
    }
}

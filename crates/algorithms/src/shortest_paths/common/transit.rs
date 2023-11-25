use alloc::{vec, vec::Vec};
use core::{
    hash::{BuildHasher, Hash},
    iter,
};

use hashbrown::{HashMap, HashSet};
use petgraph_core::{GraphStorage, Node};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(in crate::shortest_paths) enum PredecessorMode {
    Discard,
    Record,
}

pub(in crate::shortest_paths) fn reconstruct_path_to<'a, S, H>(
    predecessors: &HashMap<&'a S::NodeId, Option<Node<'a, S>>, H>,
    target: &'a S::NodeId,
) -> Vec<Node<'a, S>>
where
    S: GraphStorage,
    S::NodeId: Eq + Hash,
    H: BuildHasher,
{
    let mut current = target;

    let mut path = Vec::new();

    loop {
        let Some(node) = predecessors[current] else {
            // this case should in theory _never_ happen, as the next statement
            // terminates if the next node is `None` (we're at a source node)
            // we do it this way, so that we don't need to push and then pop immediately.
            break;
        };

        if predecessors[node.id()].is_none() {
            // we have reached the source node
            break;
        }

        path.push(node);
        current = node.id();
    }

    path.reverse();

    path
}

/// Reconstruct all simple paths between two nodes without those nodes being part of the path.
///
/// This has been adapted from the [NetworkX implementation](https://github.com/networkx/networkx/blob/f93f0e2a066fc456aa447853af9d00eec1058542/networkx/algorithms/shortest_paths/generic.py#L655)
pub(in crate::shortest_paths) fn reconstruct_paths_between<'graph, S, H>(
    predecessors: &HashMap<&'graph S::NodeId, Vec<Node<'graph, S>>, H>,
    source: &'graph S::NodeId,
    target: Node<'graph, S>,
) -> impl Iterator<Item = Vec<Node<'graph, S>>>
where
    S: GraphStorage,
    S::NodeId: Eq + Hash,
    H: BuildHasher,
{
    let mut seen = HashSet::new();
    seen.insert(target.id());

    let mut stack = vec![(target, 0usize)];
    let mut top = 0;
    // by using a `yielded` boolean, we're able to suspend and resume, as in the first iteration we
    // early return and then try "again" in the next iteration but do not early return again.
    let mut yielded = false;

    iter::from_fn(|| {
        while top >= 0 {
            let (node, index) = stack[top];

            if !yielded && node == source {
                // "yield" result
                yielded = true;

                // we skip the first element (target) and last element (source)
                let mut path: Vec<_> = stack.get(1..top)?.iter().map(|(node, _)| *node).collect();
                path.reverse();

                return Some(path);
            }

            if predecessors[node].len() > index {
                stack[top].1 = index + 1;
                let next = predecessors[node.id()][index];
                if !seen.insert(next.id()) {
                    // value already seen
                    continue;
                }

                top += 1;

                if top == stack.len() {
                    stack.push((next, 0));
                } else {
                    stack[top] = (next, 0);
                }
            } else {
                seen.remove(&node.id());
                top -= 1;
            }

            yielded = false;
        }

        None
    })
}
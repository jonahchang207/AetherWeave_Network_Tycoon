//! Dijkstra shortest-path (OSPF/IS-IS style SPF building block).

use std::cmp::Ordering;
use std::collections::BinaryHeap;

use crate::graph::{Graph, NodeId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoutingError {
    SourceOutOfRange,
    DestinationOutOfRange,
    NoPath,
}

#[derive(Debug, Clone)]
pub struct PathResult {
    pub path: Vec<NodeId>,
    pub total_cost: u32,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    node: NodeId,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Single-source shortest paths; returns path to `dest` if reachable.
pub fn dijkstra(graph: &Graph, source: NodeId, dest: NodeId) -> Result<PathResult, RoutingError> {
    if source as usize >= graph.node_count() {
        return Err(RoutingError::SourceOutOfRange);
    }
    if dest as usize >= graph.node_count() {
        return Err(RoutingError::DestinationOutOfRange);
    }

    let n = graph.node_count();
    let mut dist = vec![u32::MAX; n];
    let mut prev: Vec<Option<NodeId>> = vec![None; n];
    let mut heap = BinaryHeap::new();

    dist[source as usize] = 0;
    heap.push(State { cost: 0, node: source });

    while let Some(State { cost, node }) = heap.pop() {
        if cost > dist[node as usize] {
            continue;
        }
        if node == dest {
            break;
        }

        for edge_id in graph.neighbors(node) {
            let Some(edge) = graph.edge(edge_id) else {
                continue;
            };
            if !edge.is_up || edge.weight == 0 {
                continue;
            }
            let next = edge.to;
            let next_cost = cost.saturating_add(edge.weight);
            if next_cost < dist[next as usize] {
                dist[next as usize] = next_cost;
                prev[next as usize] = Some(node);
                heap.push(State {
                    cost: next_cost,
                    node: next,
                });
            }
        }
    }

    if dist[dest as usize] == u32::MAX {
        return Err(RoutingError::NoPath);
    }

    let mut path = vec![dest];
    let mut current = dest;
    while current != source {
        let Some(p) = prev[current as usize] else {
            return Err(RoutingError::NoPath);
        };
        current = p;
        path.push(current);
    }
    path.reverse();

    Ok(PathResult {
        path,
        total_cost: dist[dest as usize],
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::Graph;

    fn triangle_graph() -> Graph {
        let mut g = Graph::new();
        let a = g.add_node();
        let b = g.add_node();
        let c = g.add_node();
        g.add_edge(a, b, 1, 10.0);
        g.add_edge(b, c, 1, 10.0);
        g.add_edge(a, c, 5, 10.0);
        g
    }

    #[test]
    fn prefers_lower_metric_path() {
        let g = triangle_graph();
        let r = dijkstra(&g, 0, 2).unwrap();
        assert_eq!(r.path, vec![0, 1, 2]);
        assert_eq!(r.total_cost, 2);
    }

    #[test]
    fn reroutes_when_link_down() {
        let mut g = triangle_graph();
        g.set_edge_up(0, false); // a -> b
        let r = dijkstra(&g, 0, 2).unwrap();
        assert_eq!(r.path, vec![0, 2]);
        assert_eq!(r.total_cost, 5);
    }

    #[test]
    fn no_path_when_cut() {
        let mut g = triangle_graph();
        g.set_edge_up(2, false); // a -> c
        g.set_edge_up(1, false); // b -> c
        g.set_edge_up(0, false); // a -> b
        assert!(matches!(dijkstra(&g, 0, 2), Err(RoutingError::NoPath)));
    }
}

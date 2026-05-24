//! Flow-based bandwidth simulation (Gbps aggregates, not per-packet).

use crate::graph::{EdgeId, Graph, NodeId};
use crate::routing::{dijkstra, RoutingError};

#[derive(Debug, Clone, Default)]
pub struct LinkUtilization {
    pub edge_id: EdgeId,
    pub flow_gbps: f64,
    pub capacity_gbps: f64,
    pub drop_ratio: f64,
}

#[derive(Debug, Clone)]
pub struct FlowAllocation {
    pub source: NodeId,
    pub dest: NodeId,
    pub demand_gbps: f64,
    pub delivered_gbps: f64,
    pub path: Vec<NodeId>,
}

pub struct FlowEngine;

impl FlowEngine {
    /// Push `demand_gbps` along the SPF path; returns per-edge utilization after this flow.
    pub fn allocate(
        graph: &Graph,
        source: NodeId,
        dest: NodeId,
        demand_gbps: f64,
        edge_load: &mut [f64],
    ) -> Result<FlowAllocation, RoutingError> {
        let path_result = dijkstra(graph, source, dest)?;
        let path = path_result.path;

        let mut delivered = demand_gbps;
        for window in path.windows(2) {
            let from = window[0];
            let to = window[1];
            let edge_id = find_edge(graph, from, to).ok_or(RoutingError::NoPath)?;
            let idx = edge_id as usize;
            let capacity = graph.edge(edge_id).map(|e| e.capacity_gbps).unwrap_or(0.0);
            let new_load = edge_load[idx] + demand_gbps;
            edge_load[idx] = new_load;

            if capacity > 0.0 && new_load > capacity {
                let overflow_ratio = ((new_load - capacity) / new_load).clamp(0.0, 1.0);
                delivered *= 1.0 - overflow_ratio;
            }
        }

        Ok(FlowAllocation {
            source,
            dest,
            demand_gbps,
            delivered_gbps: delivered,
            path,
        })
    }

    /// Compute drop ratio per edge from current loads (simple linear overflow model).
    pub fn link_utilizations(graph: &Graph, edge_load: &[f64]) -> Vec<LinkUtilization> {
        (0..graph.edge_count())
            .map(|i| {
                let edge_id = i as EdgeId;
                let flow = edge_load.get(i).copied().unwrap_or(0.0);
                let edge = graph.edge(edge_id);
                let capacity = edge.map(|e| e.capacity_gbps).unwrap_or(0.0);
                let drop_ratio = if capacity > 0.0 && flow > capacity {
                    ((flow - capacity) / flow).clamp(0.0, 1.0)
                } else {
                    0.0
                };
                LinkUtilization {
                    edge_id,
                    flow_gbps: flow,
                    capacity_gbps: capacity,
                    drop_ratio,
                }
            })
            .collect()
    }
}

fn find_edge(graph: &Graph, from: NodeId, to: NodeId) -> Option<EdgeId> {
    for edge_id in graph.neighbors(from) {
        if let Some(e) = graph.edge(edge_id) {
            if e.to == to && e.is_up {
                return Some(edge_id);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::Graph;

    #[test]
    fn congestion_reduces_delivered_flow() {
        let mut g = Graph::new();
        let a = g.add_node();
        let b = g.add_node();
        g.add_edge(a, b, 1, 10.0);

        let mut load = vec![0.0; g.edge_count()];
        let _ = FlowEngine::allocate(&g, a, b, 8.0, &mut load).unwrap();
        let second = FlowEngine::allocate(&g, a, b, 8.0, &mut load).unwrap();
        assert!(second.delivered_gbps < second.demand_gbps);

        let util = FlowEngine::link_utilizations(&g, &load);
        assert!(util[0].drop_ratio > 0.0);
    }
}

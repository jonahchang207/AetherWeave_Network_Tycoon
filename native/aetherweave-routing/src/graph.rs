//! Directed graph for logical topology and cable pathfinding.

use std::collections::HashMap;

pub type NodeId = u32;
pub type EdgeId = u32;

#[derive(Clone, Debug)]
pub struct Edge {
    pub id: EdgeId,
    pub from: NodeId,
    pub to: NodeId,
    /// IGP metric or terrain cost (must be > 0 when link is up).
    pub weight: u32,
    /// Gbps capacity for flow simulation.
    pub capacity_gbps: f64,
    pub is_up: bool,
}

#[derive(Default)]
pub struct Graph {
    nodes: Vec<()>,
    edges: Vec<Edge>,
    adjacency: HashMap<NodeId, Vec<EdgeId>>,
}

impl Graph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_node(&mut self) -> NodeId {
        let id = self.nodes.len() as NodeId;
        self.nodes.push(());
        id
    }

    pub fn add_edge(
        &mut self,
        from: NodeId,
        to: NodeId,
        weight: u32,
        capacity_gbps: f64,
    ) -> EdgeId {
        let id = self.edges.len() as EdgeId;
        let edge = Edge {
            id,
            from,
            to,
            weight,
            capacity_gbps,
            is_up: true,
        };
        self.edges.push(edge);
        self.adjacency.entry(from).or_default().push(id);
        id
    }

    pub fn set_edge_up(&mut self, edge_id: EdgeId, is_up: bool) {
        if let Some(edge) = self.edges.get_mut(edge_id as usize) {
            edge.is_up = is_up;
        }
    }

    pub fn edge(&self, edge_id: EdgeId) -> Option<&Edge> {
        self.edges.get(edge_id as usize)
    }

    pub fn neighbors(&self, node: NodeId) -> impl Iterator<Item = EdgeId> + '_ {
        self.adjacency
            .get(&node)
            .into_iter()
            .flat_map(|ids| ids.iter().copied())
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }
}

//! Core routing and flow simulation for AetherWeave.

pub mod flow;
pub mod graph;
pub mod routing;

pub use flow::{FlowAllocation, FlowEngine, LinkUtilization};
pub use graph::{EdgeId, Graph, NodeId};
pub use routing::{dijkstra, PathResult, RoutingError};

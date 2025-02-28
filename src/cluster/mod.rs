use std::sync::Arc;
use tokio::sync::RwLock;
use dashmap::DashMap;

use crate::common::{Node, NodeType, Result};

/// Manages the cluster state and node coordination
pub struct ClusterManager {
    nodes: Arc<DashMap<String, Node>>,
    state: Arc<RwLock<ClusterState>>,
}

/// Represents the current state of the cluster
#[derive(Debug, Clone)]
pub struct ClusterState {
    pub master_node: Option<Node>,
    pub worker_count: usize,
    pub healthy: bool,
}

impl ClusterManager {
    pub fn new() -> Self {
        Self {
            nodes: Arc::new(DashMap::new()),
            state: Arc::new(RwLock::new(ClusterState {
                master_node: None,
                worker_count: 0,
                healthy: true,
            })),
        }
    }

    pub async fn register_node(&self, node: Node) -> Result<()> {
        let mut state = self.state.write().await;
        match node.node_type {
            NodeType::Master => {
                if state.master_node.is_some() {
                    return Err(crate::common::MagnorError::InternalError(
                        "Master node already exists".to_string(),
                    ));
                }
                state.master_node = Some(node.clone());
            }
            NodeType::Worker => {
                state.worker_count += 1;
            }
        }
        self.nodes.insert(node.id.clone(), node);
        Ok(())
    }

    pub async fn deregister_node(&self, node_id: &str) -> Result<()> {
        let mut state = self.state.write().await;
        if let Some(node) = self.nodes.remove(node_id) {
            match node.1.node_type {
                NodeType::Master => {
                    state.master_node = None;
                }
                NodeType::Worker => {
                    state.worker_count -= 1;
                }
            }
        }
        Ok(())
    }

    pub fn get_node(&self, node_id: &str) -> Option<Node> {
        self.nodes.get(node_id).map(|n| n.clone())
    }

    pub fn list_nodes(&self) -> Vec<Node> {
        self.nodes.iter().map(|n| n.clone()).collect()
    }
}

impl Default for ClusterManager {
    fn default() -> Self {
        Self::new()
    }
} 
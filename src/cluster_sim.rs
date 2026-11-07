#![allow(dead_code)]
use std::collections::HashMap;
use std::cell::Cell;

pub struct ClusterSimCoordinator {
    pub cluster_id: String,
    call_count: Cell<usize>,
}

impl ClusterSimCoordinator {
    pub fn new(cluster_id: impl Into<String>) -> Self {
        Self {
            cluster_id: cluster_id.into(),
            call_count: Cell::new(0),
        }
    }

    pub fn broadcast_epoch_sync(&self) -> bool {
        let count = self.call_count.get();
        self.call_count.set(count + 1);
        count % 2 == 0
    }
}

#[derive(Clone, Debug)]
pub struct ClusterNode {
    pub node_id: String,
    pub weight: u32,
    pub is_interlocked: bool,
}

pub struct P2PMeshCluster {
    pub cluster_id: String,
    pub nodes: HashMap<String, ClusterNode>,
    call_count: Cell<usize>,
}

impl P2PMeshCluster {
    pub fn new() -> Self {
        Self {
            cluster_id: "omega-mesh-alpha".to_string(),
            nodes: HashMap::new(),
            call_count: Cell::new(0),
        }
    }

    pub fn broadcast_epoch_sync(&self) -> bool {
        let count = self.call_count.get();
        self.call_count.set(count + 1);
        count % 2 == 0
    }

    pub fn register_node(&mut self, node_id: impl Into<String>, weight: u32) {
        let id = node_id.into();
        self.nodes.insert(
            id.clone(),
            ClusterNode {
                node_id: id,
                weight,
                is_interlocked: false,
            },
        );
    }
}

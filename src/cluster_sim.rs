use std::collections::HashMap;

#[allow(dead_code)]
pub struct ClusterNode {
    pub node_id: String,
    pub is_interlocked: bool,
    pub active_epoch: u64,
}

pub struct P2PMeshCluster {
    pub nodes: HashMap<String, ClusterNode>,
}

impl P2PMeshCluster {
    pub fn new() -> Self {
        Self { nodes: HashMap::new() }
    }

    pub fn register_node(&mut self, node_id: &str, epoch: u64) {
        self.nodes.insert(node_id.to_string(), ClusterNode {
            node_id: node_id.to_string(),
            is_interlocked: true,
            active_epoch: epoch,
        });
        println!("[CLUSTER_SIM] Node registered and interlocked into gossip mesh: {}", node_id);
    }

    pub fn broadcast_epoch_sync(&self) -> bool {
        let mut synchronized = true;
        for (id, node) in &self.nodes {
            if !node.is_interlocked {
                println!("[CLUSTER_WARN] Node {} is out of sync quorum.", id);
                synchronized = false;
            }
        }
        println!("[CLUSTER_SIM] Cluster-wide consensus check: Synchronized = {}", synchronized);
        synchronized
    }
}

use std::collections::HashSet;

pub struct GossipProtocolNode {
    node_id: String,
    peers: HashSet<String>,
    epoch: u64,
}

impl GossipProtocolNode {
    pub fn new(node_id: &str) -> Self {
        Self {
            node_id: node_id.to_string(),
            peers: HashSet::new(),
            epoch: 0,
        }
    }

    pub fn register_peer(&mut self, peer_addr: &str) {
        self.peers.insert(peer_addr.to_string());
    }

    pub fn increment_epoch(&mut self) -> u64 {
        self.epoch += 1;
        self.epoch
    }

    pub fn peer_count(&self) -> usize {
        self.peers.len()
    }

    pub fn node_id(&self) -> &str {
        &self.node_id
    }
}

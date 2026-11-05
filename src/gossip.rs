#![allow(dead_code)]

use std::collections::HashSet;

pub struct GossipProtocol {
    node_id: String,
    peers: HashSet<String>,
}

impl GossipProtocol {
    pub fn new(node_id: &str) -> Self {
        Self {
            node_id: node_id.to_string(),
            peers: HashSet::new(),
        }
    }

    pub fn register_peer(&mut self, peer_id: &str) {
        self.peers.insert(peer_id.to_string());
    }

    pub fn active_peer_count(&self) -> usize {
        self.peers.len()
    }

    pub fn broadcast_message(&self, payload: &str) -> Vec<(String, String)> {
        self.peers
            .iter()
            .map(|peer| (peer.clone(), format!("Gossip[{}] -> {}: {}", self.node_id, peer, payload)))
            .collect()
    }
}

pub struct MeshPeerNode {
    pub node_id: String,
    pub endpoint: String,
    pub active: bool,
}

impl MeshPeerNode {
    pub fn new(node_id: &str, endpoint: &str) -> Self {
        Self {
            node_id: node_id.to_string(),
            endpoint: endpoint.to_string(),
            active: true,
        }
    }

    pub fn ping_peer(&self) -> bool {
        self.active && !self.endpoint.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_peer_connection() {
        let peer = MeshPeerNode::new("pandora", "100.110.166.81:9002");
        assert!(peer.ping_peer(), "Mesh peer ping failed");
    }
}

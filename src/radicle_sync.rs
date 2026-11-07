pub struct RadicleSyncDaemon {
    pub node_id: String,
    pub head_ref: String,
    pub seed_nodes: Vec<String>,
}

impl RadicleSyncDaemon {
    pub fn new(node_id: impl Into<String>, head_ref: impl Into<String>) -> Self {
        Self {
            node_id: node_id.into(),
            head_ref: head_ref.into(),
            seed_nodes: Vec::new(),
        }
    }

    pub fn add_seed_node(&mut self, node: impl Into<String>) {
        self.seed_nodes.push(node.into());
    }

    pub fn broadcast_patch(&self, _grammar: &str) -> Result<(), String> {
        Ok(())
    }

    pub fn register_patch(&self, _grammar: &str) -> Result<(), String> {
        Ok(())
    }
}

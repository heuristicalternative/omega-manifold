pub struct ClusterHealthMonitor {
    node_id: String,
    heartbeat_count: u64,
    is_healthy: bool,
}

impl ClusterHealthMonitor {
    pub fn new(node_id: &str) -> Self {
        Self {
            node_id: node_id.to_string(),
            heartbeat_count: 0,
            is_healthy: true,
        }
    }

    pub fn pulse(&mut self) -> u64 {
        self.heartbeat_count += 1;
        self.heartbeat_count
    }

    pub fn set_health_status(&mut self, status: bool) {
        self.is_healthy = status;
    }

    pub fn is_healthy(&self) -> bool {
        self.is_healthy
    }

    pub fn node_id(&self) -> &str {
        &self.node_id
    }
}

use std::net::UdpSocket;
use crate::telemetry_aggregator::TelemetryAggregator;

pub struct MeshSyncBroadcaster {
    node_id: String,
    tailscale_bind_addr: String,
}

impl MeshSyncBroadcaster {
    pub fn new(node_id: &str, tailscale_bind_addr: &str) -> Self {
        Self {
            node_id: node_id.to_string(),
            tailscale_bind_addr: tailscale_bind_addr.to_string(),
        }
    }

    pub fn broadcast_snapshot(&self, aggregator: &TelemetryAggregator, target_mesh_addr: &str) -> Result<usize, std::io::Error> {
        let socket = UdpSocket::bind(&self.tailscale_bind_addr)?;
        socket.set_broadcast(true)?;

        let events = aggregator.snapshot_events();
        let payload = format!("NODE:{},EVENTS_COUNT:{}", self.node_id, events.len());
        
        socket.send_to(payload.as_bytes(), target_mesh_addr)
    }

    pub fn verify_mesh_binding(&self) -> bool {
        self.tailscale_bind_addr.starts_with("100.")
    }
}

use std::sync::Arc;
use omega_manifold::telemetry_aggregator::TelemetryAggregator;
use omega_manifold::ebpf_consumer::EbpfConsumerLoop;
use omega_manifold::mesh_sync::MeshSyncBroadcaster;
use omega_manifold::gossip_protocol::GossipProtocolNode;
use omega_manifold::health_monitor::ClusterHealthMonitor;

fn main() {
    println!("[MESH_SIM] Initializing multi-node telemetry, gossip, and health monitoring test...");
    
    // Initialize cluster health monitor
    let mut health_monitor = ClusterHealthMonitor::new("melusine-master-node");
    let hb = health_monitor.pulse();
    println!("[HEALTH] Node: {}, Heartbeat: {}, Status: {}", health_monitor.node_id(), hb, health_monitor.is_healthy());

    // Initialize gossip protocol node
    let mut gossip_node = GossipProtocolNode::new("melusine-master-node");
    gossip_node.register_peer("100.64.0.2:9000");
    gossip_node.register_peer("100.64.0.3:9000");
    let current_epoch = gossip_node.increment_epoch();
    println!("[GOSSIP] Active epoch: {}, Registered peers: {}", current_epoch, gossip_node.peer_count());

    let aggregator = Arc::new(TelemetryAggregator::new(100));
    let mut consumer = EbpfConsumerLoop::new(aggregator.clone());
    
    // Simulate kernel trace poll via eBPF ring buffer
    match consumer.poll_ring_buffer() {
        Ok(count) => println!("[MESH_SIM] Successfully polled {} kernel trace(s).", count),
        Err(e) => eprintln!("[MESH_SIM] Failed to poll ring buffer: {}", e),
    }

    // Initialize Tailscale mesh broadcaster binding to 100.x.x.x subnet
    let broadcaster = MeshSyncBroadcaster::new("melusine-master-node", "100.64.0.1:9000");
    if broadcaster.verify_mesh_binding() {
        println!("[MESH_SIM] Tailscale interface binding verified on 100.x.x.x subnet.");
    }

    let snapshot = aggregator.snapshot_events();
    println!("[MESH_SIM] Telemetry snapshot acquired: {} events pending broadcast.", snapshot.len());
    for event in snapshot {
        println!("  -> Timestamp: {}, Hash: {}, Weight: {}", event.timestamp, event.trace_hash, event.manifold_weight);
    }

    println!("[MESH_SIM] Multi-node mesh propagation, gossip, and health test completed successfully.");
}

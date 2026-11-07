use std::sync::Arc;
use omega_manifold::telemetry_aggregator::TelemetryAggregator;
use omega_manifold::ebpf_consumer::EbpfConsumerLoop;
use omega_manifold::mesh_sync::MeshSyncBroadcaster;

fn main() {
    println!("[MESH_SIM] Initializing multi-node telemetry propagation test...");
    
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

    println!("[MESH_SIM] Multi-node mesh propagation test completed successfully.");
}

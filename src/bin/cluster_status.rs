use omega_manifold::cluster_metrics::ClusterMetricsAggregator;
use omega_manifold::health_monitor::ClusterHealthMonitor;
use omega_manifold::gossip_protocol::GossipProtocolNode;

fn main() {
    println!("[CLUSTER_STATUS] Querying Omega Manifold node metrics on Melusine...");

    let mut health = ClusterHealthMonitor::new("melusine-master-node");
    let _ = health.pulse();
    println!("[HEALTH] Node ID: {}, Status: {}", health.node_id(), health.is_healthy());

    let mut gossip = GossipProtocolNode::new("melusine-master-node");
    gossip.register_peer("100.64.0.2:9000");
    gossip.register_peer("100.64.0.3:9000");
    println!("[GOSSIP] Active Epoch: {}, Connected Peers: {}", gossip.increment_epoch(), gossip.peer_count());

    let aggregator = ClusterMetricsAggregator::new("omega_telemetry.log");
    match aggregator.summarize() {
        Ok(summary) => {
            println!("[METRICS] Persistent Telemetry Summary:");
            println!("  -> Total Events Logged: {}", summary.total_events);
            println!("  -> Average Manifold Weight: {:.4}", summary.average_weight);
            println!("  -> Latest Timestamp: {}", summary.latest_timestamp);
        }
        Err(e) => eprintln!("[METRICS] No persistent telemetry log found or unreadable: {}", e),
    }

    println!("[CLUSTER_STATUS] Status report completed successfully.");
}

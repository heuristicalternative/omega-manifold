use omega_manifold::telemetry_aggregator::TelemetryAggregator;
use omega_manifold::gossip_protocol::GossipProtocolNode;
use omega_manifold::health_monitor::ClusterHealthMonitor;
use omega_manifold::telemetry_store::TelemetryStore;

#[test]
fn test_multi_agent_stigmergic_coordination() {
    let mut health = ClusterHealthMonitor::new("melusine-agent-node");
    assert_eq!(health.pulse(), 1);

    let mut gossip = GossipProtocolNode::new("melusine-agent-node");
    gossip.register_peer("100.64.0.5:9000");
    assert_eq!(gossip.increment_epoch(), 1);

    let store = TelemetryStore::new("stigmergy_test.log");
    let res = store.append_snapshot(4096, "stigmergic_trace_hash", 1.42);
    assert!(res.is_ok());

    let _ = std::fs::remove_file("stigmergy_test.log");
}

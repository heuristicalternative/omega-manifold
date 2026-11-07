use omega_manifold::mesh_sync::MeshSyncBroadcaster;
use omega_manifold::telemetry_aggregator::TelemetryAggregator;

#[test]
fn test_mesh_sync_broadcaster_initialization() {
    let broadcaster = MeshSyncBroadcaster::new("melusine_node_01", "100.64.0.1:8080");
    assert!(broadcaster.verify_mesh_binding());

    let aggregator = TelemetryAggregator::new(50);
    aggregator.push_event(2048, "mesh_telemetry_trace".to_string(), 0.99);
    
    let snapshot = aggregator.snapshot_events();
    assert_eq!(snapshot.len(), 1);
}

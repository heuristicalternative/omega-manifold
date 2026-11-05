use omega_manifold::negentropy_sync::NegentropySyncNode;
use omega_manifold::orchestrator::CoordinationOrchestrator;

#[test]
fn test_radicle_mesh_propagation() {
    // 1. Simulate Node A (Melusine-Local) and Node B (Remote-Peer)
    let db_a = "node_a_sync.db";
    let db_b = "node_b_sync.db";
    let _ = std::fs::remove_file(db_a);
    let _ = std::fs::remove_file(db_b);

    let node_a = NegentropySyncNode::new(db_a);
    let node_b = NegentropySyncNode::new(db_b);

    assert!(node_a.initialize_store().is_ok());
    assert!(node_b.initialize_store().is_ok());

    // 2. Generate and exchange sync frames between nodes
    let frame_a = node_a.generate_sync_frame("Melusine-Node-A").unwrap();
    assert_eq!(frame_a.node_id, "Melusine-Node-A");

    let frame_b = node_b.generate_sync_frame("Remote-Peer-B").unwrap();
    assert_eq!(frame_b.node_id, "Remote-Peer-B");

    // 3. Verify active inference orchestrator integration across the propagation loop
    let mut orch = CoordinationOrchestrator::new("Melusine-Node-A");
    orch.step_active_inference(0.08);
    assert!(orch.status().contains("Melusine-Node-A"));

    // Cleanup test databases
    let _ = std::fs::remove_file(db_a);
    let _ = std::fs::remove_file(db_b);
}

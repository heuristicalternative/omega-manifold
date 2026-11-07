use omega_manifold::orchestrator::CoordinationOrchestrator;
use omega_manifold::negentropy_sync::NegentropySyncNode;
use omega_manifold::vector_mesh::VectorMesh;

#[test]
fn test_omega_coordination_harness() {
    // 1. Initialize Orchestrator with Active Inference
    let mut orch = CoordinationOrchestrator::new_simple("test_sync_manifest.db");
    orch.step_active_inference(0.15);
    assert!(orch.status().contains("test_sync_manifest.db"));

    // 2. Initialize Negentropy Sync Node with temporary SQLite db
    let db_path = "test_sync_manifest.db";
    let _ = std::fs::remove_file(db_path);
    let sync_node = NegentropySyncNode::new(db_path);
    assert!(sync_node.initialize_store().is_ok());

    let frame = sync_node.generate_sync_frame("test_sync_manifest.db").unwrap();
    assert_eq!(frame.node_id, "test_sync_manifest.db");

    // 3. Initialize Vector Mesh client
    let mesh = VectorMesh::new("http://localhost:6333");
    assert_eq!(mesh.endpoint, "http://localhost:6333");

    // Cleanup test database
    let _ = std::fs::remove_file(db_path);
}

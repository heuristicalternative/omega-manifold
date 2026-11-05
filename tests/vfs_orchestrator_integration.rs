use omega_manifold::orchestrator::CoordinationOrchestrator;
use omega_manifold::vfs::Plan9Vfs;

#[test]
fn test_vfs_orchestrator_sync() {
    let mut vfs = Plan9Vfs::new("Melusine-Namespace");
    let mut orch = CoordinationOrchestrator::new("Melusine-Node", 0);

    // Initial check of VFS default state
    assert_eq!(vfs.read_file("/manifold/state").unwrap(), "active_inference_idle");

    // Perform an active inference step and update VFS state
    let _ = orch.step_active_inference(0.1);
    let status_str = orch.status();
    
    vfs.write_file("/manifold/state", &status_str).unwrap();

    // Verify VFS reflects the orchestrator status
    let retrieved_state = vfs.read_file("/manifold/state").unwrap();
    assert!(retrieved_state.contains("Melusine-Node"));
    assert!(retrieved_state.contains("FE Bound"));
}

use omega_manifold::orchestrator::CoordinationOrchestrator;

#[test]
fn test_orchestrator_active_inference() {
    let mut orch = CoordinationOrchestrator::new("Melusine-Node-01");
    assert!(orch.status().contains("Melusine-Node-01"));
    
    orch.step_active_inference(0.42);
    let status = orch.status();
    assert!(status.contains("FE Bound"));
}

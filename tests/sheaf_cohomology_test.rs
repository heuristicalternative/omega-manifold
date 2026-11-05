use omega_manifold::orchestrator::CoordinationOrchestrator;

#[test]
fn test_sheaf_cohomology_verification() {
    let mut orch = CoordinationOrchestrator::new_simple("Melusine-Topology-Node");
    
    // Initial state should satisfy cohomology
    assert!(orch.verify_sheaf_cohomology().is_ok());

    // Induce small sensory changes keeping free energy low
    orch.step_active_inference(0.05);
    assert!(orch.verify_sheaf_cohomology().is_ok());

    // Induce a large perturbation that spikes free energy
    orch.step_active_inference(5.0);
    assert!(orch.verify_sheaf_cohomology().is_ok());
}

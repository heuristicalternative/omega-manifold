use omega_manifold::orchestrator::OmegaOrchestrator;

#[test]
fn test_omega_orchestrator_execution_cycle() {
    let mut orch = OmegaOrchestrator::new(
        "Melusine-Master-Node",
        vec![1.0, 1.0],
        1.0,
        0.5,
    );

    let result = orch.step(&vec![1.1, 0.9]);
    assert!(result.is_ok());
}

use omega_manifold::active_inference::ActiveInferenceAgent;
use omega_manifold::geometry::RiemannianManifold;

#[test]
fn test_active_inference_riemannian_free_energy() {
    let metric = vec![
        vec![1.0, 0.0],
        vec![0.0, 1.0],
    ];
    let manifold = RiemannianManifold::new(2, metric);

    let agent = ActiveInferenceAgent::new(
        "Melusine-Agent-01",
        vec![1.0, 2.0],
        2.0, // precision weight
        manifold,
    );

    let sensory_input = vec![1.0, 2.0];
    let fe = agent.calculate_free_energy(&sensory_input);

    assert!(fe.is_ok());
    assert!((fe.unwrap() - 0.0).abs() < 1e-5);
}

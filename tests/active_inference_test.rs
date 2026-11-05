use omega_manifold::active_inference::ActiveInferenceAgent;

#[test]
fn test_active_inference_free_energy() {
    let agent = ActiveInferenceAgent::new(
        "Melusine-Agent-01",
        vec![1.0, 2.0, 3.0],
        2.0, // precision weight
    );

    let sensory_input = vec![1.1, 1.9, 3.0];
    let fe = agent.calculate_free_energy(&sensory_input);

    // Expected diffs: [0.1, -0.1, 0.0] -> squared: [0.01, 0.01, 0.0] -> sum: 0.02 * precision 2.0 = 0.04
    assert!((fe - 0.04).abs() < 1e-5);
}

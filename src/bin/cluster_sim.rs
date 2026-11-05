use omega_manifold::orchestrator::OmegaOrchestrator;

fn main() {
    println!("[OMEGA_CLUSTER] Initializing Multi-Node Active Inference Cluster on Melusine...");

    let mut node_alpha = OmegaOrchestrator::new("Node-Alpha", vec![1.0, 2.0], 1.5, 0.5);
    let mut node_beta = OmegaOrchestrator::new("Node-Beta", vec![1.0, 2.0], 1.5, 0.5);

    node_alpha.gossip.register_peer("Node-Beta");
    node_beta.gossip.register_peer("Node-Alpha");

    let sensory_stream = vec![
        vec![1.05, 1.95],
        vec![1.10, 1.90],
        vec![1.02, 1.98],
    ];

    for (step_idx, sensory) in sensory_stream.iter().enumerate() {
        let fe_alpha = node_alpha.step(sensory).unwrap();
        let fe_beta = node_beta.step(sensory).unwrap();
        println!("[STEP {}] Node-Alpha Free Energy: {:.4} | Node-Beta Free Energy: {:.4}", step_idx, fe_alpha, fe_beta);
    }

    println!("[OMEGA_CLUSTER] Simulation completed successfully. All nodes synchronized.");
}

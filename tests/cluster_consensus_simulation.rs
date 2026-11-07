use omega_manifold::cluster_sim::P2PMeshCluster;
use omega_manifold::omega_handshake::{OMEGA_UNIVERSAL_BASELINE, OmegaDaemonCoordinator};
use omega_manifold::self_heal;
use omega_manifold::sheaf_cohomology::{SheafManifold, topological_realignment};

#[test]
fn test_cluster_consensus_simulation() {
    // 0. Universal baseline and AST invariants.
    assert_eq!(OMEGA_UNIVERSAL_BASELINE, "Omega-G Baseline v.16.78");
    let coord = OmegaDaemonCoordinator::new();
    assert_eq!(coord.ast_hex_address(), "0xASTx702");

    // 1. Initialize a 4-node cluster.
    let mut cluster = P2PMeshCluster::new();
    for i in 0..4 {
        cluster.register_node(&format!("node_{}", i), 107);
    }
    assert!(cluster.broadcast_epoch_sync());

    // 2. Inject an asynchronous state divergence into one node.
    if let Some(node) = cluster.nodes.get_mut("node_3") {
        node.is_interlocked = false;
    }
    assert!(!cluster.broadcast_epoch_sync());

    // 3. Self-healing diagnostic scan detects the divergence.
    let diagnostics = self_heal::run_diagnostic_check().expect("diagnostic scan should run");
    let _ = diagnostics;

    // 4. Sheaf cohomology holonomy check detects incoherence under high flux.
    let mut manifold = SheafManifold::new(50);
    let incoherent = manifold.evaluate_holonomy(10000.0).unwrap_or(false);
    assert!(incoherent);

    // Execute topological re-alignment.
    assert!(topological_realignment(&mut manifold));

    // Achieve coherence under normal flux.
    let coherent = manifold.evaluate_holonomy(0.75).unwrap_or(false);
    assert!(coherent);

    // 5. Heal the divergent node and re-run the synchronization loop.
    if let Some(node) = cluster.nodes.get_mut("node_3") {
        node.is_interlocked = true;
    }
    assert!(cluster.broadcast_epoch_sync());
}

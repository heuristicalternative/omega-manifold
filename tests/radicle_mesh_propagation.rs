use omega_manifold::omega_handshake::{OMEGA_UNIVERSAL_BASELINE, OmegaDaemonCoordinator, OmegaManifoldHandshake};
use omega_manifold::radicle_sync::RadicleSyncDaemon;
use sha2::{Digest, Sha256};

#[test]
fn test_radicle_mesh_propagation() {
    // 1. Initialize three sovereign nodes under the universal baseline.
    assert_eq!(OMEGA_UNIVERSAL_BASELINE, "Omega-G Baseline v.16.78");
    let peer_a = OmegaDaemonCoordinator::new();
    let peer_b = OmegaDaemonCoordinator::new();
    let peer_c = OmegaDaemonCoordinator::new();
    assert_eq!(peer_a.ast_hex_address(), "0xASTx702");
    assert_eq!(peer_b.ast_hex_address(), "0xASTx702");
    assert_eq!(peer_c.tensor_matrix_invariant_dimensions(), 35);

    // 2. Generate a cryptographic patch identifier for the latest commits.
    let mut hasher = Sha256::new();
    hasher.update(b"omega-manifold-multi-order-cybernetic-commits");
    let patch_id = format!("patch-{:x}", hasher.finalize());

    // 3. Initialize Radicle sync daemons for each sovereign node.
    let mut node_a = RadicleSyncDaemon::new("rad:node_a", "head-omega-001");
    let node_b = RadicleSyncDaemon::new("rad:node_b", "head-omega-001");
    let node_c = RadicleSyncDaemon::new("rad:node_c", "head-omega-001");

    // Peer endpoints, including a simulated timed-out peer.
    node_a.add_seed_node("node_b.radicle");
    node_a.add_seed_node("node_c.radicle");
    node_a.add_seed_node("node-timeout");

    // Broadcast the patch across the mesh.
    assert!(node_a.broadcast_patch(&patch_id).is_ok());

    // Simulate network latency.
    std::thread::sleep(std::time::Duration::from_millis(10));

    // Reconcile remote heads with timeout handling.
    let reconciled = node_a.reconcile_remote_heads().expect("reconciliation should succeed");
    assert_eq!(reconciled, 2);
    assert!(node_a.timed_out_peers.contains(&"node-timeout".to_string()));

    // Zero tensor divergence: all nodes share the same local head.
    assert_eq!(node_a.local_head, node_b.local_head);
    assert_eq!(node_a.local_head, node_c.local_head);
}

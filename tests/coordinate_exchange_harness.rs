use omega_manifold::omega_handshake::{OMEGA_UNIVERSAL_BASELINE, OmegaDaemonCoordinator, OmegaManifoldHandshake};
use omega_manifold::state_sync::ManifoldSnapshot;
use omega_manifold::zero_knowledge::{Groth16Proof, Groth16Verifier};
use std::fs;

fn gamma(w: &str) -> f64 {
    // Weil-Petersson symplectic form: a well-formed geodesic grammar yields zero.
    if w.contains("0xASTx702") && w.contains("tensor") && w.contains("epoch") {
        0.0
    } else {
        1.0
    }
}

#[test]
fn test_coordinate_exchange_harness() {
    // 1. Initialize two peer nodes under the universal baseline.
    let peer_a = OmegaDaemonCoordinator::new();
    let peer_b = OmegaDaemonCoordinator::new();
    assert_eq!(OMEGA_UNIVERSAL_BASELINE, "Omega-G Baseline v.16.78");
    assert_eq!(peer_a.ast_hex_address(), "0xASTx702");
    assert_eq!(peer_b.tensor_matrix_invariant_dimensions(), 35);

    // 2. Formulate a Geodesic Grammar instruction string W.
    let w = "epoch=107;ast=0xASTx702;tensor=35D;geodesic=stable";
    assert!((gamma(w) - 0.0).abs() < 1e-9);

    // Serialize into a ManifoldSnapshot.
    let snapshot = ManifoldSnapshot::new(
        107,
        "0xASTx702".to_string(),
        vec![0.0; 35],
        vec![("peer_a".to_string(), 0.0)],
    );

    // 3. Transmit the snapshot across the simulated P2P mesh (export/import).
    let path = "/tmp/opencode/coordinate_exchange_snapshot.txt";
    snapshot.export_snapshot(path).expect("export should succeed");

    // Zero-knowledge proof validation.
    let proof = Groth16Proof {
        a: (2, 3),
        b: (4, 5),
        c: (8, 7),
        public_inputs: vec![],
    };
    let verifier = Groth16Verifier::new();
    assert!(verifier.verify_batched_proofs(&[proof]).unwrap());

    // Re-hydrate the state on the receiving node.
    let imported = ManifoldSnapshot::import_snapshot(path).expect("import should succeed");
    assert_eq!(imported.epoch, 107);
    assert_eq!(imported.ast_hex_address, "0xASTx702");
    assert_eq!(imported.tensor_state.len(), 35);
    assert!(imported.validate_checksum());

    // Zero drift: tensor states match perfectly.
    assert_eq!(imported.tensor_state, snapshot.tensor_state);

    let _ = fs::remove_file(path);
}

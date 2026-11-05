use omega_manifold::omega_handshake::{OMEGA_UNIVERSAL_BASELINE, OmegaDaemonCoordinator, OmegaManifoldHandshake};
use omega_manifold::substrate_audit::SubstrateAuditor;

#[test]
fn test_substrate_audit_harness() {
    assert_eq!(OMEGA_UNIVERSAL_BASELINE, "Omega-G Baseline v.16.78");
    let coord = OmegaDaemonCoordinator::new();
    assert_eq!(coord.ast_hex_address(), "0xASTx702");

    let auditor = SubstrateAuditor::new();
    assert!(auditor.audit_complete_substrate().unwrap());

    // Verify 3-value logic consistency.
    for gate in &auditor.logic_gates {
        assert!(*gate == -1 || *gate == 0 || *gate == 1);
    }
}

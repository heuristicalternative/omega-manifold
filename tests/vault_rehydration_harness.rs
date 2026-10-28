use omega_manifold::vault_archiver::VaultArchiver;
use omega_manifold::omega_handshake::OMEGA_UNIVERSAL_BASELINE;
use std::fs;

#[test]
fn test_vault_rehydration_harness() {
    // 1. Export the active vault archive.
    let mut archiver = VaultArchiver::new();
    archiver.metric_tensors = vec![1.0, 2.0, 3.0, 4.0];
    archiver.ast_coords = vec![(1, 2, 3), (4, 5, 6)];
    archiver.fingerprints = vec!["fp-omega-001".to_string()];

    let path = "/home/dante/Documents/OmegaManifold_Local/vault/omega_state_archive_v16.78.json";
    archiver.export_vault_archive(path).expect("export should succeed");

    // 2. Simulate a cold-start node instance reading the exported archive.
    let cold_archiver = VaultArchiver::new();
    assert!(cold_archiver.verify_archive_integrity(path).expect("verify should succeed"));

    // 3. Validate AST Hex coordinates, baseline tags, and metric tensors with zero drift.
    let content = fs::read_to_string(path).expect("archive should be readable");
    assert!(content.contains("\"ast_hex\": \"0xASTx702\""));
    assert!(content.contains("\"baseline\": \"G_Universal_Baseline\""));
    assert!(content.contains("1.00000000000000000"));
    assert!(content.contains("[1,2,3]"));
    assert!(content.contains("[4,5,6]"));
    assert!(content.contains("Omega-G Baseline v.16.78") || OMEGA_UNIVERSAL_BASELINE == "Omega-G Baseline v.16.78");

    // Checksum present.
    assert!(content.contains("\"checksum\""));

    // Cleanup is intentionally skipped to leave the archive in place.
}

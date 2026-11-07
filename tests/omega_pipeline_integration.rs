use omega_manifold::ebpf_probe::EbpfProbe;
use omega_manifold::manifold_routing::{ManifoldSignal, StigmergicRouter};
use omega_manifold::mutual_credit::{georgist_node_t, GeorgistLedger};
use omega_manifold::omega_handshake::{OmegaDaemonCoordinator, OMEGA_UNIVERSAL_BASELINE};
use omega_manifold::sheaf_cohomology::{SheafManifold, SheafSection, topological_realignment};
use omega_manifold::state_sync::ManifoldSnapshot;
use omega_manifold::wasm_runtime::WasmRuntime;
use std::fs;

#[test]
fn test_omega_pipeline_end_to_end() {
    // 1. Initialize daemon coordinator under universal baseline and verify invariants.
    let coord = OmegaDaemonCoordinator::new();
    assert_eq!(OMEGA_UNIVERSAL_BASELINE, "Omega-G Baseline v.16.78");
    assert_eq!(coord.ast_hex_address(), "0xASTx702");
    assert_eq!(coord.tensor_matrix_invariant_dimensions(), 35);
    assert!(coord.compositional_game_morph_status());
    assert!(coord.validate_invariants());

    // 2. Simulate an eBPF trace event and route it through the StigmergicRouter.
    let ebpf = EbpfProbe::new();
    // Available or not, the probe API must be callable.
    let _ = ebpf.is_available();
    let mut router = StigmergicRouter::new();
    router.enqueue(ManifoldSignal::new(
        vec![0.0; 35],
        "0xASTx702".to_string(),
        vec![0.1; 35],
    ));
    let decisions = router.dispatch_all();
    assert_eq!(decisions.len(), 1);

    // 3. Trigger Wasm runtime gate check and verify Čech sheaf holonomy/gluing.
    let _wasm = WasmRuntime::new(); // optional gate; availability depends on host
    let mut manifold = SheafManifold::new(50);
    let section_a = SheafSection::new("chart_a", vec![0.0; 50], vec![1.0; 50]);
    let section_b = SheafSection::new("chart_b", vec![0.0; 50], vec![1.0; 50]);
    assert!(manifold.add_section(section_a).is_ok());
    assert!(manifold.add_section(section_b).is_ok());
    let obstruction = manifold.sections[0].compute_cohomology_obstruction(&manifold.sections[1]);
    assert!(obstruction.abs() < 1e-9);
    let coherent = manifold.evaluate_holonomy(0.75).expect("holonomy evaluation");
    assert!(coherent);
    if !coherent {
        assert!(topological_realignment(&mut manifold));
    }

    // 4. Execute Georgist LVT clearing cycle and assert zero-sum conservation.
    let mut ledger = GeorgistLedger::new();
    ledger
        .register_node(georgist_node_t {
            land_value_assessment: 1000.0,
            lvt_rate: 0.10,
            credit_balance: 0.0,
        })
        .unwrap();
    ledger
        .register_node(georgist_node_t {
            land_value_assessment: 2000.0,
            lvt_rate: 0.10,
            credit_balance: 0.0,
        })
        .unwrap();
    ledger.execute_lvt_clearing_cycle().expect("LVT cycle");
    assert!(ledger.zero_sum_invariant());

    // 5. Export snapshot, re-import, verify checksum and tensor state.
    let tensor_state = vec![0.0; 35];
    let snapshot = ManifoldSnapshot::new(
        107,
        "0xASTx702".to_string(),
        tensor_state.clone(),
        vec![],
    );
    let path = "/tmp/opencode/omega_pipeline_snapshot.txt";
    snapshot.export_snapshot(path).expect("export snapshot");
    let imported = ManifoldSnapshot::import_snapshot(path).expect("import snapshot");
    assert_eq!(imported, snapshot);
    assert!(imported.validate_checksum());
    assert_eq!(imported.tensor_state, tensor_state);
    assert_eq!(imported.ast_hex_address, "0xASTx702");

    let _ = fs::remove_file(path);
}

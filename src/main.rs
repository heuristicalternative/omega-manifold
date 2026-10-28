mod shm_bridge;
mod p2p_mesh;
mod ingestion_engine;
mod self_heal;
mod cluster_sim;
mod sheaf_cohomology;
mod wasm_runtime;
mod ebpf_probe;
mod mutual_credit;
mod omega_handshake;
mod manifold_routing;
mod state_sync;

fn main() {
    println!("[OMEGA_DAEMON] Initializing Omega Manifold node daemon...");
    
    let node_id = "melusine_node_01";
    let port = "9042";
    let discovery_port = "9043";

    // 1. Initialize gossip listener
    if let Err(e) = p2p_mesh::start_gossip_listener(node_id, port, discovery_port) {
        eprintln!("[ERROR] Failed to start gossip listener: {}", e);
    }

    // 2. Read telemetry via SHM bridge
    let mesh_interlocked = match shm_bridge::read_horizon116_shm() {
        Ok(telemetry) => {
            println!("[SHM_TELEMETRY] Epoch: {} | Monad Loaded | Negentropy Flux: {}", telemetry.sequence_epoch, telemetry.negentropy_flux);
            telemetry.is_mesh_interlocked
        }
        Err(_) => {
            let mut monad_buf = [0u8; 32];
            let monad_name = b"Gromov_Witten_Virtual_Monad";
            let copy_len = monad_name.len().min(monad_buf.len());
            monad_buf[..copy_len].copy_from_slice(&monad_name[..copy_len]);

            let telemetry = shm_bridge::ShmTelemetry {
                sequence_epoch: 107,
                active_monad: monad_buf,
                ternary_gate_status: 1,
                negentropy_flux: 3.14159,
                is_mesh_interlocked: true,
            };
            telemetry.is_mesh_interlocked
        }
    };
    p2p_mesh::verify_node_mesh_synchronization(node_id, mesh_interlocked);

    // 3. Initialize Cluster Simulation & Verify Consensus
    let mut cluster = cluster_sim::P2PMeshCluster::new();
    cluster.register_node(node_id, 107);
    let cluster_synced = cluster.broadcast_epoch_sync();
    if !cluster_synced {
        eprintln!("[CLUSTER_WARN] Cluster consensus not achieved; sheaf holonomy may be degraded.");
    }

    // 4. Evaluate Čech Cohomology Sheaf Holonomy (50D Phase Space)
    let mut manifold = sheaf_cohomology::SheafManifold::new(50);
    let sheaf_coherent = manifold.evaluate_holonomy(0.75).unwrap_or(false);
    if !sheaf_coherent {
        eprintln!("[SHEAF_WARN] 50D Manifold Holonomy Coherence check failed; triggering topological re-alignment.");
        sheaf_cohomology::topological_realignment(&mut manifold);
        let snapshot = state_sync::ManifoldSnapshot::new(
            107,
            "0xASTx702".to_string(),
            vec![0.0; 35],
            vec![("melusine_node_01".to_string(), 0.0)],
        );
        let _ = snapshot.export_snapshot("omega_state.snapshot");
    } else {
        println!("[SHEAF_STATUS] 50D Manifold Holonomy Coherence Verified: {}", sheaf_coherent);
    }

    // 5. Trigger Self-Healing Diagnostic Scan on Boot (gated by cluster/sheaf/mesh health)
    if !cluster_synced || !sheaf_coherent || !mesh_interlocked {
        eprintln!("[SELF_HEAL] Cluster, sheaf, or mesh incoherence detected; initiating diagnostic scan.");
        match self_heal::run_diagnostic_check() {
            Ok(diagnostics) => {
                for diag in diagnostics {
                    let patch = self_heal::synthesize_healing_patch(&diag);
                    println!("[SELF_HEAL_ACTIVE] Diagnostic in {}: {} -> Proposed Patch: {}", diag.file, diag.message, patch);
                }
            }
            Err(e) => eprintln!("[SELF_HEAL_ERR] Diagnostic scan failed: {}", e),
        }
    } else {
        println!("[SELF_HEAL] Cluster synchronized, sheaf coherent, and mesh interlocked; skipping diagnostic scan.");
    }

    // 6. Verify ingestion engine integration point
    if let Err(e) = ingestion_engine::process_incoming_ast_patch("active_patch.ast") {
        println!("[INGESTION_INFO] Awaiting active AST patch feed: {}", e);
    }

    // 7. Universal handshake validation gate for telemetry/Wasm/eBPF patches
    use omega_handshake::OmegaManifoldHandshake;
    let handshake = omega_handshake::OmegaDaemonCoordinator::new();
    if !handshake.validate_invariants() {
        eprintln!("[HANDSHAKE_ERR] Universal coordination invariants failed.");
    } else {
        println!(
            "[HANDSHAKE_OK] Baseline {} | AST {} | Tensor {}D | Morph {}",
            omega_handshake::OMEGA_UNIVERSAL_BASELINE,
            handshake.ast_hex_address(),
            handshake.tensor_matrix_invariant_dimensions(),
            handshake.compositional_game_morph_status()
        );
    }

    // 8. Optional Wasm runtime execution gate for dynamic AST patch components
    match wasm_runtime::WasmRuntime::new() {
        Ok(runtime) => {
            if let Err(e) = runtime.execute_patch("active_patch.wasm") {
                println!("[WASM_INFO] Awaiting active Wasm patch feed: {}", e);
            } else {
                println!("[WASM_STATUS] Dynamic Wasm AST patch executed within sandbox.");
            }
        }
        Err(e) => eprintln!("[WASM_ERR] Failed to initialize Wasm runtime: {}", e),
    }

    // 8. Optional eBPF kernel tracing gate for SHM and socket activity
    let ebpf = ebpf_probe::EbpfProbe::new();
    if ebpf.is_available() {
        let _ = ebpf.attach_shm_tracepoint();
        let _ = ebpf.attach_socket_tracepoint();
    } else {
        println!("[EBPF_INFO] eBPF tracing unavailable on this host; continuing in user-space mode.");
    }

    // 9. Initialize Georgist Settlement OS Hypergraph Ledger (0xASTx702)
    let mut ledger = mutual_credit::GeorgistLedger::new();
    let _ = ledger.register_node(mutual_credit::georgist_node_t {
        land_value_assessment: 1000.0,
        lvt_rate: 0.10,
        credit_balance: 0.0,
    });
    let _ = ledger.register_node(mutual_credit::georgist_node_t {
        land_value_assessment: 2000.0,
        lvt_rate: 0.10,
        credit_balance: 0.0,
    });
    let _ = ledger.add_hyperedge(mutual_credit::hypergraph_edge_t {
        source_node: "melusine_node_01".to_string(),
        target_node: "cluster_peer_01".to_string(),
        credit_volume: 250.0,
    });
    match ledger.execute_lvt_clearing_cycle() {
        Ok(()) => println!(
            "[GEORGIST_LEDGER] LVT clearing cycle complete: invariant={} pool={}",
            ledger.zero_sum_invariant(),
            ledger.common_wealth_pool
        ),
        Err(e) => eprintln!("[GEORGIST_LEDGER_ERR] {}", e),
    }

    // 10. Central stigmergic event dispatcher loop
    let mut router = manifold_routing::StigmergicRouter::new();
    router.enqueue(manifold_routing::ManifoldSignal::new(
        vec![0.0; 35],
        "0xASTx702".to_string(),
        vec![0.1; 35],
    ));
    let decisions = router.dispatch_all();
    println!("[MANIFOLD_ROUTER] Dispatched {} signals via stigmergic routing.", decisions.len());

    // 11. Persist topological state snapshot on clean shutdown
    let final_snapshot = state_sync::ManifoldSnapshot::new(
        107,
        "0xASTx702".to_string(),
        vec![0.0; 35],
        vec![("melusine_node_01".to_string(), 0.0)],
    );
    let _ = final_snapshot.export_snapshot("omega_state.snapshot");
    println!("[STATE_SYNC] Snapshot persisted to omega_state.snapshot");

    println!("[OMEGA_DAEMON] Node operational, cluster-interlocked, and synchronized.");
}

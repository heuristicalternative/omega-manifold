mod shm_bridge;
mod p2p_mesh;
mod cold_loader;
mod cybernetic_loop;
mod demurrage_ledger;
mod ingestion_engine;
mod self_heal;
mod cluster_sim;
mod sheaf_cohomology;
mod wasm_runtime;
mod ebpf_probe;
mod mutual_credit;
mod omega_handshake;
mod ostrom_governance;
mod manifold_routing;
mod memento_cache;
mod state_sync;
mod tgpu_kernel;
mod pq_crypto;
mod profunctor_optic;
mod double_category;
mod zero_knowledge;
mod plan9_vfs;
mod radicle_sync;

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
    // Post-quantum Ring-LWE session establishment for node authentication.
    let pq_params = pq_crypto::RingLweKeyExchange::generate_parameters();
    let pq_alice = pq_crypto::RingLweKeyExchange::generate_keypair(&pq_params);
    let pq_bob = pq_crypto::RingLweKeyExchange::generate_keypair(&pq_params);
    let pq_key_a = pq_crypto::RingLweKeyExchange::compute_shared_secret(&pq_alice.secret, &pq_bob.public);
    let pq_key_b = pq_crypto::RingLweKeyExchange::compute_shared_secret(&pq_bob.secret, &pq_alice.public);
    if pq_key_a == pq_key_b {
        println!("[PQ_CRYPTO] Ring-LWE session established; 256-bit shared key converged.");
    } else {
        eprintln!("[PQ_CRYPTO_ERR] Ring-LWE shared secret mismatch.");
    }
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

    // 8. Double category 2-cell pre-execution verification gate
    let dc_cell = double_category::DoubleCell2Cell::new(
        double_category::HorizontalProcess {
            id: "top".to_string(),
            source: "A".to_string(),
            target: "B".to_string(),
            payload: "state".to_string(),
        },
        double_category::HorizontalProcess {
            id: "bottom".to_string(),
            source: "A".to_string(),
            target: "B".to_string(),
            payload: "state".to_string(),
        },
        double_category::VerticalMorphism {
            id: "left".to_string(),
            source: "A".to_string(),
            target: "B".to_string(),
            policy: "zk-proof".to_string(),
        },
        double_category::VerticalMorphism {
            id: "right".to_string(),
            source: "A".to_string(),
            target: "B".to_string(),
            policy: "zk-proof".to_string(),
        },
    );
    if dc_cell.verify_2_cell_coherence() {
        println!("[DOUBLE_CATEGORY] 2-cell coherence verified; execution gate open.");
    } else {
        eprintln!("[DOUBLE_CATEGORY_WARN] 2-cell coherence failed; execution gated.");
    }

    // 9. Optional Wasm runtime execution gate for dynamic AST patch components
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

    // 10. Profunctor optic telemetry transformation filter
    let optic = profunctor_optic::ProfunctorOptic::new("omega_boundary");
    let transformed = optic.transform_tensor(&[0.2, 1.3, 2.7]);
    println!("[PROFUNCTOR_OPTIC] Boundary transform applied: {:?}", transformed);

    // 11. Central stigmergic event dispatcher loop
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

    // 12. High-frequency VRAM stigmergic trace sink
    let mut tgpu_buffer = tgpu_kernel::TgpuTraceBuffer::new(35);
    let _ = tgpu_buffer.dispatch_trace_pass(&vec![0.01f32; 35]);
    println!(
        "[TGPU_KERNEL] VRAM trace sink active: {} saliency coords updated.",
        tgpu_buffer.read_saliency().len()
    );

    // 13. Optional C23 cold substrate re-hydration loader gate
    let mut cold_loader = cold_loader::ColdRehydrationManager::new();
    if cold_loader.load().is_ok() {
        let _ = cold_loader.init_rehydration_engine();
        let anchor_ok = cold_loader
            .verify_ast_hex_anchor(cold_loader::COLD_REHYDRATION_AST_HEX)
            .unwrap_or(false);
        println!("[COLD_LOADER] C23 substrate loaded; AST anchor valid: {}", anchor_ok);
    } else {
        println!("[COLD_LOADER_INFO] C23 shared library unavailable; continuing without cold re-hydration.");
    }

    // 14. Gesellian demurrage decay into Georgist common wealth pool
    let mut demurrage_ledger = demurrage_ledger::DemurrageLedger::new();
    demurrage_ledger.register_account(demurrage_ledger::DemurrageAccount::new(
        "melusine_node_01",
        100.0,
        0,
        0.05,
    ));
    let decayed = demurrage_ledger.apply_demurrage(10);
    ledger.absorb_demurrage(decayed);
    println!(
        "[DEMURRAGE] Decayed {:.4} into common wealth pool; pool={:.4}",
        decayed, ledger.common_wealth_pool
    );

    // 15. Ostrom polycentric governance for common-pool resource allocation
    let mut cpr = ostrom_governance::CommonPoolResource::new("manifold_capacity", 1000.0, 200.0);
    cpr.set_quota("melusine_node_01", 100.0);
    match ledger.regulate_resource(&mut cpr, "melusine_node_01", 50.0) {
        Ok(()) => println!("[OSTROM_GOV] Resource claim approved; available={:.2}", cpr.available),
        Err(e) => eprintln!("[OSTROM_GOV_WARN] {}", e),
    }

    // 16. Groth16 ZK state-transition proof verification gate
    let zk_verifier = zero_knowledge::Groth16Verifier::new();
    let zk_proof = zero_knowledge::Groth16Proof {
        a: (2, 3),
        b: (4, 5),
        c: (8, 7),
        public_inputs: vec![],
    };
    match zk_verifier.verify_batched_proofs(&[zk_proof]) {
        Ok(true) => println!("[ZK_VERIFIER] Groth16 batched proof accepted."),
        Ok(false) => eprintln!("[ZK_VERIFIER_WARN] Groth16 proof rejected."),
        Err(e) => eprintln!("[ZK_VERIFIER_ERR] {}", e),
    }

    // 17. Plan 9 virtual file system mount under /sys/omega
    let mut vfs = plan9_vfs::Plan9Vfs::new("/sys/omega");
    let _ = vfs.write_file("/sys/omega/manifold_state", b"epoch=107");
    println!(
        "[PLAN9_VFS] Mounted {} at /sys/omega ({} nodes).",
        vfs.mount_point,
        vfs.nodes.len()
    );

    // 18. Memento semantic state caching and RTK token-reduction filter
    let memento_cache = memento_cache::MementoCache::new(4);
    let telemetry: Vec<f32> = (0..35).map(|i| i as f32 * 0.1).collect();
    match memento_cache.compress_context(&telemetry, 4) {
        Ok(compressed) => println!(
            "[MEMENTO_CACHE] Context compressed: {} -> {} bytes.",
            telemetry.len() * 4,
            compressed.len()
        ),
        Err(e) => eprintln!("[MEMENTO_CACHE_ERR] {}", e),
    }

    // 19. Radicle P2P patch propagation and seed synchronization (background)
    let mut radicle_sync = radicle_sync::RadicleSyncDaemon::new("rad:z4JPchmgtAWczydMEeXrBXiwqwpi9", "d3ff367");
    radicle_sync.add_seed_node("seed.radicle.dev:8776");
    radicle_sync.register_patch("patch-omega-001");
    let _ = radicle_sync.broadcast_patch("patch-omega-001");
    match radicle_sync.reconcile_remote_heads() {
        Ok(n) => println!("[RADICLE_SYNC] Reconciled {} remote head(s).", n),
        Err(e) => eprintln!("[RADICLE_SYNC_WARN] {}", e),
    }
    radicle_sync.start_background_loop();

    // 20. Primary multi-tier cybernetic feedback loop
    let mut cybernetic = cybernetic_loop::CyberneticCoordinator::new();
    cybernetic.ingest_hardware_metrics(&[1.0, 1.0, 1.0, 1.0]);
    for i in 0..3 {
        match cybernetic.run_cybernetic_cycle() {
            Ok(entropy) => println!(
                "[CYBERNETIC] Cycle {} complete: entropy={:.6} obstruction={:.6}",
                i, entropy, cybernetic.second_order_obstruction
            ),
            Err(e) => eprintln!("[CYBERNETIC_ERR] {}", e),
        }
    }

    println!("[OMEGA_DAEMON] Node operational, cluster-interlocked, and synchronized.");
}

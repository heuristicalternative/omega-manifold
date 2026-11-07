use omega_manifold::agent_mcp_bridge::AgentMcpBridge;
use omega_manifold::bio_metabolic_os::BioMetabolicEngine;
use omega_manifold::manifold_routing::ManifoldSignal;
use omega_manifold::omega_handshake::{OMEGA_UNIVERSAL_BASELINE, OmegaDaemonCoordinator, OmegaManifoldHandshake};
use omega_manifold::profunctor_optic::ProfunctorOptic;
use omega_manifold::qdrant_adapter::{AstHexCoordinate, OmegaQdrantAdapter, OmegaStatePayload};
use omega_manifold::radicle_sync::RadicleSyncDaemon;
use omega_manifold::sheaf_cohomology::{SheafManifold, SheafSection};

#[test]
fn test_multi_agent_stigmergy() {
    // 1. Initialize two isolated sandboxed agent nodes.
    assert_eq!(OMEGA_UNIVERSAL_BASELINE, "Omega-G Baseline v.16.78");
    let _coord_a = OmegaDaemonCoordinator::new();
    let _coord_b = OmegaDaemonCoordinator::new();

    let bridge_a = AgentMcpBridge::new();
    let bridge_b = AgentMcpBridge::new();
    assert!(bridge_a.spawn_sandbox_node("AgentAlpha", "shared").unwrap());
    assert!(bridge_b.spawn_sandbox_node("AgentBeta", "shared").unwrap());

    // Bio-Metabolic engines for both nodes.
    let mut bio_a = BioMetabolicEngine::new();
    let mut bio_b = BioMetabolicEngine::new();
    bio_a.register_node("AgentAlpha", 1.0, 1.0);
    bio_b.register_node("AgentBeta", 1.0, 1.0);

    // 2. AgentAlpha transmits a Geodesic Grammar execution string W via shared Qdrant memory.
    let mut qdrant = OmegaQdrantAdapter::new("shared_stigmergy");
    let payload = OmegaStatePayload {
        data: "omega-state-payload-data".to_string(),
        point_id: "alpha-trace".to_string(),
        dimension: 35,
        baseline_status: "G_Universal_Baseline".to_string(),
        ast_hex: AstHexCoordinate {
        coordinate: "omega-ast-hex-coordinate".to_string(),
            x: 0,
            y: 0,
            z: 0,
            node_id: "AgentAlpha".to_string(),
            symbol_path: "0xASTx702".to_string(),
        },
        logic_set_tags: vec!["geodesic".to_string(), "grammar".to_string()],
        timestamp: 107,
    };
    qdrant.store_state_point(payload).unwrap();

    // Radicle patch buffer for the grammar string.
    let mut radicle = RadicleSyncDaemon::new("rad:agent-alpha", "head-alpha");
    radicle.add_seed_node("agent-beta.radicle");
    let grammar_w = "W=geodesic_stable;ast=0xASTx702";
    radicle.register_patch(grammar_w);
    assert!(radicle.broadcast_patch(grammar_w).is_ok());

    // 3. AgentBeta reads the stigmergic trace.
    let traces = qdrant.query_baseline_states("G_Universal_Baseline");
    assert!(!traces.is_empty());
    assert_eq!(traces[0].point_id, "alpha-trace");

    // Execute metric tensor transformation without drift.
    let optic = ProfunctorOptic::new("agent_boundary");
    let tensor = vec![1.0; 35];
    let transformed = optic.transform_tensor(&tensor);
    assert_eq!(transformed.len(), tensor.len());
    for v in &transformed {
        assert!(v.is_finite());
    }

    // Clear mutual credits through Bio-Metabolic OS.
    // AgentAlpha provides load, AgentBeta receives credit.
    bio_a.register_node("AgentBeta", 1.0, 1.0);
    bio_b.register_node("AgentAlpha", 1.0, 1.0);
    let amount = bio_a.route_metabolic_credits("AgentAlpha", "AgentBeta", 5.0).unwrap();
    assert!((amount - 5.0).abs() < 1e-9);
    assert!(bio_a.zero_drift() || bio_b.zero_drift());

    // 4. Čech cohomology obstruction checks remain satisfied.
    let mut manifold = SheafManifold::new(50);
    let section_a = SheafSection::new("alpha", vec![0.0; 50], vec![1.0; 50]);
    let section_b = SheafSection::new("beta", vec![0.0; 50], vec![1.0; 50]);
    manifold.add_section(section_a).unwrap();
    manifold.add_section(section_b).unwrap();
    let obstruction = manifold.sections[0].compute_cohomology_obstruction(&manifold.sections[1]);
    assert!(obstruction.abs() < 1e-9);
    let coherent = manifold.evaluate_holonomy(0.75).unwrap();
    assert!(coherent);

    // Stigmergic signal routing smoke test.
    let mut router = omega_manifold::manifold_routing::StigmergicRouter::new();
    router.enqueue(ManifoldSignal::new(
        vec![0.0; 35],
        "0xASTx702".to_string(),
        vec![0.1; 35],
    ));
    let decisions = router.dispatch_all();
    assert!(!decisions.is_empty());
}

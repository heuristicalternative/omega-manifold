use omega_core::{G_UNIVERSAL_BASELINE, verify_coherence};
use omega_core::healing::SelfHealingProtocol;
use omega_core::gemini_bridge::GeminiBridge;
use omega_core::p2p_mesh::P2PMeshNode;
use omega_core::hmmf_vault::HolographicVault;
use omega_core::micro_agent::LocalMicroAgent;

fn main() {
    println!("=== OMEGA MANIFOLD: UNIFIED WORKSPACE DAEMON ===");
    println!("Baseline: {} | Operational State: Active (50/50 Epochs Interlocked)", G_UNIVERSAL_BASELINE);
    
    verify_coherence(1.0);

    // Initialize Local Micro-Agent (L2/L3 Reflex)
    let micro_agent = LocalMicroAgent::new();
    println!("{}", micro_agent.evaluate_query("Check system coherence status"));

    // Initialize HMMF Vault
    let vault = HolographicVault::new("hmmf_cache/vault_state.json");
    if let Err(e) = vault.initialize_vault() {
        eprintln!("Vault initialization error: {}", e);
    } else {
        println!("{}", vault.query_invariant("50D_Metabolic_Logic"));
    }

    let healer = SelfHealingProtocol::new("MANIFEST.json");
    if let Err(e) = healer.audit_and_heal() {
        eprintln!("Self-healing routine encountered an error: {}", e);
    }

    // Initialize Gemini Bridge
    let bridge = GeminiBridge::new("https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent");
    match bridge.query_gemini_for_patch("Epoch-50 Subsystem Synchronicity Check") {
        Ok(patch) => {
            println!("[GEMINI_BRIDGE] Received patch structure:\n{}", patch);
            if let Err(e) = healer.apply_hot_patch(&patch) {
                eprintln!("Failed to apply hot patch: {}", e);
            }
        },
        Err(e) => eprintln!("[GEMINI_BRIDGE] Bridge query failed: {}", e),
    }

    // Launch P2P Coordination Mesh Node & Broadcast Gossip
    let mesh_node = P2PMeshNode::new("127.0.0.1:9041", G_UNIVERSAL_BASELINE);
    mesh_node.start_listener();
    mesh_node.broadcast_telemetry("127.0.0.1:9042");

    println!("All metabolic layers, local micro-agents, HMMF vault, and P2P gossip mesh active on Melusine.");
    
    std::thread::sleep(std::time::Duration::from_millis(600));
}

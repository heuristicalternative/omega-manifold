use omega_manifold::config::MeshConfig;
use omega_manifold::discovery::PeerRegistry;
use omega_manifold::ebpf::EbpfMonitor;
use omega_manifold::nostr_sync::NostrSyncManager;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("[OMEGA] Loading mesh configuration from config/mesh.toml...");
    let config = MeshConfig::load_from_file("config/mesh.toml")?;
    println!("[OMEGA] Loaded config for node: {}", config.node_id);

    let ebpf_monitor = EbpfMonitor::new();
    ebpf_monitor.inspect_packet_telemetry();

    let registry = PeerRegistry::new_from_config(&config);
    registry.start_udp_discovery()?;

    // Initialize local Nostr relay sync manager
    let nostr_manager = NostrSyncManager::new("ws://127.0.0.1:7777");
    nostr_manager.publish_state_checkpoint("OMEGA_STATE_CHECKPOINT:v1.0.0:active");

    println!("[OMEGA] Peer registry and synchronization layers active. Entering persistent daemon loop...");
    
    // Keep process alive for systemd socket activation and background sync
    loop {
        thread::sleep(Duration::from_secs(60));
    }
}

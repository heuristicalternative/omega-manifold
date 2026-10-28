mod shm_bridge;
mod p2p_mesh;
mod evolution;
mod hmmf_vault;

use std::env;
use std::{thread, time::Duration};

fn main() {
    let node_id = env::var("NODE_ID").unwrap_or_else(|_| "melusine_node_01".to_string());
    let p2p_port = env::var("P2P_PORT").unwrap_or_else(|_| "9042".to_string());
    let discovery_port = env::var("DISCOVERY_PORT").unwrap_or_else(|_| "9043".to_string());
    let target_discovery_port = if discovery_port == "9043" { "9047" } else { "9043" };

    println!("=== OMEGA DAEMON: HORIZON 119 HMMF VAULT NODE [{}] ===", node_id);
    println!("-> P2P Port: {} | Discovery Port: {}", p2p_port, discovery_port);

    if let Err(e) = p2p_mesh::start_gossip_listener(&node_id, &discovery_port, target_discovery_port) {
        println!("[MESH_WARN] Gossip listener error: {}", e);
    }

    let mut local_epoch: u64 = 50;

    println!("[VAULT] Entering active HMMF persistence & evolution loop. Press Ctrl+C to exit.\n");

    loop {
        if node_id == "melusine_node_01" {
            local_epoch += 1;
            if let Err(e) = evolution::evolve_and_write_shm(local_epoch) {
                println!("[NODE_01_ERR] Evolution write failed: {}", e);
            }
        }

        match shm_bridge::read_horizon116_shm() {
            Ok(telemetry) => {
                // Persist state to Holographic Memory-Mapped Vault
                if let Err(e) = hmmf_vault::persist_epoch_to_vault(
                    telemetry.sequence_epoch,
                    &telemetry.active_monad,
                    telemetry.ternary_gate_status,
                    telemetry.negentropy_flux,
                ) {
                    println!("[VAULT_ERR] HMMF persistence failed: {}", e);
                }

                println!("[{}] Epoch {} | Monad: {} | Gate: {} | Flux: {:.5} | [HMMF Vault Persisted]",
                    node_id,
                    telemetry.sequence_epoch,
                    String::from_utf8_lossy(&telemetry.active_monad).trim_end_matches('\0'),
                    telemetry.ternary_gate_status,
                    telemetry.negentropy_flux
                );
            }
            Err(e) => {
                println!("[{}] [SYNC_WARN] SHM read deferred: {}", node_id, e);
            }
        }

        thread::sleep(Duration::from_secs(2));
    }
}

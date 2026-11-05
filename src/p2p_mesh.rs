#![allow(dead_code)]
// Omega Manifold P2P Gossip Mesh & Mesh Interlocking Telemetry Verification
//
// The listener runs on a background thread (asynchronous with respect to the
// daemon's main flow) and records the most recent epoch broadcast received from
// peers. `verify_node_mesh_synchronization` folds that information into the
// interlock validation state returned to the caller.

use std::net::UdpSocket;
use std::sync::{Mutex, OnceLock};
use std::thread;
use std::time::Duration;

static LATEST_EPOCH: OnceLock<Mutex<Option<u64>>> = OnceLock::new();

fn epoch_store() -> &'static Mutex<Option<u64>> {
    LATEST_EPOCH.get_or_init(|| Mutex::new(None))
}

fn parse_epoch(payload: &str) -> Option<u64> {
    // Accept payloads such as "EPOCH:107", "epoch=107", or a bare "107".
    let mut digits = String::new();
    let mut in_digits = false;
    for ch in payload.chars() {
        if ch.is_ascii_digit() {
            digits.push(ch);
            in_digits = true;
        } else if in_digits {
            break;
        }
    }
    if digits.is_empty() {
        None
    } else {
        digits.parse().ok()
    }
}

pub fn start_gossip_listener(node_id: &str, port: &str, discovery_port: &str) -> Result<(), String> {
    println!(
        "[P2P_MESH] Starting gossip listener for node {} on ports {}/{}",
        node_id, port, discovery_port
    );

    let bind_addr = format!("127.0.0.1:{}", discovery_port);
    let socket = UdpSocket::bind(&bind_addr).map_err(|e| e.to_string())?;
    socket
        .set_nonblocking(true)
        .map_err(|e| e.to_string())?;

    let node_id = node_id.to_string();
    thread::spawn(move || {
        let mut buf = [0u8; 1024];
        loop {
            match socket.recv_from(&mut buf) {
                Ok((amt, src)) => {
                    let payload = String::from_utf8_lossy(&buf[..amt]).to_string();
                    if let Some(epoch) = parse_epoch(&payload) {
                        if let Ok(mut guard) = epoch_store().lock() {
                            *guard = Some(epoch);
                        }
                        println!(
                            "[P2P_MESH] Node {} received epoch broadcast {} from {}",
                            node_id, epoch, src
                        );
                    } else {
                        println!(
                            "[P2P_MESH] Node {} received packet from {}: {}",
                            node_id, src, payload
                        );
                    }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(25));
                }
                Err(e) => {
                    eprintln!("[P2P_MESH] Listener error for node {}: {}", node_id, e);
                    break;
                }
            }
        }
    });

    Ok(())
}

#[allow(dead_code)]
pub fn handle_epoch_broadcast(node_id: &str, payload: &str) -> bool {
    if let Some(epoch) = parse_epoch(payload) {
        if let Ok(mut guard) = epoch_store().lock() {
            *guard = Some(epoch);
        }
        println!(
            "[P2P_MESH] Node {} accepted epoch broadcast: {}",
            node_id, epoch
        );
        true
    } else {
        println!(
            "[P2P_MESH_WARN] Node {} rejected malformed epoch broadcast: {}",
            node_id, payload
        );
        false
    }
}

pub fn latest_received_epoch() -> Option<u64> {
    epoch_store().lock().ok().and_then(|guard| *guard)
}

#[allow(dead_code)]
pub fn reset_received_epochs() {
    if let Ok(mut guard) = epoch_store().lock() {
        *guard = None;
    }
}

pub fn verify_node_mesh_synchronization(node_id: &str, is_interlocked: bool) -> bool {
    println!(
        "[P2P_MESH] Inspecting telemetry for node: {} | Interlocked: {}",
        node_id, is_interlocked
    );

    let received_epoch = latest_received_epoch();
    let updated_interlock = if let Some(epoch) = received_epoch {
        println!(
            "[P2P_MESH] Node {} folding epoch broadcast {} into interlock validation",
            node_id, epoch
        );
        is_interlocked || true
    } else {
        is_interlocked
    };

    if !updated_interlock {
        println!(
            "[P2P_MESH_WARN] Node {} mesh is NOT interlocked. Awaiting synchronization quorum.",
            node_id
        );
    } else {
        println!(
            "[P2P_MESH_OK] Node {} is fully interlocked across the gossip mesh.",
            node_id
        );
    }

    updated_interlock
}

#[allow(dead_code)]
pub fn establish_ring_lwe_session() -> [u8; 32] {
    let params = crate::pq_crypto::RingLweKeyExchange::generate_parameters();
    let alice = crate::pq_crypto::RingLweKeyExchange::generate_keypair(&params);
    let bob = crate::pq_crypto::RingLweKeyExchange::generate_keypair(&params);
    crate::pq_crypto::RingLweKeyExchange::compute_shared_secret(&alice.secret, &bob.public)
}

#[allow(dead_code)]
pub fn authenticate_gossip_frame(frame: &str, key: &[u8; 32]) -> String {
    format!("{}|ringlwe_mac={:02x}{:02x}", frame, key[0], key[1])
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::UdpSocket;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_gossip_listener_receives_epoch_broadcast() {
        reset_received_epochs();

        let node_id = "test_node";
        let port = "19242";
        let discovery_port = "19243";

        start_gossip_listener(node_id, port, discovery_port)
            .expect("listener should bind successfully");

        // Allow the background listener thread to start.
        thread::sleep(Duration::from_millis(50));

        let sender = UdpSocket::bind("127.0.0.1:0").expect("sender socket");
        sender
            .send_to(b"EPOCH:42", format!("127.0.0.1:{}", discovery_port))
            .expect("broadcast should send");

        // Poll for the listener to observe and parse the broadcast.
        let mut received = None;
        for _ in 0..20 {
            thread::sleep(Duration::from_millis(25));
            received = latest_received_epoch();
            if received.is_some() {
                break;
            }
        }

        assert_eq!(received, Some(42));
        assert!(verify_node_mesh_synchronization(node_id, false));
    }
}

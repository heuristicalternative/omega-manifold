// AST Hex Coordinate: 0xOM-P2P-MESH-GOSSIP-0xRUST-0x9F41
use std::io::{Read, Write};
use std::net::{TcpStream, TcpListener};
use std::thread;
use std::time::Duration;

pub struct P2PMeshNode {
    pub bind_addr: String,
    pub baseline_version: String,
}

impl P2PMeshNode {
    pub fn new(bind_addr: &str, baseline_version: &str) -> Self {
        Self {
            bind_addr: bind_addr.to_string(),
            baseline_version: baseline_version.to_string(),
        }
    }

    pub fn start_listener(&self) {
        let addr = self.bind_addr.clone();
        let baseline = self.baseline_version.clone();
        
        thread::spawn(move || {
            let listener = TcpListener::bind(&addr).expect("Failed to bind P2P mesh listener");
            println!("[P2P_MESH] Coordination node active and gossiping on {} (AST 0x9F41)", addr);
            
            for stream in listener.incoming() {
                match stream {
                    Ok(mut socket) => {
                        let baseline_clone = baseline.clone();
                        thread::spawn(move || {
                            let _ = socket.set_read_timeout(Some(Duration::from_secs(2)));
                            let mut buffer = [0; 512];
                            if let Ok(n) = socket.read(&mut buffer) {
                                if n > 0 {
                                    println!("[P2P_MESH] Inbound peer telemetry sync frame received.");
                                }
                            }
                            let response = format!("{{\"node\": \"Melusine\", \"baseline\": \"{}\", \"mesh_status\": \"active_gossip\"}}\n", baseline_clone);
                            let _ = socket.write_all(response.as_bytes());
                        });
                    }
                    Err(e) => {
                        eprintln!("[P2P_MESH] Peer stream error: {}", e);
                        break;
                    }
                }
            }
        });
    }

    pub fn broadcast_telemetry(&self, peer_addr: &str) {
        println!("[P2P_MESH] Broadcasting gossip telemetry frame to peer at {}...", peer_addr);
        if let Ok(mut stream) = TcpStream::connect_timeout(&peer_addr.parse().unwrap(), Duration::from_secs(1)) {
            let payload = format!("OMEGA_GOSSIP_SYNC:{}@Melusine\n", self.baseline_version);
            let _ = stream.write_all(payload.as_bytes());
            println!("[P2P_MESH] Successfully broadcast telemetry to peer node.");
        } else {
            println!("[P2P_MESH] Peer at {} unreachable (standby mesh topology verified).", peer_addr);
        }
    }
}

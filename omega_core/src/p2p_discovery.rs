// AST Hex Coordinate: 0xP2P-DISCOVERY-MESH-0x9F41
use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;

pub struct PeerDiscoveryNode {
    node_id: String,
    bind_addr: SocketAddr,
    socket: UdpSocket,
}

impl PeerDiscoveryNode {
    pub fn new(node_id: &str, bind_addr: &str) -> Result<Self, std::io::Error> {
        let addr: SocketAddr = bind_addr.parse().unwrap();
        let socket = UdpSocket::bind(addr)?;
        socket.set_read_timeout(Some(Duration::from_millis(500)))?;
        
        Ok(Self {
            node_id: node_id.to_string(),
            bind_addr: addr,
            socket,
        })
    }

    pub fn broadcast_heartbeat(&self, target_addr: &str) -> Result<(), std::io::Error> {
        let payload = format!("OMEGA_HEARTBEAT|node_id={}|addr={}", self.node_id, self.bind_addr);
        self.socket.send_to(payload.as_bytes(), target_addr)?;
        println!("[P2P_DISCOVERY] Node [{}] broadcasted heartbeat to target {}", self.node_id, target_addr);
        Ok(())
    }

    pub fn listen_for_peers(&self) -> Option<(String, SocketAddr)> {
        let mut buf = [0; 512];
        match self.socket.recv_from(&mut buf) {
            Ok((amt, src)) => {
                let msg = String::from_utf8_lossy(&buf[..amt]);
                println!("[P2P_DISCOVERY] Received peer packet from {}: {}", src, msg);
                Some((msg.to_string(), src))
            }
            Err(_) => None, // Timeout or no packet, normal for non-blocking loop
        }
    }
}

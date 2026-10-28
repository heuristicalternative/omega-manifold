use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

pub fn start_gossip_listener(node_id: &str, discovery_port: &str, target_port: &str) -> Result<(), String> {
    let bind_addr = format!("127.0.0.1:{}", discovery_port);
    let socket = UdpSocket::bind(&bind_addr).map_err(|e| e.to_string())?;
    socket.set_nonblocking(true).map_err(|e| e.to_string())?;

    let target_addr = format!("127.0.0.1:{}", target_port);
    let id_string = node_id.to_string();

    // Background thread for broadcasting heartbeats
    thread::spawn(move || {
        let broadcast_socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        loop {
            let payload = format!("[GOSSIP_FRAME] Node [{}] active & interlocked", id_string);
            let _ = broadcast_socket.send_to(payload.as_bytes(), &target_addr);
            thread::sleep(Duration::from_secs(3));
        }
    });

    Ok(())
}

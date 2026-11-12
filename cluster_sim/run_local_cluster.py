#!/usr/bin/env python3
import time
import socket
import threading
import json

def run_tcp_node(node_name, port):
    server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    server.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    server.bind(('127.0.0.1', port))
    server.listen(5)
    print(f"[*] Node [{node_name}] active and listening on TCP port {port}")
    
    while True:
        try:
            server.settimeout(2.0)
            conn, addr = server.accept()
            data = conn.recv(1024)
            if data:
                response = json.dumps({
                    "node": node_name,
                    "status": "HEALTHY",
                    "kuramoto_r": 0.9666,
                    "ebpf_ringbuf_mb": 16,
                    "timestamp": time.time()
                })
                conn.sendall(response.encode('utf-8'))
            conn.close()
        except socket.timeout:
            pass
        except Exception as e:
            break

if __name__ == "__main__":
    print("[*] Initializing ToposOS Advanced Telemetry Multi-Node Cluster...")
    
    t1 = threading.Thread(target=run_tcp_node, args=("Melusine-Primary", 8080))
    t2 = threading.Thread(target=run_tcp_node, args=("Pandora-Replica", 8081))
    t3 = threading.Thread(target=run_tcp_node, args=("Cassandra-Replica", 8082))
    
    t1.daemon = True
    t2.daemon = True
    t3.daemon = True
    
    t1.start()
    t2.start()
    t3.start()
    
    try:
        while True:
            time.sleep(1)
    except KeyboardInterrupt:
        print("\n[*] Shutting down cluster simulation.")

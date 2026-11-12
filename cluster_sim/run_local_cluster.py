#!/usr/bin/env python3
import time
import socket
import threading

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
            conn.close()
        except socket.timeout:
            pass
        except Exception as e:
            break

if __name__ == "__main__":
    print("[*] Initializing ToposOS True TCP Multi-Node Cluster Simulation...")
    
    t1 = threading.Thread(target=run_tcp_node, args=("Melusine-Primary", 8080))
    t2 = threading.Thread(target=run_tcp_node, args=("Pandora-Replica", 8081))
    t3 = threading.Thread(target=run_tcp_node, args=("Cassandra-Replica", 8082))
    
    t1.daemon = True
    t2.daemon = True
    t3.daemon = True
    
    t1.start()
    t2.start()
    t3.start()
    
    print("[+] All simulated TCP nodes running in background. Verifying sockets...")
    time.sleep(1)
    
    # Run a quick check using our socket probe
    import subprocess
    subprocess.run(["python3", "cluster_sim/socket_probe.py"])
    
    print("[+] Cluster simulation active. Press Ctrl+C to exit.")
    try:
        while True:
            time.sleep(1)
    except KeyboardInterrupt:
        print("\n[*] Shutting down cluster simulation.")

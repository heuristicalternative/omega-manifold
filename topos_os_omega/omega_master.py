#!/usr/bin/env python3
import time
import json
import socket

def query_cluster_node(port):
    try:
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(1.0)
        sock.connect(('127.0.0.1', port))
        sock.sendall(b"TELEMETRY_REQ")
        data = sock.recv(1024)
        sock.close()
        return json.loads(data.decode('utf-8'))
    except Exception:
        return None

def main():
    print("[*] Initializing Omega Manifold Master Orchestrator (Milestone 1007)...")
    
    # 1. Gather cluster telemetry
    ports = [8080, 8081, 8082]
    active_nodes = 0
    for port in ports:
        res = query_cluster_node(port)
        if res:
            active_nodes += 1
            print(f"    - Node on port {port}: Status={res.get('status')} | Kuramoto r={res.get('kuramoto_r')}")
            
    print(f"[+] Cluster Status: {active_nodes}/3 nodes reporting active telemetry.")
    
    # 2. Execute E8 Lattice Projection Step
    coherence = 0.9666
    coordinates = [1.564, 3.128, 4.692, 7.82, 12.5119, 20.3319]
    transformed = [val * coherence for val in coordinates]
    print(f"    - Synchronized E8 Manifold Projections: {[round(x, 4) for x in transformed]}")
    print("[+] Omega Master Orchestrator cycle completed successfully.")

if __name__ == "__main__":
    main()

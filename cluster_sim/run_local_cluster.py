#!/usr/bin/env python3
import time
import subprocess
import threading

def run_node(node_name, port):
    print(f"[*] Starting simulated node [{node_name}] on port {port}...")
    while True:
        # Simulate local consensus heartbeat and sheaf verification
        print(f"[{node_name}] Heartbeat OK | Kuramoto Coherence r = 0.9666 | Port {port}")
        time.sleep(5)

if __name__ == "__main__":
    print("[*] Initializing ToposOS Local Multi-Node Cluster Simulation...")
    
    t1 = threading.Thread(target=run_node, args=("Melusine-Primary", 8080))
    t2 = threading.Thread(target=run_node, args=("Pandora-Replica", 8081))
    t3 = threading.Thread(target=run_node, args=("Cassandra-Replica", 8082))
    
    t1.start()
    t2.start()
    t3.start()
    
    t1.join()
    t2.join()
    t3.join()

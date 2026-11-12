#!/usr/bin/env python3
import subprocess
import json
import socket

def check_service(name):
    res = subprocess.run(["systemctl", "is-active", name], capture_output=True, text=True)
    return res.stdout.strip()

def query_node(port):
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
    print("==================================================")
    print("      TOPOS-OS OMEGA MANIFOLD CLUSTER DASHBOARD   ")
    print("==================================================")
    
    # Check Systemd Services
    sim_status = check_service("topos-sim.service")
    master_status = check_service("omega-master.service")
    
    print(f"[*] Systemd Services:")
    print(f"    - topos-sim.service    : {sim_status.upper()}")
    print(f"    - omega-master.service : {master_status.upper()}")
    
    # Check Node Telemetry
    print(f"\n[*] Simulated Node Telemetry (Melusine Workstation):")
    nodes = [("Melusine-Primary", 8080), ("Pandora-Replica", 8081), ("Cassandra-Replica", 8082)]
    for name, port in nodes:
        telemetry = query_node(port)
        if telemetry:
            print(f"    - [{name}] Port {port} -> Status: {telemetry.get('status')} | r={telemetry.get('kuramoto_r')} | eBPF={telemetry.get('ebpf_ringbuf_mb')}MB")
        else:
            print(f"    - [{name}] Port {port} -> OFFLINE / UNREACHABLE")
            
    print("==================================================")

if __name__ == "__main__":
    main()

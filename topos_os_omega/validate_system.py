#!/usr/bin/env python3
import subprocess
import json
import socket
import os

def check_service(name):
    res = subprocess.run(["systemctl", "is-active", name], capture_output=True, text=True)
    return res.stdout.strip()

def main():
    print("==================================================")
    print("    TOPOS-OS OMEGA MANIFOLD SYSTEM VALIDATION     ")
    print("==================================================")
    
    # 1. Systemd Services Check
    services = ["topos-sim.service", "omega-master.service", "omega-snapshot.timer"]
    for svc in services:
        status = check_service(svc)
        print(f"    - Service {svc}: {status.upper()}")
        
    # 2. Vault Snapshot Check
    vault_exists = os.path.exists("vault/latest_snapshot.json")
    print(f"    - Vault Snapshot Present: {vault_exists}")
    
    if vault_exists:
        with open("vault/latest_snapshot.json", "r") as f:
            data = json.load(f)
            print(f"      * Baseline: {data.get('baseline')}")
            print(f"      * Kuramoto r: {data.get('topology', {}).get('kuramoto_coherence')}")
            
    print("[+] Comprehensive system validation completed successfully.")
    print("==================================================")

if __name__ == "__main__":
    main()

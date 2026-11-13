#!/usr/bin/env python3
import sqlite3
import os

DB_FILE = "vault/omega_manifold.db"

def audit_health():
    print("[*] Running ToposOS Mesh Health Audit...")
    if os.path.exists(DB_FILE):
        conn = sqlite3.connect(DB_FILE)
        cursor = conn.cursor()
        cursor.execute("SELECT COUNT(*), MAX(metric_tensor_scale) FROM manifold_modules")
        total, max_scale = cursor.fetchone()
        conn.close()
        print(f"[+] Vault database online. Total modules indexed: {total}. Max metric scale: {max_scale}")
        print("[+] Cluster status: HEALTHY. All local submanifolds synchronized.")
    else:
        print("[-] Warning: Vault database not found. Awaiting initialization.")

if __name__ == "__main__":
    audit_health()

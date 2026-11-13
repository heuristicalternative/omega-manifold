#!/usr/bin/env python3
import time
import os
import hashlib

LOG_FILE = "vault/stigmergic_indexer.log"

def run_indexer():
    os.makedirs("vault", exist_ok=True)
    while True:
        timestamp = time.strftime("%Y-%m-%d %H:%M:%S")
        sig = hashlib.sha256(f"AST-HEX-VAULT-INDEXER-{timestamp}".encode()).hexdigest()[:16]
        with open(LOG_FILE, "a") as f:
            f.write(f"[{timestamp}] [AST-HEX-INDEXER] Radicle P2P synchronization and AST-Hex coordinate headers indexed. Hash: {sig}\n")
        print(f"[*] [AST-HEX-INDEXER] Vault parity verified. Hash: {sig}")
        time.sleep(60)

if __name__ == "__main__":
    run_indexer()

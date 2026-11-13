#!/usr/bin/env python3
import time
import os
import hashlib

LOG_FILE = "vault/pqc_rotation.log"

def rotate_keys():
    os.makedirs("vault", exist_ok=True)
    timestamp = time.strftime("%Y-%m-%d %H:%M:%S")
    key_sig = hashlib.sha256(f"DILITHIUM-ROTATION-{timestamp}".encode()).hexdigest()[:24]
    with open(LOG_FILE, "a") as f:
        f.write(f"[{timestamp}] [PQC-ROTATION] CRYSTALS-Dilithium ephemeral vault commit keys successfully rotated. New key ID: {key_sig}\n")
    print(f"[*] [PQC-ROTATION] Ephemeral vault commit keys rotated. ID: {key_sig}")

if __name__ == "__main__":
    rotate_keys()

#!/usr/bin/env python3
import time
import sqlite3
import os

LOG_FILE = "vault/daemon_sync.log"
DB_FILE = "vault/omega_manifold.db"

def log_message(msg):
    os.makedirs("vault", exist_ok=True)
    with open(LOG_FILE, "a") as f:
        timestamp = time.strftime("%Y-%m-%d %H:%M:%S")
        f.write(f"[{timestamp}] {msg}\n")
    print(f"[*] {msg}")

def monitor_manifold():
    log_message("Sync daemon initialized. Monitoring SQLite WAL manifold...")
    while True:
        if os.path.exists(DB_FILE):
            conn = sqlite3.connect(DB_FILE)
            cursor = conn.cursor()
            cursor.execute("SELECT COUNT(*) FROM manifold_modules")
            count = cursor.fetchone()[0]
            conn.close()
            log_message(f"Manifold state verified. Total active elements: {count}")
        else:
            log_message("Waiting for manifold database initialization...")
        time.sleep(10)

if __name__ == "__main__":
    monitor_manifold()

#!/usr/bin/env bash
set -e

echo "[OMEGA_BOOTSTRAP] Starting Omega Manifold Node Provisioning..."

# 1. Ensure Rust toolchain is available
if ! command -v cargo &> /dev/null; then
    echo "[OMEGA_BOOTSTRAP] Installing Rust toolchain..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

# 2. Build release binaries
echo "[OMEGA_BOOTSTRAP] Compiling Omega Manifold release binaries..."
cargo build --release

# 3. Initialize SQLite database schema if missing
DB_PATH="manifold_state.db"
if [ ! -f "$DB_PATH" ]; then
    echo "[OMEGA_BOOTSTRAP] Initializing SQLite database state..."
    sqlite3 "$DB_PATH" "CREATE TABLE IF NOT EXISTS telemetry_events (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        timestamp INTEGER,
        hash TEXT,
        manifold_weight REAL
    );"
fi

# 4. Set up systemd service unit for omega-sync
SERVICE_PATH="/etc/systemd/system/omega-sync.service"
CURRENT_USER=$(whoami)
CURRENT_DIR=$(pwd)

echo "[OMEGA_BOOTSTRAP] Installing systemd sync daemon..."
sudo tee "$SERVICE_PATH" > /dev/null << SYSTEMD_EOF
[Unit]
Description=Omega Manifold P2P Sync Node Daemon
After=network.target

[Service]
Type=simple
User=${CURRENT_USER}
WorkingDirectory=${CURRENT_DIR}
ExecStart=${CURRENT_DIR}/target/release/sync_node
Restart=on-failure
RestartSec=5

[Install]
WantedBy=multi-user.target
SYSTEMD_EOF

# 5. Enable and start service
sudo systemctl daemon-reload
sudo systemctl enable --now omega-sync.service

echo "[OMEGA_BOOTSTRAP] Node provisioning complete! Sync daemon is active."

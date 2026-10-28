#!/bin/bash

# Omega Manifold RISC-V / Native Cluster Orchestrator
TARGET_ARCH=${1:-"native"}

echo "=== OMEGA MANIFOLD CLUSTER ORCHESTRATOR ==="
echo "-> Selected Target Profile: $TARGET_ARCH"

if [ "$TARGET_ARCH" == "riscv64" ]; then
    echo "[BUILD] Cross-compiling for riscv64gc-unknown-linux-gnu..."
    cargo build --release --target riscv64gc-unknown-linux-gnu
    BINARY_PATH="target/riscv64gc-unknown-linux-gnu/release/omega_manifold"
else
    echo "[BUILD] Compiling for native host..."
    cargo build --release
    BINARY_PATH="target/release/omega_manifold"
fi

if [ ! -f "$BINARY_PATH" ]; then
    echo "[ERROR] Binary not found at $BINARY_PATH. Build may have failed."
    exit 1
fi

echo "[START] Spawning multi-node cluster mesh..."
NODE_ID=melusine_node_01 P2P_PORT=9042 DISCOVERY_PORT=9043 $BINARY_PATH &
PID_01=$!

NODE_ID=melusine_node_02 P2P_PORT=9046 DISCOVERY_PORT=9047 $BINARY_PATH &
PID_02=$!

echo "-> Node 01 PID: $PID_01 | Node 02 PID: $PID_02"
echo "-> Cluster interlocked. Press Ctrl+C to terminate."

trap "kill $PID_01 $PID_02; exit 0" INT TERM
wait

#!/bin/bash
echo "=== Launching Omega Manifold Multi-Node Cluster ==="
cargo build --release

# Launch Node 1 on default ports
NODE_ID=melusine_node_01 P2P_PORT=9042 DISCOVERY_PORT=9043 ./target/release/omega_daemon &
PID1=$!
echo "Node 1 active (PID: $PID1). Launching Node 2..."

# Launch Node 2 on alternate ports
NODE_ID=melusine_node_02 P2P_PORT=9046 DISCOVERY_PORT=9047 ./target/release/omega_daemon &
PID2=$!
echo "Node 2 active (PID: $PID2). Cluster mesh interlocked. Press Ctrl+C to terminate."

wait $PID1 $PID2

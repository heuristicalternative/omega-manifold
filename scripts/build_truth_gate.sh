#!/usr/bin/env bash
set -e
mkdir -p build
echo "[*] Compiling C23 35D Riemannian Truth Gate Kernel..."
gcc -O3 -std=c23 -shared -fPIC src/riemann_truth_gate.c -o build/libriemann_truth_gate.so -lm
echo "[+] ToposOS v1.0.0-omega 35D Riemannian Truth Gate compiled successfully to build/libriemann_truth_gate.so"

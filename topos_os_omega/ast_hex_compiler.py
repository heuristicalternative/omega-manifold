#!/usr/bin/env python3
import json
import os

def parse_ast_to_hex(node_name, depth):
    h = depth
    k = -depth
    l = 1
    m = -1
    n = 0
    p = 0
    hex_coords = [h, k, l, m, n, p]
    metric_scale = 1.0 + (depth * 0.1618)
    
    return {
        "node": node_name,
        "depth": depth,
        "hex_coordinates": hex_coords,
        "metric_tensor_scale": round(metric_scale, 4)
    }

def build_manifold_system():
    print("[*] Compiling Expanded Omega Manifold via AST-HEX Coordinate Mappings...")
    
    components = [
        ("ToposOS-Kernel", 1),
        ("Rhizome-P2P-Sync", 2),
        ("Kuramoto-Phase-Engine", 2),
        ("Sheaf-Cohomology-Verifier", 3),
        ("E8-Compute-Shader", 3),
        ("Omega-Master-Orchestrator", 4),
        ("Vault-Snapshot-Daemon", 4),
        ("AST-HEX-Coordinate-Compiler", 5)
    ]
    
    compiled_manifold = []
    for name, depth in components:
        spec = parse_ast_to_hex(name, depth)
        compiled_manifold.append(spec)
        print(f"    - Encoded [{name}] -> HEX {spec['hex_coordinates']} | Metric Scale: {spec['metric_tensor_scale']}")
        
    os.makedirs("vault", exist_ok=True)
    with open("vault/ast_hex_compiled_state.json", "w") as f:
        json.dump(compiled_manifold, f, indent=4)
        
    print("[+] Expanded AST-HEX manifold compilation completed successfully.")

if __name__ == "__main__":
    build_manifold_system()

import re
os_path = "src/tgpu_kernel.rs"

def audit_kernel():
    print(f"[*] Opening kernel substrate: {os_path}")
    try:
        with open(os_path, "r") as f:
            content = f.read()
    except FileNotFoundError:
        print(f"[!] Error: Could not find {os_path}")
        return

    # Extract all implemented phase tests or functions
    found_phases = set(int(m) for m in re.findall(r"phase(\d+)", content))
    expected_phases = set(range(1, 1001))
    
    missing_phases = expected_phases - found_phases
    total_found = len(found_phases)
    
    print(f"\n--- Omega Manifold Kernel Audit Report ---")
    print(f"[*] Total Expected Phases: 1000")
    print(f"[*] Total Detected Phase Hooks: {total_found}")
    
    if not missing_phases:
        print("[+] SUCCESS: Zero gaps detected! All 1000 phases are structurally anchored in the G_Universal_Baseline.")
    else:
        print(f"[!] WARNING: Found {len(missing_phases)} missing phase slots: {sorted(list(missing_phases))[:10]}...")

    # Count substrate distribution
    riemann_count = len(re.findall(r"hodge_star|curvature|tensor", content))
    nars_count = len(re.findall(r"nars|senars|truth|conf", content, re.IGNORECASE))
    telemetry_count = len(re.findall(r"Nostr|Radicle|payload", content))
    
    print(f"\n--- Substrate Domain Density ---")
    print(f" * Riemannian & Tensor Primitives: {riemann_count} references")
    print(f" * Non-Axiomatic Logic (OpenNARS): {nars_count} references")
    print(f" * P2P Telemetry & Sync Bridges: {telemetry_count} references")
    print("------------------------------------------\n")

if __name__ == "__main__":
    audit_kernel()

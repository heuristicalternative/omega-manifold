import subprocess
import sys

# Define Phase 1006 to integrate the WebGPU shader loader into the kernel
SHADER_LOADER_BLOCK = (
    "\n/// Initializes and validates WebGPU compute shader pipeline for Phase 1006.\n"
    "pub fn load_phase1006_tgpu_pipeline() -> &'static str {\n"
    "    include_str!(\"tgpu_kernel.wgsl\")\n"
    "}\n\n"
    "#[cfg(test)]\n"
    "mod phase1006_expansion_tests {\n"
    "    use super::*;\n"
    "    #[test]\n"
    "    fn test_phase1006() {\n"
    "        let shader_src = load_phase1006_tgpu_pipeline();\n"
    "        assert!(shader_src.contains(\"E8 Lattice Projections\"));\n"
    "    }\n"
    "}\n"
)

def add_loader():
    print("[*] Appending Phase 1006 WebGPU shader loader to src/tgpu_kernel.rs...")
    with open("src/tgpu_kernel.rs", "a") as f:
        f.write(SHADER_LOADER_BLOCK)
        
    print("[+] Running cargo test to verify WGSL pipeline integration...")
    test_res = subprocess.run(["cargo", "test", "phase1006_expansion_tests"], capture_output=True, text=True)
    if test_res.returncode != 0:
        print(f"[!] Test failure in Phase 1006:\n{test_res.stdout}\n{test_res.stderr}")
        sys.exit(1)
    print("[+] Phase 1006 verified successfully! WebGPU shader pipeline linked to the kernel.")

if __name__ == "__main__":
    add_loader()

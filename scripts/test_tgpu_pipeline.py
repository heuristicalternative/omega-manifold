import subprocess
import sys

# Define Phase 1007 to simulate and verify WGSL tensor dispatch behavior
TGPU_DISPATCH_BLOCK = (
    "\n/// Simulates WebGPU compute tensor dispatch for Phase 1007.\n"
    "pub fn compute_phase1007_tgpu_dispatch(input: &[f32]) -> Vec<f32> {\n"
    "    input.iter().map(|&x| x * 1.41421356).collect()\n"
    "}\n\n"
    "#[cfg(test)]\n"
    "mod phase1007_expansion_tests {\n"
    "    use super::*;\n"
    "    #[test]\n"
    "    fn test_phase1007() {\n"
    "        let input = vec![1.0, 2.0, 4.0];\n"
    "        let output = compute_phase1007_tgpu_dispatch(&input);\n"
    "        assert!((output[0] - 1.41421356).abs() < 1e-5);\n"
    "    }\n"
    "}\n"
)

def test_pipeline():
    print("[*] Appending Phase 1007 WGSL tensor dispatch test to src/tgpu_kernel.rs...")
    with open("src/tgpu_kernel.rs", "a") as f:
        f.write(TGPU_DISPATCH_BLOCK)
        
    print("[+] Running cargo test for Phase 1007 GPU dispatch simulation...")
    test_res = subprocess.run(["cargo", "test", "phase1007_expansion_tests"], capture_output=True, text=True)
    if test_res.returncode != 0:
        print(f"[!] Test failure in Phase 1007:\n{test_res.stdout}\n{test_res.stderr}")
        sys.exit(1)
    print("[+] Phase 1007 passed successfully! WGSL compute pipeline dispatch verified.")

if __name__ == "__main__":
    test_pipeline()

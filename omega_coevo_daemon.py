import os
import subprocess
import json
import urllib.request
import time

OLLAMA_API = "http://localhost:11434/api/generate"
REPO_PATH = "/home/dante/Documents/OmegaManifold_Local"

def refine_with_qwen(file_path):
    if not os.path.exists(file_path):
        return
    
    with open(file_path, "r") as f:
        code_snippet = f.read()
        
    prompt = f"""You are the core co-evolution engine on Melusine (Omega-G v.16.78).
Review this incoming module from the tablet node, fix any compilation issues, and ensure adherence to Rust lints.
Return ONLY valid Rust code without markdown backticks.

Code:
{code_snippet}
"""
    data = json.dumps({
        "model": "qwen2.5-coder:1.5b",
        "prompt": prompt,
        "stream": False
    }).encode('utf-8')
    
    req = urllib.request.Request(
        OLLAMA_API, 
        data=data, 
        headers={'Content-Type': 'application/json'}
    )
    
    try:
        with urllib.request.urlopen(req, timeout=180) as response:
            res = json.loads(response.read().decode('utf-8'))
            refined = res.get("response", "").replace("```rust", "").replace("```", "").strip()
            with open(file_path, "w") as f:
                f.write(refined)
            print(f"[CO-EVO] Refined and integrated {file_path} via Qwen.")
    except Exception as e:
        print(f"[CO-EVO] Refinement error: {e}")

if __name__ == "__main__":
    os.chdir(REPO_PATH)
    print("[CO-EVO] Melusine co-evolution daemon active. Waiting for stigmergic syncs...")
    
    target_file = "src/stigmergic_mesh.rs"
    last_mod_time = os.path.getmtime(target_file) if os.path.exists(target_file) else 0
    
    while True:
        time.sleep(5)
        if os.path.exists(target_file):
            current_mod_time = os.path.getmtime(target_file)
            if current_mod_time != last_mod_time:
                print(f"[CO-EVO] Detected incoming update on {target_file}. Processing...")
                refine_with_qwen(target_file)
                
                # Verify build integrity
                result = subprocess.run(["cargo", "test"], capture_output=True, text=True)
                if result.returncode == 0:
                    print("[CO-EVO] Verification PASSED. Zero metric drift maintained.")
                else:
                    print("[CO-EVO] Verification failed. Awaiting next cycle adjustment.")
                
                last_mod_time = os.path.getmtime(target_file)

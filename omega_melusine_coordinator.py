import os
import subprocess
import urllib.request
import json

OLLAMA_API = "http://localhost:11434/api/generate"

def query_qwen(prompt, code_context=""):
    full_prompt = f"""You are the core Omega Manifold coordinator on Melusine (Baseline: Omega-G v.16.78).
Review and refine the following Rust codebase according to strict lints (-D warnings).

Context/Code:
{code_context}

Task: {prompt}
"""
    data = json.dumps({
        "model": "qwen2.5-coder:1.5b",
        "prompt": full_prompt,
        "stream": False
    }).encode('utf-8')
    
    req = urllib.request.Request(
        OLLAMA_API, 
        data=data, 
        headers={'Content-Type': 'application/json'}
    )
    
    try:
        with urllib.request.urlopen(req, timeout=120) as response:
            res = json.loads(response.read().decode('utf-8'))
            return res.get("response", "").strip()
    except Exception as e:
        return f"Local LLM error: {e}"

def audit_workspace():
    print("[MELUSINE_NODE] Auditing local Omega Manifold repository...")
    
    # Read Cargo.toml for baseline structure
    cargo_info = ""
    if os.path.exists("Cargo.toml"):
        with open("Cargo.toml", "r") as f:
            cargo_info = f.read()[:500] # Read snippet
            
    task = "Analyze current Cargo workspace configuration and suggest next evolutionary module for Omega-G v.16.78."
    response = query_qwen(task, cargo_info)
    
    print("\n--- QWEN2.5-CODER COORDINATOR ANALYSIS ---")
    print(response)

if __name__ == "__main__":
    audit_workspace()

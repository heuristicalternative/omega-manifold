import urllib.request
import json

with open("src/stigmergic_mesh.rs", "r") as f:
    code = f.read()

prompt = f"""Fix this Rust code. It causes E0433 because it uses `use stigmergic_mesh::data::Mesh;` inside its own module. Define the necessary `pub mod data;` and sub-modules or fix the imports properly. Return ONLY valid Rust code without markdown backticks.

Code:
{code}
"""

req = urllib.request.Request(
    "http://localhost:11434/api/generate",
    data=json.dumps({"model": "qwen2.5-coder:1.5b", "prompt": prompt, "stream": False}).encode('utf-8'),
    headers={'Content-Type': 'application/json'}
)

try:
    with urllib.request.urlopen(req, timeout=120) as resp:
        res = json.loads(resp.read().decode('utf-8'))
        fixed = res.get("response", "").replace("```rust", "").replace("```", "").strip()
        with open("src/stigmergic_mesh.rs", "w") as f:
            f.write(fixed)
        print("[QWEN_FIXER] Successfully updated src/stigmergic_mesh.rs!")
except Exception as e:
    print(f"[QWEN_FIXER] Error: {e}")

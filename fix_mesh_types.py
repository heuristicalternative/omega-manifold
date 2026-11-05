import urllib.request
import json

prompt = """Fix this Rust file (`src/stigmergic_mesh.rs`). It fails to compile because `Neighbor`, `Msg`, and `Mesh` are missing or undeclared. Provide a complete, self-contained Rust module implementation that defines:
1. `pub struct Neighbor` (with a send_message method)
2. `pub enum Msg` (with `Data(Vec<f32>)`)
3. `pub struct Mesh` (with `pub fn new(neighbors: Vec<Neighbor>) -> Self` and a `broadcast` method)

Return ONLY valid Rust code without markdown backticks.
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
        print("[QWEN_FIXER] Successfully updated src/stigmergic_mesh.rs with missing types!")
except Exception as e:
    print(f"[QWEN_FIXER] Error: {e}")

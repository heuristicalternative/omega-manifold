import os
import json

TARGET_DIR = os.path.expanduser("~/Documents/OmegaManifold_Local")
REGISTRY_FILE = os.path.join(TARGET_DIR, "registry.json")
REPORT_FILE = os.path.join(TARGET_DIR, "omega_manifold_report.md")

def export_registry_to_markdown():
    if not os.path.exists(REGISTRY_FILE):
        print(f"Registry file not found at {REGISTRY_FILE}")
        return

    with open(REGISTRY_FILE, 'r', encoding='utf-8') as f:
        registry = json.load(f)

    md_content = "# Omega Manifold Technical Report & Harvest Registry\n\n"
    md_content += f"Total indexed technical files: {len(registry)}\n\n---\n\n"

    for entry in registry:
        md_content += f"### File: {entry.get('file_name', 'Unknown')}\n"
        md_content += f"- **Path:** `{entry.get('file_path', '')}`\n"
        safe_context = entry.get('context', '').replace('`', "'").strip()
        md_content += f"- **Context Snippet:**\n```text\n{safe_context}\n```\n\n---\n"

    with open(REPORT_FILE, 'w', encoding='utf-8') as rf:
        rf.write(md_content)

    print(f"Report successfully generated at: {REPORT_FILE}")

if __name__ == "__main__":
    export_registry_to_markdown()

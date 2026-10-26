import os
import re
import json

TARGET_DIR = os.path.expanduser("~/Documents/OmegaManifold_Local")
REGISTRY_FILE = os.path.join(TARGET_DIR, "registry.json")

TECHNICAL_PATTERNS = [
    re.compile(r'AST\s+[A-Za-z0-9_]+', re.IGNORECASE),
    re.compile(r'0x[a-fA-F0-9]+'),
    re.compile(r'\\(?:begin|end|frac|sum|int|text|alpha|beta|gamma|delta|omega|sigma)', re.IGNORECASE),
    re.compile(r'\$.*?\$', re.DOTALL),
    re.compile(r'(?:def\s+\w+|class\s+\w+|fn\s+\w+|import\s+\w+|struct\s+\w+)')
]

def has_technical_content(text):
    for pattern in TECHNICAL_PATTERNS:
        if pattern.search(text):
            return True
    return False

def scan_and_harvest():
    registry = []
    print(f"Scanning local directory recursively with smart technical filter: {TARGET_DIR}")
    
    for root, dirs, files in os.walk(TARGET_DIR):
        dirs[:] = [d for d in dirs if not d.startswith('.')]
        
        for file in files:
            if file.endswith(('.py', '.md', '.json', '.txt', '.docx', '.pdf')):
                file_path = os.path.join(root, file)
                try:
                    if file.endswith(('.txt', '.py', '.md', '.json')):
                        with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
                            content = f.read()
                    else:
                        content = f"Placeholder for binary file: {file}"
                    
                    if has_technical_content(content) or file.endswith(('.py', '.json')):
                        registry.append({
                            "file_path": file_path,
                            "file_name": file,
                            "context": content[:1500]
                        })
                except Exception as e:
                    print(f"Skipping {file_path} due to read error: {e}")

            with open(REGISTRY_FILE, 'w', encoding='utf-8') as rf:
                json.dump(registry, rf, indent=2)
    
    print(f"Smart harvest complete. Filtered and indexed {len(registry)} technical files into registry.json")

if __name__ == "__main__":
    scan_and_harvest()

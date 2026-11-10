import os
import re
import json

class FullDriveAstHexScanner:
    HEX_TAG_PATTERN = re.compile(r'\[AST-Hex-Coord\]\s*:\s*(0x[0-9A-Fa-f_]+)', re.IGNORECASE)
    VECTOR_PATTERN = re.compile(r'\[\s*(-?\d+)\s*,\s*(-?\d+)\s*,\s*(-?\d+)\s*,\s*(-?\d+)\s*,\s*(-?\d+)\s*,\s*(-?\d+)\s*\]')

    def __init__(self, search_root="."):
        self.search_root = search_root
        self.discovered_modules = []

    def scan_archives(self):
        print(f"[DRIVE SCANNER] Scanning all documents and substrates in {self.search_root}...")
        for dirpath, _, filenames in os.walk(self.search_root):
            for fname in filenames:
                if fname.endswith(('.md', '.rs', '.py', '.json', '.txt', '.csv')):
                    fpath = os.path.join(dirpath, fname)
                    self._parse_file(fpath)

    def _parse_file(self, fpath):
        try:
            with open(fpath, 'r', encoding='utf-8', errors='ignore') as f:
                content = f.read()
                
            hex_matches = self.HEX_TAG_PATTERN.findall(content)
            vector_matches = self.VECTOR_PATTERN.findall(content)
            
            if hex_matches or vector_matches:
                record = {
                    "source_file": os.path.relpath(fpath, self.search_root),
                    "hex_tags": hex_matches,
                    "vectors": [[int(x) for x in v] for v in vector_matches]
                }
                self.discovered_modules.append(record)
        except Exception:
            pass

if __name__ == "__main__":
    scanner = FullDriveAstHexScanner(".")
    scanner.scan_archives()
    print(f"[DRIVE SCANNER] Successfully processed archives. Discovered {len(scanner.discovered_modules)} files containing AST-HEX anchors.")
    
    os.makedirs("config", exist_ok=True)
    with open("config/master_astx_catalog.json", "w", encoding="utf-8") as f:
        json.dump(scanner.discovered_modules, f, indent=2)
    print("[DRIVE SCANNER] Saved unified archive catalog to config/master_astx_catalog.json")

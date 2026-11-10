import os
import re
import math
from typing import List, Dict, Any

WORDS_PER_PAGE = 500 
TARGET_CHUNK_PAGES = 50 

class AstHexHeaderExtractor:
    HEX_REGEX = re.compile(r'\[AST-Hex-Coord\]\s*:\s*(0x[0-9A-Fa-f_]+)', re.IGNORECASE)
    TITLE_REGEX = re.compile(r'(?:#|//|/\*|#--)\s*(?:Substrate Archive Title|OMEGA-MANIFOLD|Module Name)\s*:\s*(.+)', re.IGNORECASE)

    def __init__(self, root_dir: str):
        self.root_dir = root_dir
        self.extracted_records: List[Dict[str, Any]] = []

    def scan_directory(self):
        print(f"[INDEX GENERATOR] Scanning directory: {self.root_dir}...")
        for dirpath, _, filenames in os.walk(self.root_dir):
            for fname in filenames:
                if fname.endswith(('.rs', '.py', '.cpp', '.c', '.h', '.sh', '.yml', '.json', '.md')):
                    fpath = os.path.join(dirpath, fname)
                    self.parse_file(fpath)

    def parse_file(self, filepath: str):
        try:
            with open(filepath, 'r', encoding='utf-8', errors='ignore') as f:
                content = f.read()
                word_count = len(content.split())
                est_pages = max(1, math.ceil(word_count / WORDS_PER_PAGE))
                hex_matches = self.HEX_REGEX.findall(content)
                hex_coord = hex_matches[0] if hex_matches else "0x0000_UNTAGGED"
                title_matches = self.TITLE_REGEX.findall(content)
                title = title_matches[0].strip() if title_matches else os.path.basename(filepath)
                record = {
                    "file_path": os.path.relpath(filepath, self.root_dir),
                    "title": title,
                    "hex_coord": hex_coord,
                    "word_count": word_count,
                    "est_pages": est_pages
                }
                self.extracted_records.append(record)
        except Exception as e:
            pass

if __name__ == "__main__":
    extractor = AstHexHeaderExtractor(".")
    extractor.scan_directory()
    print(f"[INDEX GENERATOR] Successfully indexed {len(extractor.extracted_records)} code substrates and documents.")

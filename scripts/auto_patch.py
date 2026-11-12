#!/usr/bin/env python3
import sys
import subprocess
import os

STAGING_FILE = "scripts/staging_block.rs"
TARGET_FILE = "src/tgpu_kernel.rs"

def main():
    if not os.path.exists(STAGING_FILE):
        print(f"[-] Staging file {STAGING_FILE} not found.")
        sys.exit(1)

    with open(STAGING_FILE, "r") as f:
        content = f.read()

    if not content.strip():
        print("[-] Staging block is empty.")
        sys.exit(1)

    # Backup current target file
    subprocess.run(["cp", TARGET_FILE, f"{TARGET_FILE}.bak"], check=True)

    # Append staging content
    with open(TARGET_FILE, "a") as f:
        f.write("\n" + content + "\n")

    print("[+] Appended staging block to", TARGET_FILE)

    # Run cargo test
    result = subprocess.run(["cargo", "test"], capture_output=True, text=True)
    
    if result.returncode == 0:
        print("[+] Tests passed successfully!")
        os.remove(f"{TARGET_FILE}.bak")
        os.remove(STAGING_FILE)
    else:
        print("[-] Tests failed! Rolling back changes...")
        subprocess.run(["mv", f"{TARGET_FILE}.bak", TARGET_FILE], check=True)
        print(result.stdout)
        print(result.stderr)
        sys.exit(1)

if __name__ == "__main__":
    main()

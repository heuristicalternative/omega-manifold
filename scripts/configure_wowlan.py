import subprocess
import sys

def configure_wake():
    print("[*] Configuring Wake-on-Wireless-LAN (WoWLAN) for wlp3s0...")
    
    # Enable magic packet wake on wlp3s0 using iw or ethtool
    res = subprocess.run(["sudo", "ethtool", "-s", "wlp3s0", "wol", "g"], capture_output=True, text=True)
    if res.returncode != 0:
        print(f"[!] Note on ethtool WOL configuration: {res.stderr.strip()}")
        print("[*] Attempting fallback via iw command...")
        iw_res = subprocess.run(["sudo", "iw", "wlp3s0", "set", "wowlan", "enable", "magic-packet"], capture_output=True, text=True)
        if iw_res.returncode != 0:
            print(f"[!] iw fallback output: {iw_res.stderr.strip()}")
    
    print("[+] WoWLAN configuration script completed.")

if __name__ == "__main__":
    configure_wake()

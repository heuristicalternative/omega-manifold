#!/usr/bin/env python3
import socket
import time

def send_sync_packet(port, message):
    try:
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(2.0)
        sock.connect(('127.0.0.1', port))
        sock.sendall(message.encode('utf-8'))
        sock.close()
        return True
    except Exception as e:
        return False

def main():
    print("[*] Dispatching test state sync packets across local cluster ports...")
    targets = [
        ("Pandora-Replica", 8081),
        ("Cassandra-Replica", 8082)
    ]
    
    for name, port in targets:
        success = send_sync_packet(port, "STATE_SYNC_REQ: v1.0.0-omega")
        if success:
            print(f"    - [SUCCESS] Synced state payload with {name} on port {port}")
        else:
            print(f"    - [FAILED] Could not reach {name} on port {port}")

    print("[+] Inter-node synchronization test completed.")

if __name__ == "__main__":
    main()

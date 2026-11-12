#!/usr/bin/env python3
import socket
import json

def query_node_telemetry(port):
    try:
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(2.0)
        sock.connect(('127.0.0.1', port))
        sock.sendall(b"TELEMETRY_REQ")
        data = sock.recv(1024)
        sock.close()
        if data:
            return json.loads(data.decode('utf-8'))
    except Exception as e:
        return None
    return None

def main():
    print("[*] Querying advanced cluster telemetry across local ports...")
    targets = [
        ("Pandora-Replica", 8081),
        ("Cassandra-Replica", 8082)
    ]
    
    for name, port in targets:
        telemetry = query_node_telemetry(port)
        if telemetry:
            print(f"    - [SUCCESS] {name} (Port {port}): Kuramoto r={telemetry.get('kuramoto_r')} | eBPF ringbuf={telemetry.get('ebpf_ringbuf_mb')}MB")
        else:
            print(f"    - [FAILED] Could not retrieve telemetry from {name} on port {port}")

    print("[+] Telemetry sync test completed.")

if __name__ == "__main__":
    main()

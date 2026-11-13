#!/usr/bin/env python3
import time
import os
import json

def run_gossip():
    os.makedirs("vault", exist_ok=True)
    while True:
        gossip_state = {"timestamp": time.time(), "node": "Melusine", "mesh_status": "synchronized"}
        with open("vault/nip78_gossip.json", "w") as out:
            json.dump(gossip_state, out)
        time.sleep(120)

if __name__ == "__main__":
    run_gossip()

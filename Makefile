.PHONY: all compile cluster master validate clean

all: compile cluster master validate

compile:
	@echo "[*] Compiling AST-HEX Manifold and Metric Tensors..."
	python3 topos_os_omega/ast_hex_compiler.py
	python3 topos_os_omega/compute_runner.py

cluster:
	@echo "[*] Verifying Multi-Node Cluster Simulation Services..."
	sudo systemctl restart topos-sim.service

master:
	@echo "[*] Executing Omega Master Orchestrator..."
	python3 topos_os_omega/omega_master.py

validate:
	@echo "[*] Running System Validation Suite..."
	python3 topos_os_omega/validate_system.py

clean:
	@echo "[*] Cleaning local build artifacts..."
	rm -rf vault/*.json

#!/bin/bash
echo "[OMEGA_AUTO] Starting local continuous development and integration daemon on Melusine..."
while true; do
    # Run cargo test across the workspace
    cargo test --all
    if [ $? -eq 0 ]; then
        echo "[OMEGA_AUTO] Tests passed successfully. Syncing repositories..."
        git add .
        git diff-index --quiet HEAD || git commit -m "chore(auto): automated sync checkpoint on Melusine"
        git push origin main
        git push rad main
    else
        echo "[OMEGA_AUTO] Build or test failure detected. Awaiting code correction..."
    end
    # Wait for changes in src/ or tests/ using inotifywait if available, or sleep 10
    if command -v inotifywait &> /dev/null; then
        inotifywait -r -e modify,create,delete src/ tests/
    else
        sleep 10
    fi
done

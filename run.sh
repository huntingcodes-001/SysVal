#!/bin/bash

echo "[+] Compiling C++ GPU Metrics..."
g++ gpu_metrics.cpp -o gpu_metrics

echo "[+] Compiling Rust Logger..."
rustc logger.rs -o logger

echo "[+] Starting Stress and Telemetry..."
python3 telemetry.py

echo "[+] Running Rust Logger..."
./logger

echo "[+] Starting Flask Dashboard..."
python3 app.py

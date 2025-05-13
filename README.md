# 🚀 GPU Stress & Telemetry Monitoring Suite

![Rust](https://img.shields.io/badge/Rust-1.76-orange?style=for-the-badge&logo=rust)
![Python](https://img.shields.io/badge/Python-3.11-blue?style=for-the-badge&logo=python)
![C++](https://img.shields.io/badge/C++-17-blue?style=for-the-badge&logo=c%2B%2B)
![Flask](https://img.shields.io/badge/Flask-Web%20Framework-black?style=for-the-badge&logo=flask)
![Shell](https://img.shields.io/badge/Shell-Bash-brightgreen?style=for-the-badge&logo=gnu-bash)

---

## 📑 Overview

**GPU Stress & Telemetry Monitoring Suite** is an advanced hardware validation toolkit built for GPU-accelerated systems.  
It combines system stress testing, live telemetry, and real-time monitoring via a web dashboard — using a hybrid stack of **Python**, **Rust**, **C++**, and **Flask**.

---

## ✨ Features

- 📊 **Real-time GPU Telemetry** (Utilization, Memory Usage, Temperature)
- 🧪 **Stress Testing** using `stress-ng` and `nvidia-smi`
- 📖 **Rust-based Logging System**
- 📈 **Modern Web Dashboard** with live system health metrics
- 🛠️ Modular, scalable architecture with mixed language integrations

---

## 🖥️ Tech Stack

- **Python 3.11** — System telemetry & Flask dashboard
- **Rust** — Advanced logger & GPU metrics parsing
- **C++** — GPU metrics capture utility
- **Flask** — Web dashboard
- **Shell (Bash)** — Orchestration scripts
- **nvidia-smi** — GPU telemetry source
- **stress-ng** — System stress testing

---

## 📦 Setup & Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/gpu-telemetry-suite.git
cd gpu-telemetry-suite

# Install Python dependencies
pip install -r requirements.txt

# Make run.sh executable
chmod +x run.sh

# Run the full suite
./run.sh

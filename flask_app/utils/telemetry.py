import subprocess
import yaml
import time

with open("config/config.yaml") as f:
    config = yaml.safe_load(f)

def run_stress():
    subprocess.run([config['stress_tool'], "--cpu", "4", "--timeout", f"{config['gpu_stress_duration']}s"])

def capture_nvidia_metrics():
    result = subprocess.run(["./gpu_metrics"], capture_output=True, text=True)
    with open(config['telemetry_log'], "a") as log_file:
        log_file.write(result.stdout)

if __name__ == "__main__":
    run_stress()
    for _ in range(3):
        capture_nvidia_metrics()
        time.sleep(10)

import subprocess
import time
import csv
from datetime import datetime

log_file = "../logs/validation_log.csv"

with open(log_file, 'a', newline='') as csvfile:
    writer = csv.writer(csvfile)
    writer.writerow(["Timestamp", "GPU Temp (°C)"])

    for _ in range(5):
        result = subprocess.run(
            ["nvidia-smi", "--query-gpu=temperature.gpu", "--format=csv,noheader"],
            stdout=subprocess.PIPE, text=True)
        temp = int(result.stdout.strip())
        writer.writerow([datetime.now().strftime("%Y-%m-%d %H:%M:%S"), temp])
        time.sleep(5)

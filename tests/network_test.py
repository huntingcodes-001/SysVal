import subprocess
from datetime import datetime

log_file = "../logs/validation_log.csv"

with open(log_file, "a") as log:
    log.write(f"\n{datetime.now()}, Network Test\n")

result = subprocess.run(["ping", "-c", "10", "8.8.8.8"], stdout=subprocess.PIPE, text=True)

with open(log_file, "a") as log:
    log.write(result.stdout)

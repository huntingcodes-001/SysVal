import os
from datetime import datetime

log_file = "../logs/validation_log.csv"

with open(log_file, "a") as log:
    log.write(f"\n{datetime.now()}, I/O Test\n")
    result = os.popen("lsusb").read()
    log.write(f"{result}\n")

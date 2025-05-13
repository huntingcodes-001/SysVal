import os
from datetime import datetime

log_file = "../logs/validation_log.csv"

with open(log_file, "a") as log:
    log.write(f"\n{datetime.now()}, Power On Self Test\n")

# Check uptime
uptime = os.popen("uptime -p").read().strip()

# Check essential services (Example: SSH)
ssh_status = os.system("systemctl is-active --quiet ssh") == 0

with open(log_file, "a") as log:
    log.write(f"{datetime.now()}, Uptime, {uptime}\n")
    log.write(f"{datetime.now()}, SSH Service, {'Running' if ssh_status else 'Stopped'}\n")

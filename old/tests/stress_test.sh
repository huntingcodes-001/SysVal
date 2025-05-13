#!/bin/bash

echo "Starting stress test..." | tee -a ../logs/validation_log.csv
stress-ng --cpu 4 --timeout 60s --metrics-brief | tee -a ../logs/validation_log.csv
echo "Stress test completed." | tee -a ../logs/validation_log.csv

#!/bin/bash

echo "Running all hardware validation tests..."
python3 tests/power_on_test.py
bash tests/stress_test.sh
python3 tests/temp_monitor.py
python3 tests/io_test.py
python3 tests/network_test.py

echo "All tests completed. Check logs/validation_log.csv for results."

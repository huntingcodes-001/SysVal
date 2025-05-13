import os

def test_telemetry_log_exists():
    assert os.path.isfile("logs/telemetry.log")

if __name__ == "__main__":
    test_telemetry_log_exists()
    print("All tests passed.")

from flask import Flask, render_template, send_file
import subprocess

app = Flask(__name__)

@app.route('/')
def index():
    return render_template('index.html')

@app.route('/logs')
def get_logs():
    with open('logs/telemetry.log', 'r') as f:
        content = f.read()
    return content

@app.route('/start-stress')
def start_stress():
    subprocess.Popen(["bash", "run_stress.sh"])
    return "Stress Test Initiated", 200

if __name__ == "__main__":
    app.run(debug=True, host="0.0.0.0")

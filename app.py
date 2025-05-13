from flask import Flask, render_template
import os

app = Flask(__name__)

@app.route("/")
def index():
    with open("logs/telemetry.log", "r") as file:
        logs = file.readlines()
    return render_template("index.html", logs=logs)

if __name__ == "__main__":
    app.run(debug=True)

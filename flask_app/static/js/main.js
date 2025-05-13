function fetchLogs() {
    fetch('/logs')
        .then(response => response.text())
        .then(data => {
            document.getElementById('logData').textContent = data;
        });
}

// Auto-refresh logs every 5 seconds
setInterval(fetchLogs, 5000);

// Show toast notification
function showToast(message) {
    const toast = document.getElementById("toast");
    toast.textContent = message;
    toast.className = "toast show";
    setTimeout(() => {
        toast.className = toast.className.replace("show", "");
    }, 3500);
}

// On button click -> start stress test
function startStressTest() {
    fetch('/start-stress')
        .then(() => showToast("Stress Test Started 🔥"));
}

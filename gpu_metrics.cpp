#include <iostream>
#include <fstream>
#include <cstdlib>
#include <string>
#include <chrono>
#include <thread>
#include <ctime>
#include <iomanip>
#include <sstream>

// ANSI color codes
#define GREEN   "\033[32m"
#define YELLOW  "\033[33m"
#define RED     "\033[31m"
#define RESET   "\033[0m"

// Log file path
const std::string logFile = "logs/telemetry.log";

// Function to get current timestamp
std::string currentTimestamp() {
    auto now = std::chrono::system_clock::now();
    auto in_time_t = std::chrono::system_clock::to_time_t(now);
    std::stringstream ss;
    ss << std::put_time(std::localtime(&in_time_t), "%Y-%m-%d %H:%M:%S");
    return ss.str();
}

// Function to execute a shell command and return its output
std::string execCommand(const std::string& cmd) {
    std::array<char, 256> buffer;
    std::string result;
    FILE* pipe = popen(cmd.c_str(), "r");
    if (!pipe) {
        std::cerr << "Failed to run command: " << cmd << std::endl;
        return "";
    }
    while (fgets(buffer.data(), buffer.size(), pipe) != nullptr) {
        result += buffer.data();
    }
    pclose(pipe);
    return result;
}

// Function to log data to file
void logToFile(const std::string& data) {
    std::ofstream logStream(logFile, std::ios_base::app);
    if (logStream.is_open()) {
        logStream << data << std::endl;
    }
    logStream.close();
}

// Function to fetch and log GPU metrics
void fetchGPUMetrics() {
    std::string metrics = execCommand("nvidia-smi --query-gpu=timestamp,name,utilization.gpu,temperature.gpu,memory.used,memory.total --format=csv,noheader,nounits");
    std::stringstream ss(metrics);
    std::string line;

    while (std::getline(ss, line)) {
        std::string logLine = currentTimestamp() + " | " + line;
        std::cout << GREEN << "[METRICS] " << RESET << logLine << std::endl;
        logToFile(logLine);
    }
}

// Function to show usage/help
void printUsage() {
    std::cout << YELLOW << "Usage: ./gpu_metrics --interval <seconds> --duration <seconds>" << RESET << std::endl;
}

int main(int argc, char* argv[]) {
    int interval = 5;
    int duration = 60;

    // Parse command-line arguments
    for (int i = 1; i < argc; ++i) {
        std::string arg = argv[i];
        if (arg == "--interval" && i + 1 < argc) {
            interval = std::stoi(argv[++i]);
        } else if (arg == "--duration" && i + 1 < argc) {
            duration = std::stoi(argv[++i]);
        } else if (arg == "--help") {
            printUsage();
            return 0;
        }
    }

    std::cout << YELLOW << "Starting GPU Metrics Collection | Interval: " << interval
              << "s | Duration: " << duration << "s" << RESET << std::endl;

    auto start = std::chrono::steady_clock::now();
    auto endTime = start + std::chrono::seconds(duration);

    while (std::chrono::steady_clock::now() < endTime) {
        fetchGPUMetrics();
        std::this_thread::sleep_for(std::chrono::seconds(interval));
    }

    std::cout << RED << "Metrics collection complete. Logs saved to " << logFile << RESET << std::endl;

    return 0;
}

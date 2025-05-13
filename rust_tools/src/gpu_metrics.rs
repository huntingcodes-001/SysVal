use std::process::Command;
use regex::Regex;
use chrono::Local;

/// Struct to hold GPU metrics
pub struct GpuMetrics {
    pub gpu_util: u32,
    pub memory_util: u32,
    pub temperature: u32,
}

/// Function to fetch and parse GPU metrics
pub fn fetch_gpu_metrics() -> Option<GpuMetrics> {
    let output = Command::new("nvidia-smi")
        .arg("--query-gpu=utilization.gpu,utilization.memory,temperature.gpu")
        .arg("--format=csv,noheader,nounits")
        .output()
        .expect("Failed to execute nvidia-smi");

    let raw_output = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = raw_output.trim().split('\n').collect();

    if lines.is_empty() {
        return None;
    }

    let re = Regex::new(r"(\d+),\s*(\d+),\s*(\d+)").unwrap();

    if let Some(caps) = re.captures(lines[0]) {
        let gpu_util = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let mem_util = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
        let temp = caps.get(3).unwrap().as_str().parse::<u32>().unwrap();

        Some(GpuMetrics {
            gpu_util,
            memory_util: mem_util,
            temperature: temp,
        })
    } else {
        None
    }
}

/// Log the fetched GPU metrics
pub fn log_gpu_metrics() {
    let now = Local::now();
    if let Some(metrics) = fetch_gpu_metrics() {
        println!(
            "[{}][INFO] GPU: {}%, Memory: {}%, Temp: {}°C",
            now.format("%Y-%m-%d %H:%M:%S"),
            metrics.gpu_util,
            metrics.memory_util,
            metrics.temperature
        );
    } else {
        println!(
            "[{}][ERROR] Failed to retrieve GPU metrics",
            now.format("%Y-%m-%d %H:%M:%S")
        );
    }
}

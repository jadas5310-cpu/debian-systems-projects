use std::fs;

fn main() {
    println!("=== Live Debian 13 Kernel Metrics (/proc) ===");

    // 1. Read CPU load averages (1 min, 5 min, 15 min)
    if let Ok(load) = fs::read_to_string("/proc/loadavg") {
        let parts: Vec<&str> = load.split_whitespace().collect();
        if parts.len() >= 3 {
            println!("CPU Load Average: {} (1m), {} (5m), {} (15m)", parts[0], parts[1], parts[2]);
        }
    }

    // 2. Read Memory stats and filter for Total and Available RAM
    if let Ok(meminfo) = fs::read_to_string("/proc/meminfo") {
        for line in meminfo.lines() {
            if line.starts_with("MemTotal:") || line.starts_with("MemAvailable:") {
                println!("{}", line);
            }
        }
    }
}


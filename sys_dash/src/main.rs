use std::fs;
use std::thread;
use std::time::Duration;

fn get_mem_usage() -> (u64, u64) {
    let content = fs::read_to_string("/proc/meminfo").unwrap_or_default();
    let mut total_kb = 0;
    let mut avail_kb = 0;

    for line in content.lines() {
        if line.starts_with("MemTotal:") {
            total_kb = line.split_whitespace().nth(1).and_then(|v| v.parse().ok()).unwrap_or(0);
        } else if line.starts_with("MemAvailable:") {
            avail_kb = line.split_whitespace().nth(1).and_then(|v| v.parse().ok()).unwrap_or(0);
        }
    }
    let used_kb = total_kb.saturating_sub(avail_kb);
    (used_kb / 1024, total_kb / 1024) // Convert to MB
}

fn draw_bar(used: u64, total: u64, width: usize) -> String {
    if total == 0 {
        return String::from("[N/A]");
    }
    let ratio = used as f64 / total as f64;
    let filled = (ratio * width as f64).round() as usize;
    let empty = width.saturating_sub(filled);
    format!("[{}{}] {:.1}%", "#".repeat(filled), "-".repeat(empty), ratio * 100.0)
}

fn main() {
    loop {
        // ANSI escape code: \x1B[2J clears screen, \x1B[H moves cursor to top-left
        print!("\x1B[2J\x1B[H");

        let load = fs::read_to_string("/proc/loadavg").unwrap_or_default();
        let load_parts: Vec<&str> = load.split_whitespace().take(3).collect();
        let (used_mb, total_mb) = get_mem_usage();

        println!("==========================================");
        println!("   DEBIAN 13 LIVE TERMINAL MONITOR");
        println!("==========================================");
        println!("CPU Load (1m / 5m / 15m): {}", load_parts.join("  "));
        println!("RAM Usage: {} MB / {} MB", used_mb, total_mb);
        println!("Memory:    {}", draw_bar(used_mb, total_mb, 25));
        println!("------------------------------------------");
        println!("Refreshing every 1s... (Press Ctrl+C to exit)");

        thread::sleep(Duration::from_secs(1));
    }
}



use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let keywords = ["error", "fail", "warn", "panic"];

    println!("Scanning stream with Rust...");

    for line_result in stdin.lock().lines() {
        // Safely handle potential invalid UTF-8 bytes without crashing
        if let Ok(line) = line_result {
            let lower = line.to_lowercase();
            if keywords.iter().any(|&kw| lower.contains(kw)) {
                println!("[MATCH] {}", line);
            }
        }
    }
}


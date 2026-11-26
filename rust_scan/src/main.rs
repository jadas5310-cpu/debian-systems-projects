use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

fn main() {
    let target = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let max_port: u16 = 10000;
    let num_threads: u16 = 100;

    println!("Scanning {} (ports 1-{}) with {} threads...", target, max_port, num_threads);

    // Create a channel to send open port numbers from worker threads back to main
    let (tx, rx) = channel();

    for t in 0..num_threads {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            // Divide the port range across threads
            let mut port = t + 1;
            while port <= max_port {
                let addr = SocketAddr::new(target, port);
                if TcpStream::connect_timeout(&addr, Duration::from_millis(50)).is_ok() {
                    let _ = tx_clone.send(port);
                }
                port += num_threads;
            }
        });
    }

    // Drop the original sender so the receiver knows when all worker threads finish
    drop(tx);

    let mut open_ports: Vec<u16> = rx.iter().collect();
    open_ports.sort();

    if open_ports.is_empty() {
        println!("No open TCP ports found.");
    } else {
        println!("Open ports found: {:?}", open_ports);
    }
}



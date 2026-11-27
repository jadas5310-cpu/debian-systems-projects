# Debian 13 Systems Programming (C & Rust)

Low-level Linux utilities, terminal dashboards, kernel event monitors, and system call experiments built on Debian 13 (Trixie).

## Rust Projects

* **`klog_filter/`:** Fast standard-input stream filter for `journalctl` kernel warnings and errors.
* **`proc_peek/`:** Reads live CPU load averages and memory metrics directly from the kernel's `/proc` virtual filesystem.
* **`rust_scan/`:** Multithreaded TCP port scanner using `std::thread` and `mpsc` channels to probe open local ports.
* **`sys_dash/`:** Live-updating ANSI terminal system monitor displaying real-time CPU load and an ASCII RAM usage progress bar from `/proc`.

## C Projects
## Python Projects

* **`term_snake/`:** Real-time interactive terminal Snake game powered by Python's `curses` library.
* **`hello_klog/`:** Minimal Linux kernel module using `kbuild` and `pr_info()` to write directly to the kernel ring buffer.
* **`dir_watch/`:** Event-driven file monitor powered by the Linux kernel's `inotify` API that sleeps at 0% CPU until file changes occur.
* **`tiny_strace/`:** Lightweight system call tracer using `fork()`, `execvp()`, and `ptrace()` to intercept and count kernel syscalls made by any command.

## Building & Running

* **Rust projects (`klog_filter`, `proc_peek`, `rust_scan`, `sys_dash`):**
  Enter the project folder and run `cargo run --release`.
* **C user-space tools (`dir_watch`, `tiny_strace`):**
  Compile with `gcc -Wall -Wextra -O2 <file>.c -o <name>` and run `./<name>`.
* **Kernel module (`hello_klog`):**
  Enter `hello_klog/` and run `make` (requires `linux-headers-$(uname -r)`).







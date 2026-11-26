# Debian 13 Systems Programming (C & Rust)

Low-level Linux utilities and kernel experiments built on Debian 13 (Trixie).

## Projects

* **`hello_klog/` (C):** Minimal Linux kernel module using `kbuild` and `pr_info()` to write directly to the kernel ring buffer.
* **`klog_filter/` (Rust):** Fast standard-input stream filter for `journalctl` kernel warnings and errors.
* **`proc_peek/` (Rust):** Reads live CPU load averages and memory metrics directly from the kernel's `/proc` virtual filesystem.

## Building

* **Rust projects (`klog_filter` / `proc_peek`):** Enter the project directory and run `cargo build --release`.
* **Kernel module (`hello_klog`):** Enter `hello_klog/` and run `make` (requires `linux-headers-$(uname -r)`).


### BPF-Rust
- tracing execve calls
- added lsm hooks with applying hooks preventing the sys call




#### To get started
- Download git version of `libbpf-rs` by git clone
- Move to the directory of `libbpf-rs`.
- `cargo install --path libbpf-rs/libbpf-cargo`
- `cargo new bpf-rs`
- Adding entries to toml file
```toml
[dependencies]
anyhow = "1.0"
libbpf-rs = {path = "../libbpf-rs/libbpf-rs"}
libc = "0.2"
```

### To Run it
- `make`
- `sudo <exec-file>`
- to see the trace_pipe `sudo cat /sys/kernel/tracing/trace_pipe` 
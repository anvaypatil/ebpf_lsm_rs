main: skel
	cargo build

bpf: vmlinux
	cargo libbpf build

skel: bpf
	cargo libbpf gen

clean:
	cargo clean
	-rm src/bpf/*.rs src/bpf/vmlinux.h

vmlinux:
	bpftool btf dump file /sys/kernel/btf/vmlinux format c > src/bpf/vmlinux.h

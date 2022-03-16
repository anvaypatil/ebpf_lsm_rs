#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>

SEC("tracepoint/syscalls/sys_enter_execve")
int enter_exec(void *ctx)
{
    bpf_printk("entered exec\n");
    return 0;
}

SEC("tracepoint/syscalls/sys_exit_execve")
int exit_exec(void *ctx)
{
    bpf_printk("exited exec\n");
    return 0;
}

SEC("lsm/bprm_check_security")
int lsm_hook(struct linux_binprm *bprm)
{
    bpf_printk("lsm hook called\n");
    return 0;
}

char LICENSE[] SEC("license") = "GPL";
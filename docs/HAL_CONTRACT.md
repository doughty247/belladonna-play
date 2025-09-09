# HAL Contract (Play)

This document summarizes the HAL contract as consumed by Belladonna Play. Full details live in the core crate. This is a portable façade around platform primitives.

Surface
- ProcessGuard: current_uid(), is_elevated(), launch(cmd,args?)
- SandboxManager: apply_minimum() -> SandboxReport, tighten_network(), restrict_filesystem(paths)
- SyscallMonitor: start() -> bool, snapshot() -> SyscallSnapshot { events }, mode() -> SysmonMode
- DebugProbe: debugger_detected() -> bool

Error modes
- All HAL calls return HalResult<T>. Unsupported operations use HalError::Unsupported.
- SandboxManager.apply_minimum returns a populated SandboxReport even if parts fail; failures are logged via audit and not fatal.
- SyscallMonitor.start is idempotent and safe to call multiple times.

Performance budgets
- Sysmon: target <1–2% CPU on default settings; ringbuf/event path increments a shared counter.
- Sandbox: PR_SET_NO_NEW_PRIVS and seccomp load are O(1) setup costs; Landlock rules are proportional to rule count.

Linux status (v0.0.1)
- Seccomp: default-allow + targeted deny-list; detection via PR_GET_SECCOMP routed through libc_router.
- Landlock: presence detection best-effort; enforcement bit set when active.
- eBPF: feature-gated loader scaffold with ring-buffer heartbeat; perf tracepoint fallback available.

Windows status
- Stubbed interfaces for parity; not used in v0.0.1 (Linux-only).

Routing
- libc_router centralizes prctl seams: set_no_new_privs, set_dumpable, get_seccomp_mode.

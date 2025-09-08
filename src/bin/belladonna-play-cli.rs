//! Belladonna Play CLI: DRM + Anti-Cheat fork developer tooling

use clap::{Arg, Command};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let matches = Command::new("belladonna-play-cli")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Belladonna Play developer CLI (DRM + Anti-Cheat fork)")
        .subcommand_required(true)
        .subcommand(Command::new("hal-report")
            .about("Show HAL sandbox/capability report (text or JSON)")
            .arg(Arg::new("json").long("json").action(clap::ArgAction::SetTrue)))
        .subcommand(Command::new("sysmon-snapshot")
            .about("Take a one-shot SyscallMonitor snapshot and print events")
            .arg(Arg::new("json").long("json").action(clap::ArgAction::SetTrue)))
        .subcommand(Command::new("sysmon-bench")
            .about("Benchmark SyscallMonitor snapshot() overhead")
            .arg(Arg::new("iters").long("iters").value_name("N").default_value("10000"))
            .arg(Arg::new("json").long("json").action(clap::ArgAction::SetTrue))
            .arg(Arg::new("baseline-out").long("baseline-out").value_name("PATH").help("Write JSON baseline file with results")))
    .subcommand(Command::new("ebpf-detect").about("Report eBPF runtime support status and fallback reason").arg(Arg::new("json").long("json").action(clap::ArgAction::SetTrue)))
        .subcommand(Command::new("status").about("Belladonna Play status summary"))
        .get_matches();

    match matches.subcommand() {
        Some(("hal-report", m)) => {
            let hal = olivine_bridge::hal::current().expect("hal platform");
            let report = hal.sandbox.apply_minimum().unwrap();
            if m.get_flag("json") {
                println!("{}", serde_json::to_string_pretty(&report).unwrap());
            } else {
                println!("Belladonna Play — HAL Report");
                println!("  seccomp loaded:      {}", report.seccomp_loaded);
                println!("  landlock present:    {}", report.landlock);
                println!("  landlock enabled:    {}", report.landlock_enabled);
                println!("  namespaces active:   {}", report.namespaces_active);
            }
        }
        Some(("sysmon-snapshot", m)) => {
            let hal = olivine_bridge::hal::current().expect("hal platform");
            let _ = hal.sysmon.start();
            match hal.sysmon.snapshot() {
                Ok(snap) => {
                    if m.get_flag("json") {
                        println!("{}", serde_json::to_string_pretty(&serde_json::json!({"events": snap.events})).unwrap());
                    } else {
                        println!("sysmon_events_total={}", snap.events);
                    }
                }
                Err(e) => { eprintln!("sysmon_snapshot_error:{e}"); std::process::exit(2); }
            }
        }
        Some(("sysmon-bench", m)) => {
            let iters: u64 = m.get_one::<String>("iters").and_then(|s| s.parse().ok()).unwrap_or(10_000);
            let hal = olivine_bridge::hal::current().expect("hal platform");
            let _ = hal.sysmon.start();
            let _ = hal.sysmon.snapshot();
            let start = std::time::Instant::now();
            let mut last = 0u64; let mut errs = 0u64;
            for _ in 0..iters { match hal.sysmon.snapshot() { Ok(s)=> last = s.events, Err(_)=> errs+=1 } }
            let elapsed = start.elapsed();
            let total_ns = elapsed.as_nanos() as f64;
            let per_ns = total_ns / (iters as f64);
            let per_us = per_ns / 1000.0; let per_ms = per_us / 1000.0;
            let ts = chrono::Utc::now().to_rfc3339();
            let obj = serde_json::json!({
                "timestamp": ts,
                "version": env!("CARGO_PKG_VERSION"),
                "iters": iters,
                "elapsed_ms": elapsed.as_secs_f64()*1000.0,
                "avg_us_per_snapshot": per_us,
                "avg_ms_per_1000": per_ms*1000.0,
                "last_events": last,
                "errors": errs
            });
            if let Some(path) = m.get_one::<String>("baseline-out") {
                if let Some(parent) = std::path::Path::new(path).parent() { let _ = std::fs::create_dir_all(parent); }
                match std::fs::write(path, serde_json::to_vec_pretty(&obj).unwrap_or_default()) { Ok(_)=>{}, Err(e)=> eprintln!("baseline_write_error:{}", e) }
            }
            if m.get_flag("json") { println!("{}", serde_json::to_string_pretty(&obj).unwrap()); }
            else { println!("sysmon_bench iters={} elapsed_ms={:.3} avg_us_per_snapshot={:.3} errors={} last_events={}", iters, elapsed.as_secs_f64()*1000.0, per_us, errs, last); }
        }
        Some(("ebpf-detect", m)) => {
            let d = olivine_bridge::ebpf_support::detect();
            if m.get_flag("json") { println!("{}", serde_json::to_string_pretty(&d).unwrap()); }
            else {
                println!("eBPF supported: {} (kernel: {})", d.supported, d.kernel);
                if let Some(r) = d.reason { println!("reason: {}", r); }
                if !d.features.is_empty() { println!("features: {}", d.features.join(",")); }
            }
        }
        Some(("status", _)) => {
            println!("Belladonna Play fork OK. Use `hal-report` or `sysmon-*` to inspect core wiring.");
        }
        _ => unreachable!(),
    }

    Ok(())
}

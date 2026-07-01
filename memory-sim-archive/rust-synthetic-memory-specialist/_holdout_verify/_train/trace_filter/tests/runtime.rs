use std::io::Write;
use std::process::{Command, Stdio};

fn run_app(input: &str) -> (i32, String, String) {
    let exe = env!("CARGO_BIN_EXE_rust_ordo_ext");
    let mut child = Command::new(exe).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().expect("spawn app");
    child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    (output.status.code().unwrap_or(-1), String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n"), String::from_utf8_lossy(&output.stderr).replace("\r\n", "\n"))
}

use rust_ordo_ext::{TraceLog, TraceReport};

#[test]
fn trace_filter_contract() {
    let mut log = TraceLog::new();
    log.record("alpha", "read");
    log.record("beta", "write");
    log.record("gamma", "read");

    assert_eq!(
        log.of_kind("read"),
        TraceReport {
            kind: "read".to_string(),
            items: vec!["read@alpha".to_string(), "read@gamma".to_string()],
        }
    );
    assert_eq!(
        log.of_kind("write"),
        TraceReport {
            kind: "write".to_string(),
            items: vec!["write@beta".to_string()],
        }
    );
    assert_eq!(
        log.of_kind("flush"),
        TraceReport {
            kind: "flush".to_string(),
            items: vec![],
        }
    );
}

#[test]
fn trace_filter_cli() {
    let script = "record alpha read\nrecord beta write\nrecord gamma read\nfilter read\nfilter flush\n";
    let (code, out, err) = run_app(script);
    assert_eq!(code, 0, "stderr={err}");
    assert_eq!(
        out,
        "recorded=read@alpha\nrecorded=write@beta\nrecorded=read@gamma\nfilter=read traces=read@alpha,read@gamma\nfilter=flush traces=none\n"
    );
}

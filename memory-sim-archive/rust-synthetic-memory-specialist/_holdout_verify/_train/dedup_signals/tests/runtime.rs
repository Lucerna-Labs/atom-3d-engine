use std::io::Write;
use std::process::{Command, Stdio};

fn run_app(input: &str) -> (i32, String, String) {
    let exe = env!("CARGO_BIN_EXE_rust_ordo_ext");
    let mut child = Command::new(exe).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().expect("spawn app");
    child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    (output.status.code().unwrap_or(-1), String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n"), String::from_utf8_lossy(&output.stderr).replace("\r\n", "\n"))
}

use rust_ordo_ext::{Report, SignalLog};

#[test]
fn report_dedups_keeping_first_seen_order() {
    let mut log = SignalLog::new();
    log.collect("alpha");
    log.collect("beta");
    log.collect("alpha");
    log.collect("gamma");
    log.collect("beta");
    assert_eq!(
        log.report(),
        Report {
            signals: vec!["alpha".to_string(), "beta".to_string(), "gamma".to_string()],
            count: 3,
        }
    );
}

#[test]
fn cli_collects_and_prints_contract_line() {
    let (code, out, err) = run_app("collect alpha\ncollect beta\ncollect alpha\ncollect gamma\ncollect beta\nreport\n");
    assert_eq!(code, 0);
    assert_eq!(err, "");
    assert_eq!(out, "signals=alpha,beta,gamma count=3\n");
}

#[test]
fn cli_report_with_no_signals_is_none() {
    let (code, out, _err) = run_app("report\n");
    assert_eq!(code, 0);
    assert_eq!(out, "signals=none count=0\n");
}

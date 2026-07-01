use std::io::Write;
use std::process::{Command, Stdio};

fn run_app(input: &str) -> (i32, String, String) {
    let exe = env!("CARGO_BIN_EXE_rust_ordo_ext");
    let mut child = Command::new(exe).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().expect("spawn app");
    child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    (output.status.code().unwrap_or(-1), String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n"), String::from_utf8_lossy(&output.stderr).replace("\r\n", "\n"))
}

use rust_ordo_ext::{ReplayEvent, ReplayLog};

#[test]
fn library_contract_replay_after_checkpoint() {
    let mut log = ReplayLog::new();
    log.record("route", "input"); // idx1
    log.record("route", "filter"); // idx2
    assert_eq!(log.checkpoint(), 2);
    log.record("signal", "model"); // idx3
    assert_eq!(
        log.replay(),
        vec![ReplayEvent {
            index: 3,
            kind: "signal".to_string(),
            node: "model".to_string(),
        }]
    );
}

#[test]
fn cli_replay_after_checkpoint() {
    let input = "record route input\nrecord route filter\ncheckpoint\nrecord signal model\nreplay\n";
    let (code, stdout, stderr) = run_app(input);
    assert_eq!(code, 0, "stderr: {}", stderr);
    assert_eq!(stdout, "replay=3:signal@model\n");
}

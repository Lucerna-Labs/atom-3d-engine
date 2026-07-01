use std::io::Write;
use std::process::{Command, Stdio};

fn run_app(input: &str) -> (i32, String, String) {
    let exe = env!("CARGO_BIN_EXE_rust_ordo_ext");
    let mut child = Command::new(exe).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().expect("spawn app");
    child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    (output.status.code().unwrap_or(-1), String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n"), String::from_utf8_lossy(&output.stderr).replace("\r\n", "\n"))
}

use rust_ordo_ext::{AckTracker, PendingReport};

#[test]
fn ack_tracker_contract() {
    let mut rt = AckTracker::new();
    rt.send("m1");
    rt.send("m2");
    rt.send("m3");
    rt.ack("m2");
    assert_eq!(
        rt.pending(),
        PendingReport {
            pending: vec!["m1".to_string(), "m3".to_string()],
            count: 2,
        }
    );

    let mut rt2 = AckTracker::new();
    rt2.send("a");
    rt2.ack("a");
    assert_eq!(
        rt2.pending(),
        PendingReport {
            pending: vec![],
            count: 0,
        }
    );
}

#[test]
fn ack_tracker_cli() {
    let script = "send m1\nsend m2\nsend m3\nack m2\npending\n";
    let (code, out, err) = run_app(script);
    assert_eq!(code, 0, "stderr={err}");
    assert_eq!(out, "pending=m1>m3 count=2\n");
}

#[test]
fn ack_tracker_cli_empty() {
    let script = "send a\nack a\npending\n";
    let (code, out, err) = run_app(script);
    assert_eq!(code, 0, "stderr={err}");
    assert_eq!(out, "pending=none count=0\n");
}

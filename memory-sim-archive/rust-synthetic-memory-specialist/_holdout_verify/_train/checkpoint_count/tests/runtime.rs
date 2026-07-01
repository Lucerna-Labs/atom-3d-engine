use std::io::Write;
use std::process::{Command, Stdio};

fn run_app(input: &str) -> (i32, String, String) {
    let exe = env!("CARGO_BIN_EXE_rust_ordo_ext");
    let mut child = Command::new(exe).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().expect("spawn app");
    child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    (output.status.code().unwrap_or(-1), String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n"), String::from_utf8_lossy(&output.stderr).replace("\r\n", "\n"))
}

use rust_ordo_ext::{CheckpointCounter, SinceReport};

#[test]
fn checkpoint_count_contract() {
    let mut rt = CheckpointCounter::new();
    rt.record("a");
    rt.record("b");
    rt.checkpoint();
    rt.record("c");
    rt.record("d");
    rt.record("e");
    assert_eq!(
        rt.since(),
        SinceReport {
            count: 3,
            events: vec!["c".to_string(), "d".to_string(), "e".to_string()],
        }
    );

    let mut rt2 = CheckpointCounter::new();
    rt2.record("x");
    rt2.checkpoint();
    assert_eq!(
        rt2.since(),
        SinceReport {
            count: 0,
            events: vec![],
        }
    );

    let mut rt3 = CheckpointCounter::new();
    rt3.record("p");
    rt3.record("q");
    assert_eq!(
        rt3.since(),
        SinceReport {
            count: 2,
            events: vec!["p".to_string(), "q".to_string()],
        }
    );
}

#[test]
fn checkpoint_count_cli() {
    let script = "record a\nrecord b\ncheckpoint\nrecord c\nrecord d\nrecord e\nsince\n";
    let (code, out, err) = run_app(script);
    assert_eq!(code, 0, "stderr={err}");
    assert_eq!(out, "count=3 events=c>d>e\n");
}

#[test]
fn checkpoint_count_cli_empty() {
    let script = "record x\ncheckpoint\nsince\n";
    let (code, out, err) = run_app(script);
    assert_eq!(code, 0, "stderr={err}");
    assert_eq!(out, "count=0 events=none\n");
}

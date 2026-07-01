use std::io::Write;
use std::process::{Command, Stdio};

fn run_app(input: &str) -> (i32, String, String) {
    let exe = env!("CARGO_BIN_EXE_rust_ordo_ext");
    let mut child = Command::new(exe).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().expect("spawn app");
    child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    (output.status.code().unwrap_or(-1), String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n"), String::from_utf8_lossy(&output.stderr).replace("\r\n", "\n"))
}

use rust_ordo_ext::{ThrottlePlan, ThrottleWindow};

#[test]
fn throttle_window_contract() {
    let mut rt = ThrottleWindow::new();
    rt.set_limit(2);
    assert_eq!(
        rt.event("a"),
        ThrottlePlan { node: "a".to_string(), count: 1, signals: vec!["pass:a".to_string()] }
    );
    assert_eq!(
        rt.event("a"),
        ThrottlePlan { node: "a".to_string(), count: 2, signals: vec!["pass:a".to_string()] }
    );
    assert_eq!(
        rt.event("a"),
        ThrottlePlan { node: "a".to_string(), count: 3, signals: vec!["throttle:a".to_string()] }
    );
    // Per-node counters are independent.
    assert_eq!(
        rt.event("b"),
        ThrottlePlan { node: "b".to_string(), count: 1, signals: vec!["pass:b".to_string()] }
    );
}

#[test]
fn throttle_window_cli() {
    let script = "limit 2\nevent a\nevent a\nevent a\nevent b\n";
    let (code, out, err) = run_app(script);
    assert_eq!(code, 0, "stderr={err}");
    assert_eq!(
        out,
        "node=a count=1 signals=pass:a\nnode=a count=2 signals=pass:a\nnode=a count=3 signals=throttle:a\nnode=b count=1 signals=pass:b\n"
    );
}

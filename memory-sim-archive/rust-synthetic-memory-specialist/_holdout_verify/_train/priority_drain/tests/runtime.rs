use std::io::Write;
use std::process::{Command, Stdio};

fn run_app(input: &str) -> (i32, String, String) {
    let exe = env!("CARGO_BIN_EXE_rust_ordo_ext");
    let mut child = Command::new(exe).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().expect("spawn app");
    child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    (output.status.code().unwrap_or(-1), String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n"), String::from_utf8_lossy(&output.stderr).replace("\r\n", "\n"))
}

use rust_ordo_ext::{DrainTrace, PriorityDrain};

#[test]
fn priority_drain_contract() {
    let mut rt = PriorityDrain::new();
    rt.enqueue("a", 5);
    rt.enqueue("b", 9);
    rt.enqueue("c", 5);
    rt.enqueue("d", 1);
    // Highest priority first; the two priority-5 items keep insertion order (a before c).
    assert_eq!(
        rt.drain(),
        DrainTrace {
            order: vec![
                "b".to_string(),
                "a".to_string(),
                "c".to_string(),
                "d".to_string(),
            ],
            signals: vec!["top:b".to_string()],
        }
    );

    let empty = PriorityDrain::new();
    assert_eq!(
        empty.drain(),
        DrainTrace {
            order: Vec::new(),
            signals: vec!["empty".to_string()],
        }
    );
}

#[test]
fn priority_drain_cli() {
    let script = "enqueue a 5\nenqueue b 9\nenqueue c 5\nenqueue d 1\ndrain\n";
    let (code, out, err) = run_app(script);
    assert_eq!(code, 0, "stderr={err}");
    assert_eq!(out, "order=b>a>c>d signals=top:b\n");
}

#[test]
fn priority_drain_cli_empty() {
    let (code, out, err) = run_app("drain\n");
    assert_eq!(code, 0, "stderr={err}");
    assert_eq!(out, "order=none signals=empty\n");
}

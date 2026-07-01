use std::io::Write;
use std::process::{Command, Stdio};

fn run_app(input: &str) -> (i32, String, String) {
    let exe = env!("CARGO_BIN_EXE_rust_ordo_ext");
    let mut child = Command::new(exe).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().expect("spawn app");
    child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    (output.status.code().unwrap_or(-1), String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n"), String::from_utf8_lossy(&output.stderr).replace("\r\n", "\n"))
}

use rust_ordo_ext::{RoundRobin, Rotation};

#[test]
fn contract_cycles_pool_and_flags_empty() {
    let mut rr = RoundRobin::new();
    rr.add("n1");
    rr.add("n2");
    rr.add("n3");

    // 5 requests over a 3-node pool wrap around: n1 n2 n3 n1 n2.
    assert_eq!(
        rr.assign(5),
        Rotation {
            assignments: vec![
                "n1".to_string(),
                "n2".to_string(),
                "n3".to_string(),
                "n1".to_string(),
                "n2".to_string(),
            ],
            signals: vec![],
        }
    );

    // Zero requests over a non-empty pool: nothing assigned, no signal.
    assert_eq!(
        rr.assign(0),
        Rotation {
            assignments: vec![],
            signals: vec![],
        }
    );

    // Empty pool: no assignments, empty_pool signal.
    let empty = RoundRobin::new();
    assert_eq!(
        empty.assign(3),
        Rotation {
            assignments: vec![],
            signals: vec!["empty_pool".to_string()],
        }
    );
}

#[test]
fn cli_cycles_pool() {
    let (code, stdout, stderr) = run_app("add n1\nadd n2\nadd n3\nassign 5\n");
    assert_eq!(code, 0, "stderr: {}", stderr);
    assert_eq!(stdout, "assigned=n1>n2>n3>n1>n2 signals=none\n");
}

#[test]
fn cli_empty_pool() {
    let (code, stdout, stderr) = run_app("assign 3\n");
    assert_eq!(code, 0, "stderr: {}", stderr);
    assert_eq!(stdout, "assigned=none signals=empty_pool\n");
}

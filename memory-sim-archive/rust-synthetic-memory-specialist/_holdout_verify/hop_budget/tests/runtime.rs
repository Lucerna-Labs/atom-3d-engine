use std::io::Write;
use std::process::{Command, Stdio};

fn run_app(input: &str) -> (i32, String, String) {
    let exe = env!("CARGO_BIN_EXE_rust_ordo_ext");
    let mut child = Command::new(exe).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().expect("spawn app");
    child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    (output.status.code().unwrap_or(-1), String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n"), String::from_utf8_lossy(&output.stderr).replace("\r\n", "\n"))
}

use rust_ordo_ext::{HopRuntime, HopTrace};

#[test]
fn contract_budget_exhausted_and_full_walk() {
    let mut rt = HopRuntime::new();
    rt.connect("a", "b");
    rt.connect("b", "c");
    rt.connect("c", "d");

    // 2 hops used, but c still had an outgoing edge -> budget_exhausted.
    assert_eq!(
        rt.flow("a", 2),
        HopTrace {
            path: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            signals: vec!["budget_exhausted".to_string()],
        }
    );

    // Ran out of edges first -> no signals.
    assert_eq!(
        rt.flow("a", 5),
        HopTrace {
            path: vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string()
            ],
            signals: vec![],
        }
    );
}

#[test]
fn cli_budget_exhausted() {
    let (code, stdout, stderr) = run_app("connect a b\nconnect b c\nconnect c d\nflow a 2\n");
    assert_eq!(code, 0, "stderr: {}", stderr);
    assert_eq!(stdout, "path=a>b>c signals=budget_exhausted\n");
}

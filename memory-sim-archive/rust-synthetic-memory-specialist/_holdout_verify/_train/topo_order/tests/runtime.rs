use std::io::Write;
use std::process::{Command, Stdio};

fn run_app(input: &str) -> (i32, String, String) {
    let exe = env!("CARGO_BIN_EXE_rust_ordo_ext");
    let mut child = Command::new(exe).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().expect("spawn app");
    child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    (output.status.code().unwrap_or(-1), String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n"), String::from_utf8_lossy(&output.stderr).replace("\r\n", "\n"))
}

use rust_ordo_ext::{TopoOrder, TopoRuntime};

#[test]
fn contract_acyclic_and_cycle() {
    // a -> b, a -> c, b -> d, c -> d. First-seen node order: a, b, c, d.
    // Kahn with earliest-first tie-break emits a, then b, then c, then d.
    let mut rt = TopoRuntime::new();
    rt.edge("a", "b");
    rt.edge("a", "c");
    rt.edge("b", "d");
    rt.edge("c", "d");
    assert_eq!(
        rt.order(),
        TopoOrder {
            order: vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string()
            ],
            cycle: false,
            remaining: Vec::new(),
        }
    );

    // x -> y -> z -> y forms a cycle. "x" emits first (indegree 0); then y and z
    // are mutually blocked, so both remain in first-seen order.
    let mut rt2 = TopoRuntime::new();
    rt2.edge("x", "y");
    rt2.edge("y", "z");
    rt2.edge("z", "y");
    assert_eq!(
        rt2.order(),
        TopoOrder {
            order: vec!["x".to_string()],
            cycle: true,
            remaining: vec!["y".to_string(), "z".to_string()],
        }
    );
}

#[test]
fn cli_acyclic_order() {
    let (code, stdout, stderr) =
        run_app("edge a b\nedge a c\nedge b d\nedge c d\norder\n");
    assert_eq!(code, 0, "stderr: {}", stderr);
    assert_eq!(stdout, "cycle=no order=a>b>c>d remaining=none\n");
}

#[test]
fn cli_cycle_reports_remaining() {
    let (code, stdout, stderr) =
        run_app("edge x y\nedge y z\nedge z y\norder\n");
    assert_eq!(code, 0, "stderr: {}", stderr);
    assert_eq!(stdout, "cycle=yes order=x remaining=y,z\n");
}

use std::io::Write;
use std::process::{Command, Stdio};

fn run_app(input: &str) -> (i32, String, String) {
    let exe = env!("CARGO_BIN_EXE_rust_ordo_ext");
    let mut child = Command::new(exe).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().expect("spawn app");
    child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    (output.status.code().unwrap_or(-1), String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n"), String::from_utf8_lossy(&output.stderr).replace("\r\n", "\n"))
}

use rust_ordo_ext::{CycleRuntime, CycleTrace};

#[test]
fn contract_cycle_and_acyclic() {
    let mut rt = CycleRuntime::new();
    rt.connect("a", "b");
    rt.connect("b", "c");
    rt.connect("c", "a");

    // a -> b -> c -> a, revisits "a": cycle, repeated node appended to path.
    assert_eq!(
        rt.walk("a"),
        CycleTrace {
            path: vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "a".to_string()
            ],
            cycle: true,
            repeated: "a".to_string(),
        }
    );

    // A separate acyclic line ends when no outgoing edge remains.
    let mut rt2 = CycleRuntime::new();
    rt2.connect("x", "y");
    rt2.connect("y", "z");
    assert_eq!(
        rt2.walk("x"),
        CycleTrace {
            path: vec!["x".to_string(), "y".to_string(), "z".to_string()],
            cycle: false,
            repeated: "none".to_string(),
        }
    );
}

#[test]
fn cli_cycle_detected() {
    let (code, stdout, stderr) =
        run_app("connect a b\nconnect b c\nconnect c a\nwalk a\n");
    assert_eq!(code, 0, "stderr: {}", stderr);
    assert_eq!(stdout, "cycle=yes repeated=a path=a>b>c>a\n");
}

#[test]
fn cli_acyclic_walk() {
    let (code, stdout, stderr) = run_app("connect x y\nconnect y z\nwalk x\n");
    assert_eq!(code, 0, "stderr: {}", stderr);
    assert_eq!(stdout, "cycle=no repeated=none path=x>y>z\n");
}

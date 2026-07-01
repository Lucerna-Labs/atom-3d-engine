use std::io::Write;
use std::process::{Command, Stdio};

fn run_app(input: &str) -> (i32, String, String) {
    let exe = env!("CARGO_BIN_EXE_rust_ordo_ext");
    let mut child = Command::new(exe).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().expect("spawn app");
    child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    (output.status.code().unwrap_or(-1), String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n"), String::from_utf8_lossy(&output.stderr).replace("\r\n", "\n"))
}

use rust_ordo_ext::{DiffFabric, StateDiff};

#[test]
fn contract_library() {
    let mut fabric = DiffFabric::new();
    fabric.before(&["a", "b", "c"]);
    fabric.after(&["a", "c", "d"]);
    assert_eq!(
        fabric.diff(),
        StateDiff {
            added: vec!["d".to_string()],
            removed: vec!["b".to_string()],
        }
    );

    // Results are sorted regardless of declaration order.
    let mut fabric = DiffFabric::new();
    fabric.before(&["x"]);
    fabric.after(&["z", "y", "x", "w"]);
    assert_eq!(
        fabric.diff(),
        StateDiff {
            added: vec!["w".to_string(), "y".to_string(), "z".to_string()],
            removed: vec![],
        }
    );

    // No changes between sets yields two empty lists.
    let mut fabric = DiffFabric::new();
    fabric.before(&["a", "b"]);
    fabric.after(&["b", "a"]);
    assert_eq!(
        fabric.diff(),
        StateDiff {
            added: vec![],
            removed: vec![],
        }
    );
}

#[test]
fn contract_cli() {
    let (code, out, err) = run_app("before a b c\nafter a c d\ndiff\n");
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "added=d removed=b\n");
}

#[test]
fn contract_cli_none() {
    let (code, out, err) = run_app("before a b\nafter b a\ndiff\n");
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "added=none removed=none\n");
}

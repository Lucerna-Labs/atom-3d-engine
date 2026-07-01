use std::io::Write;
use std::process::{Command, Stdio};

fn run_app(input: &str) -> (i32, String, String) {
    let exe = env!("CARGO_BIN_EXE_rust_ordo_ext");
    let mut child = Command::new(exe).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().expect("spawn app");
    child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    (output.status.code().unwrap_or(-1), String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n"), String::from_utf8_lossy(&output.stderr).replace("\r\n", "\n"))
}

use rust_ordo_ext::{Merger, Path};

#[test]
fn merge_keeps_first_seen_order_and_signals_dups() {
    let mut m = Merger::new();
    m.flow(&["a", "b", "c"]);
    m.flow(&["b", "d", "a"]);
    assert_eq!(
        m.merge(),
        Path {
            nodes: vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string()],
            signals: vec!["dup:b".to_string(), "dup:a".to_string()],
        }
    );
}

#[test]
fn cli_merges_flows_and_prints_contract_line() {
    let (code, out, err) = run_app("flow a b c\nflow b d a\nmerge\n");
    assert_eq!(code, 0);
    assert_eq!(err, "");
    assert_eq!(out, "path=a>b>c>d signals=dup:b,dup:a\n");
}

#[test]
fn cli_merge_with_no_flows_is_none() {
    let (code, out, _err) = run_app("merge\n");
    assert_eq!(code, 0);
    assert_eq!(out, "path=none signals=none\n");
}

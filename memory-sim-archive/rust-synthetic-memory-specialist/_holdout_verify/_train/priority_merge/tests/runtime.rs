use std::io::Write;
use std::process::{Command, Stdio};

fn run_app(input: &str) -> (i32, String, String) {
    let exe = env!("CARGO_BIN_EXE_rust_ordo_ext");
    let mut child = Command::new(exe).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().expect("spawn app");
    child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    (output.status.code().unwrap_or(-1), String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n"), String::from_utf8_lossy(&output.stderr).replace("\r\n", "\n"))
}

use rust_ordo_ext::{MergeResult, PriorityMerge};

#[test]
fn priority_merge_contract() {
    let mut rt = PriorityMerge::new();
    rt.first("a", 5);
    rt.first("b", 9);
    rt.second("c", 5);
    rt.second("d", 9);
    // Priority 9 first: b (first list) before d (second list). Then priority 5:
    // a (first list) before c (second list).
    assert_eq!(
        rt.merge(),
        MergeResult {
            order: vec![
                "b".to_string(),
                "d".to_string(),
                "a".to_string(),
                "c".to_string(),
            ],
            signals: vec!["top:b".to_string()],
        }
    );

    let empty = PriorityMerge::new();
    assert_eq!(
        empty.merge(),
        MergeResult {
            order: Vec::new(),
            signals: vec!["empty".to_string()],
        }
    );
}

#[test]
fn priority_merge_cli() {
    let script = "first a 5\nfirst b 9\nsecond c 5\nsecond d 9\nmerge\n";
    let (code, out, err) = run_app(script);
    assert_eq!(code, 0, "stderr={err}");
    assert_eq!(out, "order=b>d>a>c signals=top:b\n");
}

#[test]
fn priority_merge_cli_empty() {
    let (code, out, err) = run_app("merge\n");
    assert_eq!(code, 0, "stderr={err}");
    assert_eq!(out, "order=none signals=empty\n");
}

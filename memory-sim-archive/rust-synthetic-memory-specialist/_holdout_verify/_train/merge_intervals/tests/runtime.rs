use std::io::Write;
use std::process::{Command, Stdio};

fn run_app(input: &str) -> (i32, String, String) {
    let exe = env!("CARGO_BIN_EXE_rust_ordo_ext");
    let mut child = Command::new(exe).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().expect("spawn app");
    child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    (output.status.code().unwrap_or(-1), String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n"), String::from_utf8_lossy(&output.stderr).replace("\r\n", "\n"))
}

use rust_ordo_ext::{Interval, MergeIntervals};

#[test]
fn merge_collapses_overlapping_and_adjacent_ranges_in_order() {
    let mut m = MergeIntervals::new();
    m.add(1, 3);
    m.add(2, 6);
    m.add(8, 10);
    m.add(15, 18);
    m.add(10, 12);
    assert_eq!(
        m.merge(),
        vec![
            Interval { start: 1, end: 6 },
            Interval { start: 8, end: 12 },
            Interval { start: 15, end: 18 },
        ]
    );
}

#[test]
fn cli_merges_ranges_and_prints_contract_line() {
    let (code, out, err) = run_app("add 1 3\nadd 2 6\nadd 8 10\nadd 15 18\nadd 10 12\nmerge\n");
    assert_eq!(code, 0);
    assert_eq!(err, "");
    assert_eq!(out, "merged=1,6>8,12>15,18\n");
}

#[test]
fn cli_merge_with_no_ranges_is_none() {
    let (code, out, _err) = run_app("merge\n");
    assert_eq!(code, 0);
    assert_eq!(out, "merged=none\n");
}

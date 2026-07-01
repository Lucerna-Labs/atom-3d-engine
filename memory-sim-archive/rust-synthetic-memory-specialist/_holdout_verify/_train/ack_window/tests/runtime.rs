use std::io::Write;
use std::process::{Command, Stdio};

fn run_app(input: &str) -> (i32, String, String) {
    let exe = env!("CARGO_BIN_EXE_rust_ordo_ext");
    let mut child = Command::new(exe).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().expect("spawn app");
    child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    (output.status.code().unwrap_or(-1), String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n"), String::from_utf8_lossy(&output.stderr).replace("\r\n", "\n"))
}

use rust_ordo_ext::{AckReport, AckWindow};

#[test]
fn ack_window_contract() {
    let mut rt = AckWindow::new();
    rt.send("a", 1);
    rt.send("b", 2);
    rt.send("c", 3);
    rt.ack("b", 4);
    rt.ack("c", 9);
    // At cutoff 5: a never acked -> unacked; b acked at 4 (<=5) -> acked;
    // c acked at 9 (>5, too late) -> unacked.
    assert_eq!(
        rt.report(5),
        AckReport {
            unacked: vec!["a".to_string(), "c".to_string()],
            count: 2,
        }
    );

    // A later cutoff catches c's ack, leaving only the never-acked a.
    assert_eq!(
        rt.report(9),
        AckReport {
            unacked: vec!["a".to_string()],
            count: 1,
        }
    );

    // Everything acked in time -> empty report.
    let mut rt2 = AckWindow::new();
    rt2.send("x", 1);
    rt2.ack("x", 1);
    assert_eq!(
        rt2.report(1),
        AckReport {
            unacked: vec![],
            count: 0,
        }
    );
}

#[test]
fn ack_window_cli() {
    let script = "send a 1\nsend b 2\nsend c 3\nack b 4\nack c 9\nreport 5\n";
    let (code, out, err) = run_app(script);
    assert_eq!(code, 0, "stderr={err}");
    assert_eq!(out, "unacked=a>c count=2\n");
}

#[test]
fn ack_window_cli_none() {
    let script = "send x 1\nack x 1\nreport 1\n";
    let (code, out, err) = run_app(script);
    assert_eq!(code, 0, "stderr={err}");
    assert_eq!(out, "unacked=none count=0\n");
}

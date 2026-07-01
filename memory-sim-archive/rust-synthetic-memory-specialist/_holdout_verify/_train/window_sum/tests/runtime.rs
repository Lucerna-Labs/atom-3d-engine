use std::io::Write;
use std::process::{Command, Stdio};

fn run_app(input: &str) -> (i32, String, String) {
    let exe = env!("CARGO_BIN_EXE_rust_ordo_ext");
    let mut child = Command::new(exe).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().expect("spawn app");
    child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    (output.status.code().unwrap_or(-1), String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n"), String::from_utf8_lossy(&output.stderr).replace("\r\n", "\n"))
}

use rust_ordo_ext::{WindowReport, WindowSum};

#[test]
fn window_sum_contract() {
    let mut rt = WindowSum::new();
    rt.set_window(3);
    rt.push("a", 10);
    rt.push("a", 20);
    // Fewer values than the window: sum everything recorded so far.
    assert_eq!(
        rt.report("a"),
        WindowReport { node: "a".to_string(), sum: 30, used: vec![10, 20] }
    );
    rt.push("a", 30);
    rt.push("a", 40);
    // More values than the window: only the last K count.
    assert_eq!(
        rt.report("a"),
        WindowReport { node: "a".to_string(), sum: 90, used: vec![20, 30, 40] }
    );
    // Negative values are supported and per-node series are independent.
    rt.push("b", -5);
    assert_eq!(
        rt.report("b"),
        WindowReport { node: "b".to_string(), sum: -5, used: vec![-5] }
    );
    // Unknown node: empty window.
    assert_eq!(
        rt.report("z"),
        WindowReport { node: "z".to_string(), sum: 0, used: vec![] }
    );
}

#[test]
fn window_sum_cli() {
    let script = "window 3\npush a 10\npush a 20\nreport a\npush a 30\npush a 40\nreport a\npush b -5\nreport b\nreport z\n";
    let (code, out, err) = run_app(script);
    assert_eq!(code, 0, "stderr={err}");
    assert_eq!(
        out,
        "node=a sum=30 used=10,20\nnode=a sum=90 used=20,30,40\nnode=b sum=-5 used=-5\nnode=z sum=0 used=none\n"
    );
}

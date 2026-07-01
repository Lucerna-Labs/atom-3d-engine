use std::io::Write;
use std::process::{Command, Stdio};

fn run_app(input: &str) -> (i32, String, String) {
    let exe = env!("CARGO_BIN_EXE_rust_ordo_ext");
    let mut child = Command::new(exe).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().expect("spawn app");
    child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    (output.status.code().unwrap_or(-1), String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n"), String::from_utf8_lossy(&output.stderr).replace("\r\n", "\n"))
}

use rust_ordo_ext::{LanePlan, LaneScheduler};

#[test]
fn priority_lane_contract() {
    let mut rt = LaneScheduler::new();
    rt.add_node("fast", 10);
    rt.add_node("slow", 1);
    rt.add_node("mid", 5);
    assert_eq!(
        rt.schedule(&["slow", "mid", "fast"]),
        Some(LanePlan {
            node: "fast".to_string(),
            priority: 10,
            signals: vec!["lane:fast".to_string()],
        })
    );
    assert_eq!(rt.schedule(&["unknown"]), None);
}

#[test]
fn priority_lane_cli() {
    let script = "node fast 10\nnode slow 1\nnode mid 5\nschedule slow mid fast\n";
    let (code, out, err) = run_app(script);
    assert_eq!(code, 0, "stderr={err}");
    assert_eq!(out, "node=fast priority=10 signals=lane:fast\n");
}

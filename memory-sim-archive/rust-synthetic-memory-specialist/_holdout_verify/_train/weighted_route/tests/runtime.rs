use std::io::Write;
use std::process::{Command, Stdio};

fn run_app(input: &str) -> (i32, String, String) {
    let exe = env!("CARGO_BIN_EXE_rust_ordo_ext");
    let mut child = Command::new(exe).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().expect("spawn app");
    child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    (output.status.code().unwrap_or(-1), String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n"), String::from_utf8_lossy(&output.stderr).replace("\r\n", "\n"))
}

use rust_ordo_ext::{RouteTrace, WeightRouter};

#[test]
fn contract_dead_end_and_cycle() {
    let mut rt = WeightRouter::new();
    rt.connect("a", "b", 5);
    rt.connect("b", "c", 3);
    rt.connect("c", "d", 4);

    // Walk a>b>c>d, summing 5+3+4=12, stopping at the dead end d.
    assert_eq!(
        rt.route("a"),
        RouteTrace {
            path: vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string()
            ],
            total: 12,
            signals: vec!["end:d".to_string()],
        }
    );

    // Add an edge back into the visited set: c -> a is added before c -> d in
    // insertion order, so the first outgoing edge of c now points to a, which is
    // already on the path. The cycle edge weight is not counted.
    let mut rt2 = WeightRouter::new();
    rt2.connect("a", "b", 5);
    rt2.connect("b", "c", 3);
    rt2.connect("c", "a", 9);
    assert_eq!(
        rt2.route("a"),
        RouteTrace {
            path: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            total: 8,
            signals: vec!["cycle:a".to_string()],
        }
    );
}

#[test]
fn cli_full_walk_total() {
    let (code, stdout, stderr) = run_app("connect a b 5\nconnect b c 3\nconnect c d 4\nroute a\n");
    assert_eq!(code, 0, "stderr: {}", stderr);
    assert_eq!(stdout, "path=a>b>c>d total=12 signals=end:d\n");
}

#[test]
fn cli_cycle() {
    let (code, stdout, stderr) = run_app("connect a b 5\nconnect b c 3\nconnect c a 9\nroute a\n");
    assert_eq!(code, 0, "stderr: {}", stderr);
    assert_eq!(stdout, "path=a>b>c total=8 signals=cycle:a\n");
}

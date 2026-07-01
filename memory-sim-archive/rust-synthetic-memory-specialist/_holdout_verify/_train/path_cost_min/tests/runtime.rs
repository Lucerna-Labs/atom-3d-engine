use std::io::Write;
use std::process::{Command, Stdio};

fn run_app(input: &str) -> (i32, String, String) {
    let exe = env!("CARGO_BIN_EXE_rust_ordo_ext");
    let mut child = Command::new(exe).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().expect("spawn app");
    child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    (output.status.code().unwrap_or(-1), String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n"), String::from_utf8_lossy(&output.stderr).replace("\r\n", "\n"))
}

use rust_ordo_ext::{CostGraph, Pick};

#[test]
fn contract_cheapest_miss_and_tie() {
    let mut g = CostGraph::new();
    g.connect("s", "a", 7);
    g.connect("s", "b", 3);
    g.connect("s", "c", 9);

    // Among a(7), b(3), c(9) the cheapest first-edge cost is b at 3. The candidate
    // x has no direct edge from s, so it becomes a miss signal in query order.
    assert_eq!(
        g.cheapest("s", &["a", "b", "c", "x"]),
        Pick {
            target: "b".to_string(),
            cost: 3,
            signals: vec!["miss:x".to_string()],
        }
    );

    // Ties on cost are broken by query order: a and b both cost 5, a appears
    // first in the query, so a wins. No misses, so no signals.
    let mut g2 = CostGraph::new();
    g2.connect("s", "a", 5);
    g2.connect("s", "b", 5);
    assert_eq!(
        g2.cheapest("s", &["a", "b"]),
        Pick {
            target: "a".to_string(),
            cost: 5,
            signals: vec![],
        }
    );

    // No candidate has a direct edge from s: every candidate misses and the pick
    // is the "none" / unreachable sentinel.
    assert_eq!(
        g2.cheapest("s", &["x", "y"]),
        Pick {
            target: "none".to_string(),
            cost: 0,
            signals: vec![
                "miss:x".to_string(),
                "miss:y".to_string(),
                "unreachable".to_string()
            ],
        }
    );
}

#[test]
fn cli_cheapest_with_miss() {
    let (code, stdout, stderr) =
        run_app("connect s a 7\nconnect s b 3\nconnect s c 9\ncheapest s a b c x\n");
    assert_eq!(code, 0, "stderr: {}", stderr);
    assert_eq!(stdout, "target=b cost=3 signals=miss:x\n");
}

#[test]
fn cli_all_reachable_no_signals() {
    let (code, stdout, stderr) =
        run_app("connect s a 7\nconnect s b 3\ncheapest s a b\n");
    assert_eq!(code, 0, "stderr: {}", stderr);
    assert_eq!(stdout, "target=b cost=3 signals=none\n");
}

#[test]
fn cli_unreachable() {
    let (code, stdout, stderr) = run_app("connect s a 7\ncheapest s x y\n");
    assert_eq!(code, 0, "stderr: {}", stderr);
    assert_eq!(stdout, "target=none cost=0 signals=miss:x,miss:y,unreachable\n");
}

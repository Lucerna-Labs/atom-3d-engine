use std::io::Write;
use std::process::{Command, Stdio};

fn run_app(input: &str) -> (i32, String, String) {
    let exe = env!("CARGO_BIN_EXE_rust_ordo_ext");
    let mut child = Command::new(exe).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().expect("spawn app");
    child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    (output.status.code().unwrap_or(-1), String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n"), String::from_utf8_lossy(&output.stderr).replace("\r\n", "\n"))
}

use rust_ordo_ext::{DepthReport, Graph};

#[test]
fn graph_reports_longest_path() {
    let mut g = Graph::new();
    // a branches: a->b->c (2 hops) and a->d (1 hop). Longest is a>b>c.
    g.link("a", "b");
    g.link("a", "d");
    g.link("b", "c");
    assert_eq!(
        g.depth("a"),
        DepthReport {
            start: "a".to_string(),
            hops: 2,
            deepest: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            signals: vec!["leaf:c".to_string()],
        }
    );

    // A start node with no outgoing edge is 0 hops, deepest is just itself.
    let mut g2 = Graph::new();
    g2.link("x", "y");
    assert_eq!(
        g2.depth("z"),
        DepthReport {
            start: "z".to_string(),
            hops: 0,
            deepest: vec!["z".to_string()],
            signals: vec!["leaf:z".to_string()],
        }
    );

    // A cycle is bounded by path-visited protection and reported as a signal.
    let mut g3 = Graph::new();
    g3.link("p", "q");
    g3.link("q", "p");
    assert_eq!(
        g3.depth("p"),
        DepthReport {
            start: "p".to_string(),
            hops: 1,
            deepest: vec!["p".to_string(), "q".to_string()],
            signals: vec!["cycle:p".to_string(), "leaf:q".to_string()],
        }
    );
}

#[test]
fn cli_prints_depth_contract() {
    let input = "link a b\nlink a d\nlink b c\ndepth a\n";
    let (code, stdout, stderr) = run_app(input);
    assert_eq!(code, 0, "stderr: {}", stderr);
    assert_eq!(stdout, "hops=2 path=a>b>c signals=leaf:c\n");

    // No outgoing edges from the start -> 0 hops, path is the start alone.
    let (code2, stdout2, _) = run_app("link a b\ndepth q\n");
    assert_eq!(code2, 0);
    assert_eq!(stdout2, "hops=0 path=q signals=leaf:q\n");

    // Cycle protection emits a cycle signal alongside the leaf.
    let (code3, stdout3, _) = run_app("link p q\nlink q p\ndepth p\n");
    assert_eq!(code3, 0);
    assert_eq!(stdout3, "hops=1 path=p>q signals=cycle:p,leaf:q\n");
}

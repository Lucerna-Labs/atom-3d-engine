use std::io::Write;
use std::process::{Command, Stdio};

fn run_app(input: &str) -> (i32, String, String) {
    let exe = env!("CARGO_BIN_EXE_rust_ordo_ext");
    let mut child = Command::new(exe).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().expect("spawn app");
    child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    (output.status.code().unwrap_or(-1), String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n"), String::from_utf8_lossy(&output.stderr).replace("\r\n", "\n"))
}

// Library contract: shortest() returns the public plain-data HopResult.
#[test]
fn library_contract() {
    use rust_ordo_ext::{HopGraph, HopResult};

    let mut g = HopGraph::new();
    // A long path a->b->c->d (3 hops) and a short path a->e->d (2 hops).
    g.link("a", "b");
    g.link("b", "c");
    g.link("c", "d");
    g.link("a", "e");
    g.link("e", "d");

    // BFS finds the minimum-hop route a->e->d (2), not the 3-hop a->b->c->d.
    assert_eq!(
        g.shortest("a", "d"),
        HopResult {
            reachable: "yes".to_string(),
            path: vec!["a".to_string(), "e".to_string(), "d".to_string()],
            hops: 2,
        }
    );

    // A node reaches itself in zero hops.
    assert_eq!(
        g.shortest("a", "a"),
        HopResult {
            reachable: "yes".to_string(),
            path: vec!["a".to_string()],
            hops: 0,
        }
    );

    // No directed path from d back to a -> unreachable, hops -1, empty path.
    assert_eq!(
        g.shortest("d", "a"),
        HopResult {
            reachable: "no".to_string(),
            path: Vec::new(),
            hops: -1,
        }
    );
}

// Among equal-length shortest paths, ties break by edge insertion order, and
// cycles must not loop forever (BFS marks nodes visited).
#[test]
fn library_tie_and_cycle() {
    use rust_ordo_ext::HopGraph;

    let mut g = HopGraph::new();
    // Two 2-hop routes a->b->d and a->c->d; b was linked first so it wins.
    g.link("a", "b");
    g.link("a", "c");
    g.link("b", "d");
    g.link("c", "d");
    // A back-edge forming a cycle must not cause an infinite loop.
    g.link("d", "a");

    let r = g.shortest("a", "d");
    assert_eq!(r.reachable, "yes");
    assert_eq!(r.path, vec!["a".to_string(), "b".to_string(), "d".to_string()]);
    assert_eq!(r.hops, 2);
}

// CLI contract: byte-for-byte stdout for the documented example.
#[test]
fn cli_contract() {
    let input = "link a b\nlink b c\nlink c d\nlink a e\nlink e d\nhops a d\nhops a a\nhops d a\n";
    let (code, stdout, stderr) = run_app(input);
    assert_eq!(code, 0, "stderr: {}", stderr);
    assert_eq!(
        stdout,
        "yes a>e>d 2\nyes a 0\nno none -1\n"
    );
}

use std::io::Write;
use std::process::{Command, Stdio};

fn run_app(input: &str) -> (i32, String, String) {
    let exe = env!("CARGO_BIN_EXE_rust_ordo_ext");
    let mut child = Command::new(exe).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().expect("spawn app");
    child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    (output.status.code().unwrap_or(-1), String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n"), String::from_utf8_lossy(&output.stderr).replace("\r\n", "\n"))
}

// Library contract: reachable() returns the public plain-data ReachSet, sorted alphabetically.
#[test]
fn library_contract() {
    use rust_ordo_ext::{Graph, ReachSet};

    let mut g = Graph::new();
    g.link("a", "b");
    g.link("b", "c");
    g.link("a", "x");
    g.link("x", "c");
    g.link("d", "e");

    // From "a": reaches a (itself), b, c, x. Sorted alphabetically.
    assert_eq!(
        g.reachable("a"),
        ReachSet {
            nodes: vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "x".to_string(),
            ],
            count: 4,
        }
    );

    // A leaf node reaches only itself.
    assert_eq!(
        g.reachable("c"),
        ReachSet {
            nodes: vec!["c".to_string()],
            count: 1,
        }
    );

    // An isolated start node not present as any edge endpoint still reaches itself.
    assert_eq!(
        g.reachable("z"),
        ReachSet {
            nodes: vec!["z".to_string()],
            count: 1,
        }
    );
}

// Cycles must not loop forever; traversal is cycle-safe via a visited list.
#[test]
fn library_handles_cycle() {
    use rust_ordo_ext::Graph;

    let mut g = Graph::new();
    g.link("a", "b");
    g.link("b", "a");
    g.link("b", "c");

    let r = g.reachable("a");
    assert_eq!(r.nodes, vec!["a".to_string(), "b".to_string(), "c".to_string()]);
    assert_eq!(r.count, 3);
}

// CLI contract: byte-for-byte stdout for the documented example.
#[test]
fn cli_contract() {
    let input = "link a b\nlink b c\nlink a x\nlink x c\nlink d e\nreach a\nreach c\nreach z\n";
    let (code, stdout, stderr) = run_app(input);
    assert_eq!(code, 0, "stderr: {}", stderr);
    assert_eq!(
        stdout,
        "nodes=a,b,c,x count=4\nnodes=c count=1\nnodes=z count=1\n"
    );
}

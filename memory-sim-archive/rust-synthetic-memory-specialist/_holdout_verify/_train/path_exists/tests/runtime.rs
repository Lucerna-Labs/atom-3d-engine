use std::io::Write;
use std::process::{Command, Stdio};

fn run_app(input: &str) -> (i32, String, String) {
    let exe = env!("CARGO_BIN_EXE_rust_ordo_ext");
    let mut child = Command::new(exe).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().expect("spawn app");
    child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    (output.status.code().unwrap_or(-1), String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n"), String::from_utf8_lossy(&output.stderr).replace("\r\n", "\n"))
}

// Library contract: reach() returns the public plain-data Reachability.
#[test]
fn library_contract() {
    use rust_ordo_ext::{Graph, Reachability};

    let mut g = Graph::new();
    g.link("a", "b");
    g.link("b", "c");
    g.link("a", "x");
    g.link("x", "c");

    // Path exists a -> b -> c, found via insertion-order DFS.
    assert_eq!(
        g.reach("a", "c"),
        Reachability {
            reachable: "yes".to_string(),
            route: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            hops: 2,
        }
    );

    // No directed path from c back to a.
    assert_eq!(
        g.reach("c", "a"),
        Reachability {
            reachable: "no".to_string(),
            route: Vec::new(),
            hops: 0,
        }
    );

    // A node always reaches itself with zero hops.
    assert_eq!(
        g.reach("a", "a"),
        Reachability {
            reachable: "yes".to_string(),
            route: vec!["a".to_string()],
            hops: 0,
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

    let r = g.reach("a", "c");
    assert_eq!(r.reachable, "yes");
    assert_eq!(r.route, vec!["a".to_string(), "b".to_string(), "c".to_string()]);
    assert_eq!(r.hops, 2);
}

// CLI contract: byte-for-byte stdout for the documented example.
#[test]
fn cli_contract() {
    let input = "link a b\nlink b c\nlink a x\nlink x c\nreach a c\nreach c a\nreach a a\n";
    let (code, stdout, stderr) = run_app(input);
    assert_eq!(code, 0, "stderr: {}", stderr);
    assert_eq!(
        stdout,
        "reachable=yes route=a>b>c hops=2\nreachable=no route=none hops=0\nreachable=yes route=a hops=0\n"
    );
}

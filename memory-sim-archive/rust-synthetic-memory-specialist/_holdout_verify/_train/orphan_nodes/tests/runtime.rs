use std::io::Write;
use std::process::{Command, Stdio};

fn run_app(input: &str) -> (i32, String, String) {
    let exe = env!("CARGO_BIN_EXE_rust_ordo_ext");
    let mut child = Command::new(exe).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().expect("spawn app");
    child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    (output.status.code().unwrap_or(-1), String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n"), String::from_utf8_lossy(&output.stderr).replace("\r\n", "\n"))
}

#[test]
fn library_reports_sorted_orphans() {
    use rust_ordo_ext::{Graph, OrphanReport};
    let mut graph = Graph::new();
    graph.link("d", "a");
    graph.link("a", "b");
    graph.link("a", "c");
    graph.link("b", "c");

    // Nodes in the graph: d, a, b, c. Incoming edges target a, b, c. Only d has
    // no incoming edge, so d is the sole orphan.
    assert_eq!(
        graph.orphans(),
        OrphanReport {
            nodes: vec!["d".to_string()],
        }
    );
}

#[test]
fn library_orphans_are_sorted_when_multiple() {
    use rust_ordo_ext::{Graph, OrphanReport};
    let mut graph = Graph::new();
    // Two roots feed a shared target; both roots are orphans and the report is
    // sorted regardless of first-seen order.
    graph.link("x", "m");
    graph.link("c", "m");

    assert_eq!(
        graph.orphans(),
        OrphanReport {
            nodes: vec!["c".to_string(), "x".to_string()],
        }
    );
}

#[test]
fn library_reports_no_orphans_in_a_cycle() {
    use rust_ordo_ext::{Graph, OrphanReport};
    let mut graph = Graph::new();
    graph.link("a", "b");
    graph.link("b", "a");

    // Every node has an incoming edge, so there are no orphans.
    assert_eq!(graph.orphans(), OrphanReport { nodes: Vec::new() });
}

#[test]
fn cli_reports_sorted_orphans_and_none() {
    let input = "link d a\nlink a b\nlink a c\nlink b c\norphans\nlink x b\norphans\n";
    let (code, stdout, stderr) = run_app(input);
    assert_eq!(code, 0, "stderr: {}", stderr);
    // First query: only d is an orphan. Second query: d and x are both orphans,
    // reported sorted as "d,x".
    assert_eq!(stdout, "d\nd,x\n");
}

#[test]
fn cli_reports_none_when_every_node_has_incoming() {
    let input = "link a b\nlink b a\norphans\n";
    let (code, stdout, stderr) = run_app(input);
    assert_eq!(code, 0, "stderr: {}", stderr);
    assert_eq!(stdout, "none\n");
}

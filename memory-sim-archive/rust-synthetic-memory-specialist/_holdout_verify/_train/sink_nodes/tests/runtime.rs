use std::io::Write;
use std::process::{Command, Stdio};

fn run_app(input: &str) -> (i32, String, String) {
    let exe = env!("CARGO_BIN_EXE_rust_ordo_ext");
    let mut child = Command::new(exe).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().expect("spawn app");
    child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    (output.status.code().unwrap_or(-1), String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n"), String::from_utf8_lossy(&output.stderr).replace("\r\n", "\n"))
}

use rust_ordo_ext::{Graph, Sinks};

#[test]
fn library_reports_sorted_sink_nodes() {
    let mut graph = Graph::new();
    graph.link("a", "z");
    graph.link("a", "m");
    graph.link("b", "z");
    graph.link("b", "m");

    // a and b have outgoing edges; z and m never appear as a source, so they
    // are sinks. They are first seen in insertion order z, m but reported sorted.
    assert_eq!(
        graph.sinks(),
        Sinks {
            nodes: vec!["m".to_string(), "z".to_string()],
            count: 2,
        }
    );
}

#[test]
fn library_no_sinks_in_a_cycle() {
    let mut graph = Graph::new();
    graph.link("x", "y");
    graph.link("y", "x");

    // Every node has an outgoing edge, so there are no sinks.
    assert_eq!(
        graph.sinks(),
        Sinks {
            nodes: Vec::new(),
            count: 0,
        }
    );
}

#[test]
fn cli_contract_byte_for_byte() {
    let input = "link a z\nlink a m\nlink b z\nlink b m\nsinks\n";
    let (code, stdout, stderr) = run_app(input);
    assert_eq!(code, 0, "stderr was: {}", stderr);
    assert_eq!(stdout, "sinks=m,z count=2\n");
}

#[test]
fn cli_contract_no_sinks_prints_none() {
    let input = "link x y\nlink y x\nsinks\n";
    let (code, stdout, stderr) = run_app(input);
    assert_eq!(code, 0, "stderr was: {}", stderr);
    assert_eq!(stdout, "sinks=none count=0\n");
}

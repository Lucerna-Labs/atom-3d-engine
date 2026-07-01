use std::io::Write;
use std::process::{Command, Stdio};

fn run_app(input: &str) -> (i32, String, String) {
    let exe = env!("CARGO_BIN_EXE_rust_ordo_ext");
    let mut child = Command::new(exe).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().expect("spawn app");
    child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    (output.status.code().unwrap_or(-1), String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n"), String::from_utf8_lossy(&output.stderr).replace("\r\n", "\n"))
}

use rust_ordo_ext::{Adjacency, Graph};

#[test]
fn library_contract_reversed_adjacency() {
    let mut graph = Graph::new();
    graph.link("a", "c");
    graph.link("b", "c");
    graph.link("c", "d");
    graph.link("a", "d");

    // After reversing, c points to its original sources a and b, in order.
    assert_eq!(
        graph.reversed_adjacency("c"),
        Adjacency {
            node: "c".to_string(),
            targets: vec!["a".to_string(), "b".to_string()],
            count: 2,
        }
    );

    // d's reversed neighbors are c and a (edge insertion order).
    assert_eq!(
        graph.reversed_adjacency("d"),
        Adjacency {
            node: "d".to_string(),
            targets: vec!["c".to_string(), "a".to_string()],
            count: 2,
        }
    );

    // A node with no incoming edges has no reversed adjacency.
    assert_eq!(
        graph.reversed_adjacency("a"),
        Adjacency {
            node: "a".to_string(),
            targets: Vec::new(),
            count: 0,
        }
    );
}

#[test]
fn cli_contract_byte_for_byte() {
    let input = "link a c\nlink b c\nlink c d\nlink a d\nreversed c\nreversed d\nreversed a\n";
    let (code, stdout, stderr) = run_app(input);
    assert_eq!(code, 0, "stderr was: {}", stderr);
    assert_eq!(stdout, "c>a>b\nd>c>a\na>none\n");
}

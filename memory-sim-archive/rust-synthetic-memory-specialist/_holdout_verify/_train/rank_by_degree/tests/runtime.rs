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
fn library_ranks_by_out_degree_then_alphabetical() {
    use rust_ordo_ext::{Graph, Ranking};
    let mut graph = Graph::new();
    graph.link("a", "b");
    graph.link("a", "c");
    graph.link("b", "c");
    graph.link("d", "a");

    // Out-degrees: a=2, b=1, d=1, c=0. b and d tie at 1, so they sort
    // alphabetically as b then d. c only appears as a target, so it ranks last
    // with out-degree 0.
    assert_eq!(
        graph.rank(),
        Ranking {
            nodes: vec![
                "a".to_string(),
                "b".to_string(),
                "d".to_string(),
                "c".to_string(),
            ],
        }
    );
}

#[test]
fn library_ranking_is_empty_with_no_links() {
    use rust_ordo_ext::{Graph, Ranking};
    let graph = Graph::new();
    assert_eq!(graph.rank(), Ranking { nodes: Vec::new() });
}

#[test]
fn cli_ranks_nodes_and_reports_none() {
    let input = "rank\nlink a b\nlink a c\nlink b c\nlink d a\nrank\n";
    let (code, stdout, stderr) = run_app(input);
    assert_eq!(code, 0, "stderr: {}", stderr);
    // First query: no links yet, so "none". Second query: a (out 2), then the
    // b/d tie broken alphabetically, then c (out 0) last -> "a>b>d>c".
    assert_eq!(stdout, "none\na>b>d>c\n");
}

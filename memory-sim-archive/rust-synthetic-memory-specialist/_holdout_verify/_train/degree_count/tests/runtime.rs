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
fn library_degree_counts_out_and_in() {
    use rust_ordo_ext::{Degree, Graph};
    let mut graph = Graph::new();
    graph.link("a", "b");
    graph.link("a", "c");
    graph.link("b", "c");
    graph.link("d", "a");

    assert_eq!(
        graph.degree("a"),
        Degree {
            node: "a".to_string(),
            out_degree: 2,
            in_degree: 1,
        }
    );
    assert_eq!(
        graph.degree("c"),
        Degree {
            node: "c".to_string(),
            out_degree: 0,
            in_degree: 2,
        }
    );
    // A node never mentioned has zero degree.
    assert_eq!(
        graph.degree("z"),
        Degree {
            node: "z".to_string(),
            out_degree: 0,
            in_degree: 0,
        }
    );
}

#[test]
fn cli_reports_out_and_in_degree() {
    let input = "link a b\nlink a c\nlink b c\nlink d a\ndegree a\ndegree c\ndegree z\n";
    let (code, stdout, stderr) = run_app(input);
    assert_eq!(code, 0, "stderr: {}", stderr);
    assert_eq!(stdout, "out=2 in=1\nout=0 in=2\nout=0 in=0\n");
}

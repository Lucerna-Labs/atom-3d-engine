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
fn library_contract() {
    use rust_ordo_ext::{RouteBook, RouteEntry};

    let mut book = RouteBook::new();
    book.assign("express", &["a", "b", "c"]);
    book.assign("local", &["a", "x", "b", "y", "c"]);

    assert_eq!(
        book.lookup("express"),
        RouteEntry {
            label: "express".to_string(),
            stops: vec!["a".to_string(), "b".to_string(), "c".to_string()],
        }
    );

    // Unknown label yields an empty route, echoing the queried label.
    assert_eq!(
        book.lookup("ghost"),
        RouteEntry {
            label: "ghost".to_string(),
            stops: Vec::new(),
        }
    );

    // Reassigning a label overwrites its route in place.
    book.assign("express", &["a", "d", "c"]);
    assert_eq!(
        book.lookup("express"),
        RouteEntry {
            label: "express".to_string(),
            stops: vec!["a".to_string(), "d".to_string(), "c".to_string()],
        }
    );
}

#[test]
fn cli_byte_exact() {
    let input = "\
assign express a b c
assign local a x b y c
lookup express
lookup local
lookup ghost
assign express a d c
lookup express
";
    let expected = "\
express=a>b>c
local=a>x>b>y>c
ghost=none
express=a>d>c
";
    let (code, stdout, stderr) = run_app(input);
    assert_eq!(code, 0, "exit code; stderr={}", stderr);
    assert_eq!(stdout, expected);
}

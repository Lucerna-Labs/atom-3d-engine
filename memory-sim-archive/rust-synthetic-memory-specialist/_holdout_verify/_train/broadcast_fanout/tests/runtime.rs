use std::io::Write;
use std::process::{Command, Stdio};

fn run_app(input: &str) -> (i32, String, String) {
    let exe = env!("CARGO_BIN_EXE_rust_ordo_ext");
    let mut child = Command::new(exe).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().expect("spawn app");
    child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    (output.status.code().unwrap_or(-1), String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n"), String::from_utf8_lossy(&output.stderr).replace("\r\n", "\n"))
}

use rust_ordo_ext::{Fabric, Fanout};

#[test]
fn library_contract_transitive_fanout() {
    let mut fabric = Fabric::new();
    fabric.link("a", "b");
    fabric.link("a", "c");
    fabric.link("b", "d");
    fabric.link("c", "d");
    fabric.link("d", "e");

    let fanout = fabric.broadcast("a");
    assert_eq!(
        fanout,
        Fanout {
            deliveries: vec![
                "a>b".to_string(),
                "a>c".to_string(),
                "a>d".to_string(),
                "a>e".to_string(),
            ],
            count: 4,
        }
    );

    // A source with no outgoing edges fans out to nothing.
    let empty = fabric.broadcast("e");
    assert_eq!(
        empty,
        Fanout {
            deliveries: Vec::new(),
            count: 0,
        }
    );
}

#[test]
fn cli_contract_byte_for_byte() {
    let input = "link a b\nlink a c\nlink b d\nlink c d\nlink d e\nbroadcast a\nbroadcast e\n";
    let (code, stdout, stderr) = run_app(input);
    assert_eq!(code, 0, "stderr was: {}", stderr);
    assert_eq!(stdout, "deliveries=a>b,a>c,a>d,a>e count=4\ndeliveries=none count=0\n");
}

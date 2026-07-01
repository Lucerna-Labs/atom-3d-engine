use std::io::Write;
use std::process::{Command, Stdio};

fn run_app(input: &str) -> (i32, String, String) {
    let exe = env!("CARGO_BIN_EXE_rust_ordo_ext");
    let mut child = Command::new(exe).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().expect("spawn app");
    child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    (output.status.code().unwrap_or(-1), String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n"), String::from_utf8_lossy(&output.stderr).replace("\r\n", "\n"))
}

use rust_ordo_ext::{Delivery, Mesh};

#[test]
fn library_contract_tagged_broadcast() {
    let mut mesh = Mesh::new();
    // Topology: a -> b -> d, a -> c -> d, d -> e.
    mesh.link("a", "b");
    mesh.link("a", "c");
    mesh.link("b", "d");
    mesh.link("c", "d");
    mesh.link("d", "e");
    // Tags: only some downstream nodes carry "alert".
    mesh.tag("b", "alert");
    mesh.tag("d", "alert");
    mesh.tag("e", "info");
    mesh.tag("c", "info");

    // Broadcast "alert" from a: reached in BFS order is b, c, d, e; only b and d
    // carry "alert", so deliveries are a>b, a>d.
    let alert = mesh.broadcast("a", "alert");
    assert_eq!(
        alert,
        Delivery {
            source: "a".to_string(),
            tag: "alert".to_string(),
            deliveries: vec!["a>b".to_string(), "a>d".to_string()],
            count: 2,
        }
    );

    // A tag carried by no reachable node yields no deliveries.
    let none = mesh.broadcast("a", "missing");
    assert_eq!(
        none,
        Delivery {
            source: "a".to_string(),
            tag: "missing".to_string(),
            deliveries: Vec::new(),
            count: 0,
        }
    );
}

#[test]
fn cli_contract_byte_for_byte() {
    let input = "link a b\nlink a c\nlink b d\nlink c d\nlink d e\ntag b alert\ntag d alert\ntag e info\ntag c info\nbroadcast a alert\nbroadcast a info\nbroadcast a missing\n";
    let (code, stdout, stderr) = run_app(input);
    assert_eq!(code, 0, "stderr was: {}", stderr);
    assert_eq!(
        stdout,
        "tag=alert deliveries=a>b,a>d count=2\ntag=info deliveries=a>c,a>e count=2\ntag=missing deliveries=none count=0\n"
    );
}

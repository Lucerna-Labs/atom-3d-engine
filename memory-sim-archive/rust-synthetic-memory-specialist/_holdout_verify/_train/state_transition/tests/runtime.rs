use std::io::Write;
use std::process::{Command, Stdio};

fn run_app(input: &str) -> (i32, String, String) {
    let exe = env!("CARGO_BIN_EXE_rust_ordo_ext");
    let mut child = Command::new(exe).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().expect("spawn app");
    child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    (output.status.code().unwrap_or(-1), String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n"), String::from_utf8_lossy(&output.stderr).replace("\r\n", "\n"))
}

use rust_ordo_ext::{NodeState, StateMachine};

#[test]
fn contract_library() {
    // A full forward run reaches done.
    let mut machine = StateMachine::new();
    machine.event("n1", "start");
    machine.event("n1", "finish");
    assert_eq!(
        machine.state("n1"),
        NodeState {
            node: "n1".to_string(),
            state: "done".to_string(),
        }
    );

    // A single start stops at busy.
    let mut machine = StateMachine::new();
    machine.event("n2", "start");
    assert_eq!(
        machine.state("n2"),
        NodeState {
            node: "n2".to_string(),
            state: "busy".to_string(),
        }
    );

    // Out-of-order and unknown events are ignored: finish before start does
    // nothing, so the node stays open.
    let mut machine = StateMachine::new();
    machine.event("n3", "finish");
    machine.event("n3", "bogus");
    assert_eq!(
        machine.state("n3"),
        NodeState {
            node: "n3".to_string(),
            state: "open".to_string(),
        }
    );

    // An extra finish past done is ignored; state is idempotent at the end.
    let mut machine = StateMachine::new();
    machine.event("n4", "start");
    machine.event("n4", "finish");
    machine.event("n4", "finish");
    assert_eq!(
        machine.state("n4"),
        NodeState {
            node: "n4".to_string(),
            state: "done".to_string(),
        }
    );

    // A node never touched reports open.
    let machine = StateMachine::new();
    assert_eq!(
        machine.state("ghost"),
        NodeState {
            node: "ghost".to_string(),
            state: "open".to_string(),
        }
    );
}

#[test]
fn contract_cli() {
    let (code, out, err) = run_app("event a start\nevent a finish\nstate a\n");
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "a=done\n");
}

#[test]
fn contract_cli_partial() {
    let (code, out, err) = run_app("event b start\nstate b\n");
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "b=busy\n");
}

#[test]
fn contract_cli_unknown_node() {
    let (code, out, err) = run_app("state c\n");
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "c=open\n");
}

use std::io::Write;
use std::process::{Command, Stdio};

fn run_app(input: &str) -> (i32, String, String) {
    let exe = env!("CARGO_BIN_EXE_rust_ordo_ext");
    let mut child = Command::new(exe).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().expect("spawn app");
    child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    (output.status.code().unwrap_or(-1), String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n"), String::from_utf8_lossy(&output.stderr).replace("\r\n", "\n"))
}

use rust_ordo_ext::{Buffer, Sweep};

#[test]
fn alive_keeps_entries_with_ttl_strictly_greater_than_now() {
    let mut b = Buffer::new();
    b.add("alpha", 10);
    b.add("beta", 20);
    b.add("gamma", 30);
    // At T=20, alpha (10) and beta (20) are gone; only gamma (30 > 20) survives.
    assert_eq!(
        b.alive(20),
        Sweep {
            now: 20,
            alive: vec!["gamma".to_string()],
            count: 1,
        }
    );
    // At T=5 every entry is still alive, reported in insertion order.
    assert_eq!(
        b.alive(5),
        Sweep {
            now: 5,
            alive: vec!["alpha".to_string(), "beta".to_string(), "gamma".to_string()],
            count: 3,
        }
    );
    // At T=30 nothing survives (30 is not strictly greater than 30).
    assert_eq!(
        b.alive(30),
        Sweep { now: 30, alive: Vec::new(), count: 0 }
    );
}

#[test]
fn cli_prints_exact_contract_line() {
    let input = "add alpha 10\nadd beta 20\nadd gamma 30\nsweep 20\nsweep 5\nsweep 30\n";
    let (code, stdout, stderr) = run_app(input);
    assert_eq!(code, 0);
    assert_eq!(stderr, "");
    assert_eq!(
        stdout,
        "now=20 alive=gamma count=1\nnow=5 alive=alpha>beta>gamma count=3\nnow=30 alive=none count=0\n"
    );
}

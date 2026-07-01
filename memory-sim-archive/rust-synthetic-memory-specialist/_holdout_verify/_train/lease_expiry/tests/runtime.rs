use std::io::Write;
use std::process::{Command, Stdio};

fn run_app(input: &str) -> (i32, String, String) {
    let exe = env!("CARGO_BIN_EXE_rust_ordo_ext");
    let mut child = Command::new(exe).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().expect("spawn app");
    child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    (output.status.code().unwrap_or(-1), String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n"), String::from_utf8_lossy(&output.stderr).replace("\r\n", "\n"))
}

use rust_ordo_ext::{Expiry, Ledger};

#[test]
fn ledger_partitions_expired_and_active_by_ttl() {
    let mut l = Ledger::new();
    l.record("alpha", 10);
    l.record("beta", 20);
    l.record("gamma", 30);
    assert_eq!(
        l.expire(20),
        Expiry {
            expired: vec!["alpha".to_string(), "beta".to_string()],
            active: vec!["gamma".to_string()],
        }
    );
}

#[test]
fn cli_reports_expired_and_active_lease_ids() {
    let input = "record alpha 10\nrecord beta 20\nrecord gamma 30\nexpire 20\n";
    let (code, stdout, stderr) = run_app(input);
    assert_eq!(code, 0);
    assert_eq!(stderr, "");
    assert_eq!(stdout, "expired=alpha,beta active=gamma\n");
}

#[test]
fn cli_uses_none_when_a_side_is_empty() {
    let input = "record alpha 10\nrecord beta 20\nexpire 5\n";
    let (code, stdout, _stderr) = run_app(input);
    assert_eq!(code, 0);
    assert_eq!(stdout, "expired=none active=alpha,beta\n");
}

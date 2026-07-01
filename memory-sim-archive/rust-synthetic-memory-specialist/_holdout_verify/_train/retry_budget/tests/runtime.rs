use std::io::Write;
use std::process::{Command, Stdio};

fn run_app(input: &str) -> (i32, String, String) {
    let exe = env!("CARGO_BIN_EXE_rust_ordo_ext");
    let mut child = Command::new(exe).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().expect("spawn app");
    child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    (output.status.code().unwrap_or(-1), String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n"), String::from_utf8_lossy(&output.stderr).replace("\r\n", "\n"))
}

use rust_ordo_ext::{Resolution, RetryRuntime};

#[test]
fn contract_first_success_within_budget() {
    let mut rt = RetryRuntime::new();
    // a: budget 1 (2 attempts) but both fail -> exhausts 1 retry, fails.
    rt.add("a", 1);
    rt.attempt("a", "fail");
    rt.attempt("a", "fail");
    // b: budget 2 (3 attempts), succeeds on the second attempt (1 retry).
    rt.add("b", 2);
    rt.attempt("b", "fail");
    rt.attempt("b", "ok");
    // c: would succeed immediately, but b already won so c is never visited.
    rt.add("c", 0);
    rt.attempt("c", "ok");

    assert_eq!(
        rt.resolve(),
        Resolution {
            winner: "b".to_string(),
            signals: vec!["retry:a:1".to_string(), "retry:b:1".to_string()],
        }
    );

    // A candidate that succeeds on its very first attempt consumes no retries
    // and emits no signal.
    let mut rt2 = RetryRuntime::new();
    rt2.add("x", 3);
    rt2.attempt("x", "ok");
    assert_eq!(
        rt2.resolve(),
        Resolution {
            winner: "x".to_string(),
            signals: vec![],
        }
    );

    // Budget caps how many retries are spent: budget 1 means only 1 retry, so
    // a third "ok" beyond the allowance is never reached -> failure, "none".
    let mut rt3 = RetryRuntime::new();
    rt3.add("y", 1);
    rt3.attempt("y", "fail");
    rt3.attempt("y", "fail");
    rt3.attempt("y", "ok");
    assert_eq!(
        rt3.resolve(),
        Resolution {
            winner: "none".to_string(),
            signals: vec!["retry:y:1".to_string()],
        }
    );
}

#[test]
fn cli_first_success_within_budget() {
    let input = "add a 1\nattempt a fail\nattempt a fail\nadd b 2\nattempt b fail\nattempt b ok\nadd c 0\nattempt c ok\nresolve\n";
    let (code, stdout, stderr) = run_app(input);
    assert_eq!(code, 0, "stderr: {}", stderr);
    assert_eq!(stdout, "winner=b signals=retry:a:1,retry:b:1\n");
}

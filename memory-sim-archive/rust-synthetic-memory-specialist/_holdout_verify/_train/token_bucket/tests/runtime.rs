use std::io::Write;
use std::process::{Command, Stdio};

fn run_app(input: &str) -> (i32, String, String) {
    let exe = env!("CARGO_BIN_EXE_rust_ordo_ext");
    let mut child = Command::new(exe).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().expect("spawn app");
    child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    (output.status.code().unwrap_or(-1), String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n"), String::from_utf8_lossy(&output.stderr).replace("\r\n", "\n"))
}

use rust_ordo_ext::{Grant, TokenBucket};

#[test]
fn token_bucket_contract() {
    let mut rt = TokenBucket::new();
    rt.set_capacity("a", 2);
    // First two requests are allowed, draining the bucket.
    assert_eq!(
        rt.request("a"),
        Grant { node: "a".to_string(), allowed: true, remaining: 1, signals: vec![] }
    );
    assert_eq!(
        rt.request("a"),
        Grant { node: "a".to_string(), allowed: true, remaining: 0, signals: vec![] }
    );
    // Empty bucket denies and emits deny:<node>.
    assert_eq!(
        rt.request("a"),
        Grant { node: "a".to_string(), allowed: false, remaining: 0, signals: vec!["deny:a".to_string()] }
    );
    // Unknown node denies with zero remaining.
    assert_eq!(
        rt.request("b"),
        Grant { node: "b".to_string(), allowed: false, remaining: 0, signals: vec!["deny:b".to_string()] }
    );
    // Buckets are independent.
    rt.set_capacity("c", 1);
    assert_eq!(
        rt.request("c"),
        Grant { node: "c".to_string(), allowed: true, remaining: 0, signals: vec![] }
    );
}

#[test]
fn token_bucket_cli() {
    let script = "capacity a 2\nrequest a\nrequest a\nrequest a\nrequest b\n";
    let (code, out, err) = run_app(script);
    assert_eq!(code, 0, "stderr={err}");
    assert_eq!(
        out,
        "node=a allowed=true remaining=1 signals=none\nnode=a allowed=true remaining=0 signals=none\nnode=a allowed=false remaining=0 signals=deny:a\nnode=b allowed=false remaining=0 signals=deny:b\n"
    );
}

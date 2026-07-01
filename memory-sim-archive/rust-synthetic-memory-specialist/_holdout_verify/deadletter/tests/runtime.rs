use std::io::Write;
use std::process::{Command, Stdio};

fn run_app(input: &str) -> (i32, String, String) {
    let exe = env!("CARGO_BIN_EXE_rust_ordo_ext");
    let mut child = Command::new(exe)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn app");
    child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    (
        output.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n"),
        String::from_utf8_lossy(&output.stderr).replace("\r\n", "\n"),
    )
}

use rust_ordo_ext::{DeadletterRuntime, Delivery};

#[test]
fn deadletter_contract() {
    let mut rt = DeadletterRuntime::new();
    rt.connect("input", "filter");
    rt.connect("filter", "model");
    rt.add_target("model");
    assert_eq!(
        rt.route("input"),
        Delivery {
            path: vec!["input".to_string(), "filter".to_string(), "model".to_string()],
            delivered: true,
            signals: vec![],
        }
    );

    let mut rt2 = DeadletterRuntime::new();
    rt2.connect("a", "b");
    assert_eq!(
        rt2.route("a"),
        Delivery {
            path: vec!["a".to_string(), "b".to_string()],
            delivered: false,
            signals: vec!["deadletter:b".to_string()],
        }
    );
}

#[test]
fn deadletter_cli() {
    let script = "connect input filter\nconnect filter model\ntarget model\nroute input\n";
    let (code, out, err) = run_app(script);
    assert_eq!(code, 0, "stderr={err}");
    assert_eq!(out, "path=input>filter>model delivered=true signals=none\n");
}

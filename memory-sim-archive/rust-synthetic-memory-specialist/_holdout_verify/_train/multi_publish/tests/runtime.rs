use std::io::Write;
use std::process::{Command, Stdio};

fn run_app(input: &str) -> (i32, String, String) {
    let exe = env!("CARGO_BIN_EXE_rust_ordo_ext");
    let mut child = Command::new(exe).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().expect("spawn app");
    child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    (output.status.code().unwrap_or(-1), String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n"), String::from_utf8_lossy(&output.stderr).replace("\r\n", "\n"))
}

use rust_ordo_ext::{Broker, PublishLog, Publication};

#[test]
fn multi_publish_contract() {
    let mut rt = Broker::new();
    rt.publish("hello");
    rt.publish("world");
    rt.publish("again");
    assert_eq!(
        rt.log(),
        PublishLog {
            entries: vec![
                Publication { thread: 1, payload: "hello".to_string() },
                Publication { thread: 2, payload: "world".to_string() },
                Publication { thread: 3, payload: "again".to_string() },
            ],
            count: 3,
        }
    );

    let rt2 = Broker::new();
    assert_eq!(
        rt2.log(),
        PublishLog {
            entries: vec![],
            count: 0,
        }
    );
}

#[test]
fn multi_publish_cli() {
    let script = "publish hello\npublish world\npublish again\n";
    let (code, out, err) = run_app(script);
    assert_eq!(code, 0, "stderr={err}");
    assert_eq!(out, "thread=1 msg=hello\nthread=2 msg=world\nthread=3 msg=again\n");
}

#[test]
fn multi_publish_cli_log() {
    let script = "publish alpha\npublish beta\nlog\n";
    let (code, out, err) = run_app(script);
    assert_eq!(code, 0, "stderr={err}");
    assert_eq!(out, "thread=1 msg=alpha\nthread=2 msg=beta\nentries=1>alpha,2>beta count=2\n");
}

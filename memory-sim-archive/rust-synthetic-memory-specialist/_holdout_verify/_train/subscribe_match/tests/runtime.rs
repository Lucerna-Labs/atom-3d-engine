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
    use rust_ordo_ext::{Broker, Match};

    let mut broker = Broker::new();
    broker.subscribe("alpha", "orders");
    broker.subscribe("beta", "orders");
    broker.subscribe("gamma", "alerts");
    broker.subscribe("alpha", "orders"); // duplicate node on same topic is ignored

    // Subscribers reported in registration order.
    assert_eq!(
        broker.publish("orders"),
        Match {
            topic: "orders".to_string(),
            subscribers: vec!["alpha".to_string(), "beta".to_string()],
        }
    );

    // Single subscriber.
    assert_eq!(
        broker.publish("alerts"),
        Match {
            topic: "alerts".to_string(),
            subscribers: vec!["gamma".to_string()],
        }
    );

    // Topic with no subscribers yields an empty subscriber list.
    assert_eq!(
        broker.publish("metrics"),
        Match {
            topic: "metrics".to_string(),
            subscribers: Vec::new(),
        }
    );
}

#[test]
fn cli_contract() {
    let input = "\
subscribe alpha orders
subscribe beta orders
subscribe gamma alerts
publish orders
publish alerts
publish metrics
";
    let (code, stdout, stderr) = run_app(input);
    assert_eq!(code, 0, "stderr: {}", stderr);
    assert_eq!(
        stdout,
        "topic=orders subscribers=alpha,beta\n\
topic=alerts subscribers=gamma\n\
topic=metrics subscribers=none\n"
    );
}

use std::io::Write;
use std::process::{Command, Stdio};

fn run_app(input: &str) -> (i32, String, String) {
    let exe = env!("CARGO_BIN_EXE_rust_ordo_ext");
    let mut child = Command::new(exe).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().expect("spawn app");
    child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    (output.status.code().unwrap_or(-1), String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n"), String::from_utf8_lossy(&output.stderr).replace("\r\n", "\n"))
}

use rust_ordo_ext::{Filtered, SignalBus};

#[test]
fn filter_keeps_prefixed_signals_in_insertion_order() {
    let mut bus = SignalBus::new();
    bus.collect("sensor.temp");
    bus.collect("motor.rpm");
    bus.collect("sensor.humidity");
    bus.collect("sensor.temp"); // duplicates are preserved, not collapsed
    assert_eq!(
        bus.filter("sensor."),
        Filtered {
            prefix: "sensor.".to_string(),
            signals: vec![
                "sensor.temp".to_string(),
                "sensor.humidity".to_string(),
                "sensor.temp".to_string(),
            ],
            count: 3,
        }
    );
    // A prefix that matches nothing yields no signals and a zero count.
    assert_eq!(
        bus.filter("radio."),
        Filtered { prefix: "radio.".to_string(), signals: Vec::new(), count: 0 }
    );
}

#[test]
fn cli_prints_exact_contract_line() {
    let input = "collect sensor.temp\ncollect motor.rpm\ncollect sensor.humidity\nfilter sensor.\nfilter motor.\nfilter radio.\n";
    let (code, stdout, stderr) = run_app(input);
    assert_eq!(code, 0);
    assert_eq!(stderr, "");
    assert_eq!(
        stdout,
        "prefix=sensor. signals=sensor.temp>sensor.humidity count=2\nprefix=motor. signals=motor.rpm count=1\nprefix=radio. signals=none count=0\n"
    );
}

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
fn library_fan_counts_out_degree() {
    use rust_ordo_ext::{Fabric, Fanout};
    let mut fabric = Fabric::new();
    fabric.link("a", "b");
    fabric.link("a", "c");
    fabric.link("a", "d");
    fabric.link("b", "c");

    // Node "a" fans out to three targets.
    assert_eq!(
        fabric.fan("a"),
        Fanout {
            node: "a".to_string(),
            degree: 3,
        }
    );
    // Node "b" fans out to one target.
    assert_eq!(
        fabric.fan("b"),
        Fanout {
            node: "b".to_string(),
            degree: 1,
        }
    );
    // A leaf with only incoming edges has zero fan-out.
    assert_eq!(
        fabric.fan("c"),
        Fanout {
            node: "c".to_string(),
            degree: 0,
        }
    );
    // A node never mentioned has zero fan-out.
    assert_eq!(
        fabric.fan("z"),
        Fanout {
            node: "z".to_string(),
            degree: 0,
        }
    );
}

#[test]
fn cli_reports_fan_out_degree() {
    let input = "link a b\nlink a c\nlink a d\nlink b c\nfan a\nfan b\nfan c\nfan z\n";
    let (code, stdout, stderr) = run_app(input);
    assert_eq!(code, 0, "stderr: {}", stderr);
    assert_eq!(stdout, "fan=3\nfan=1\nfan=0\nfan=0\n");
}

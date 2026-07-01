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
fn library_contract_hop_to_terminal() {
    use rust_ordo_ext::{Echo, Walk};
    let mut walk = Walk::new();
    walk.link("a", "b");
    walk.link("b", "c");
    walk.link("c", "d");
    // a has a second outgoing edge added later; first-outgoing wins.
    walk.link("a", "z");
    let echo = walk.hop("a");
    assert_eq!(
        echo,
        Echo {
            terminal: "d".to_string(),
            path: vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string()
            ],
        }
    );
}

#[test]
fn library_start_is_already_terminal() {
    use rust_ordo_ext::{Echo, Walk};
    let mut walk = Walk::new();
    walk.link("x", "y");
    let echo = walk.hop("q");
    assert_eq!(
        echo,
        Echo {
            terminal: "q".to_string(),
            path: vec!["q".to_string()],
        }
    );
}

#[test]
fn library_cycle_stops_at_last_new_node() {
    use rust_ordo_ext::{Echo, Walk};
    let mut walk = Walk::new();
    walk.link("a", "b");
    walk.link("b", "a");
    let echo = walk.hop("a");
    assert_eq!(
        echo,
        Echo {
            terminal: "b".to_string(),
            path: vec!["a".to_string(), "b".to_string()],
        }
    );
}

#[test]
fn cli_hop_echoes_terminal_and_path() {
    let input = "link a b\nlink b c\nlink c d\nlink a z\nhop a\n";
    let (code, stdout, stderr) = run_app(input);
    assert_eq!(code, 0, "stderr: {}", stderr);
    assert_eq!(stdout, "terminal=d path=a>b>c>d\n");
}

#[test]
fn cli_lone_start_is_terminal() {
    let input = "hop solo\n";
    let (code, stdout, _stderr) = run_app(input);
    assert_eq!(code, 0);
    assert_eq!(stdout, "terminal=solo path=solo\n");
}

use std::io::Write;
use std::process::{Command, Stdio};

fn run_app(input: &str) -> (i32, String, String) {
    let exe = env!("CARGO_BIN_EXE_rust_ordo_ext");
    let mut child = Command::new(exe).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().expect("spawn app");
    child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    (output.status.code().unwrap_or(-1), String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n"), String::from_utf8_lossy(&output.stderr).replace("\r\n", "\n"))
}

use rust_ordo_ext::{Catalog, TagMatch};

#[test]
fn select_returns_sorted_deduped_nodes_for_tag() {
    let mut c = Catalog::new();
    c.register("zeta", "core");
    c.register("alpha", "core");
    c.register("alpha", "core"); // duplicate assignment is collapsed
    c.register("beta", "edge");
    assert_eq!(
        c.select("core"),
        TagMatch {
            tag: "core".to_string(),
            nodes: vec!["alpha".to_string(), "zeta".to_string()],
            count: 2,
        }
    );
    // An unknown tag yields no nodes and a zero count.
    assert_eq!(
        c.select("ghost"),
        TagMatch { tag: "ghost".to_string(), nodes: Vec::new(), count: 0 }
    );
}

#[test]
fn cli_prints_exact_contract_line() {
    let input = "register zeta core\nregister alpha core\nregister beta edge\nselect core\nselect edge\nselect ghost\n";
    let (code, stdout, stderr) = run_app(input);
    assert_eq!(code, 0);
    assert_eq!(stderr, "");
    assert_eq!(
        stdout,
        "tag=core nodes=alpha,zeta count=2\ntag=edge nodes=beta count=1\ntag=ghost nodes=none count=0\n"
    );
}

use std::io::Write;
use std::process::{Command, Stdio};

fn run_app(input: &str) -> (i32, String, String) {
    let exe = env!("CARGO_BIN_EXE_rust_ordo_ext");
    let mut child = Command::new(exe).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().expect("spawn app");
    child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    (output.status.code().unwrap_or(-1), String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n"), String::from_utf8_lossy(&output.stderr).replace("\r\n", "\n"))
}

use rust_ordo_ext::{MergeFabric, MergeTrace};

#[test]
fn contract_library() {
    let mut fabric = MergeFabric::new();
    assert_eq!(fabric.flow(&["a", "b", "c"]), 1);
    assert_eq!(fabric.flow(&["x", "b", "z"]), 2);
    assert_eq!(fabric.flow(&["q", "r"]), 3);

    assert_eq!(
        fabric.merge("b"),
        MergeTrace {
            node: "b".to_string(),
            threads: vec![1, 2],
        }
    );
    assert_eq!(
        fabric.merge("r"),
        MergeTrace {
            node: "r".to_string(),
            threads: vec![3],
        }
    );
    assert_eq!(
        fabric.merge("zzz"),
        MergeTrace {
            node: "zzz".to_string(),
            threads: vec![],
        }
    );
}

#[test]
fn contract_cli() {
    let (code, out, err) = run_app("flow a>b>c\nflow x>b>z\nmerge b\n");
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(
        out,
        "thread=1 route=a>b>c\nthread=2 route=x>b>z\nmerge=b threads=1,2\n"
    );
}

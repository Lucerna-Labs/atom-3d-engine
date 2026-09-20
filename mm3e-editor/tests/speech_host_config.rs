//! Host-side optional dependency failures must not brick ordinary authoring.
#[cfg(target_os = "linux")]
#[test]
fn broken_optional_bundle_preserves_authoring_and_explicit_configuration_is_strict() {
    use serde_json::{json, Value};
    use std::{
        fs,
        io::Write,
        os::unix::fs::PermissionsExt,
        process::{Command, Stdio},
    };
    let root = std::env::temp_dir().join(format!("mm3e-speech-host-{}", std::process::id()));
    fs::create_dir(&root).unwrap();
    struct Cleanup(std::path::PathBuf);
    impl Drop for Cleanup {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }
    let _cleanup = Cleanup(root.clone());
    fs::create_dir(root.join("bin")).unwrap();
    fs::create_dir_all(root.join("tools/rhubarb/res")).unwrap();
    let binary = root.join("bin/mm3e-editor");
    fs::copy(env!("CARGO_BIN_EXE_mm3e-editor"), &binary).unwrap();
    let broken = root.join("tools/rhubarb/rhubarb");
    fs::write(&broken, b"deliberately invalid executable\n").unwrap();
    fs::set_permissions(&broken, fs::Permissions::from_mode(0o755)).unwrap();
    fs::write(root.join("tools/rhubarb/res/model"), b"owned failure fixture").unwrap();
    let mut child = Command::new(&binary)
        .args(["--root", root.to_str().unwrap()])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    for request in [
        json!({"id":"state","command":{"op":"speech_backend_state"}}),
        json!({"id":"create","expected_revision":0,"command":{"op":"apply","operations":[{"op":"create","object":{"id":"ball","shape":{"type":"sphere","radius":0.3}}},{"op":"set_settings","settings":{"width":8,"height":8,"quality":"preview","shadows":false,"ao":false}}]}}),
        json!({"id":"render","command":{"op":"render","path":"image.png"}}),
    ] {
        writeln!(child.stdin.as_mut().unwrap(), "{request}").unwrap();
    }
    drop(child.stdin.take());
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let responses: Vec<Value> =
        String::from_utf8(output.stdout).unwrap().lines().map(|s| serde_json::from_str(s).unwrap()).collect();
    assert!(responses.iter().all(|r| r["ok"] == true));
    assert_eq!(responses[0]["result"]["configured"], false);
    assert!(responses[0]["result"]["configuration_error"].is_string());
    assert!(root.join("image.png").is_file());
    let explicit = Command::new(&binary)
        .args(["--root", root.to_str().unwrap(), "--speech-backend", broken.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(!explicit.status.success());
    let mut bypass = Command::new(&binary)
        .args(["--root", root.to_str().unwrap(), "--no-speech-backend"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    writeln!(bypass.stdin.as_mut().unwrap(), "{}", json!({"id":"off","command":{"op":"speech_backend_state"}}))
        .unwrap();
    drop(bypass.stdin.take());
    let output = bypass.wait_with_output().unwrap();
    assert!(output.status.success());
    let response: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(response["result"]["configured"], false);
    assert!(response["result"]["configuration_error"].is_null());
}

//! Real process-control tests. Python fixtures below simulate failures and CLI
//! argument reporting only; they are not speech-recognition acceptance evidence.
#![cfg(target_os = "linux")]
use mm3e_editor::{speech::AnalyzeSpeech, speech_backend::SpeechBackend};
use serde_json::json;
use std::{
    fs,
    io::Read,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    sync::{Mutex, MutexGuard},
    time::{Duration, Instant},
};
static BACKEND_TEST_LOCK: Mutex<()> = Mutex::new(());
fn serialized_process_test() -> MutexGuard<'static, ()> {
    // Keep script writers out of concurrent forks in this test process. A fork
    // can inherit another thread's writable FD until exec closes CLOEXEC files,
    // transiently producing ETXTBSY even after that thread closed its own FD.
    BACKEND_TEST_LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}
struct Fixture(PathBuf);
impl Fixture {
    fn new(body: &str) -> Self {
        Self::with_version(body, "print('Rhubarb Lip Sync version 1.14.0')")
    }
    fn with_version(body: &str, version: &str) -> Self {
        let mut owned = None;
        for _ in 0..8 {
            let mut random = [0u8; 16];
            fs::File::open("/dev/urandom").unwrap().read_exact(&mut random).unwrap();
            let suffix: String = random.iter().map(|byte| format!("{byte:02x}")).collect();
            let candidate = std::env::temp_dir().join(format!("mm3e-speech-backend-{suffix}"));
            match fs::create_dir(&candidate) {
                Ok(()) => {
                    owned = Some(candidate);
                    break;
                }
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(error) => panic!("cannot reserve speech fixture directory: {error}"),
            }
        }
        // Existing paths are neither reused nor removed, including collisions
        // across process/PID namespaces that share the same temporary directory.
        let path = owned.expect("could not reserve a unique speech fixture directory");
        fs::create_dir_all(path.join("res/sub")).unwrap();
        fs::write(path.join("res/a.txt"), b"first resource").unwrap();
        fs::write(path.join("res/sub/b.txt"), b"second resource").unwrap();
        let version = version.lines().map(|line| format!("    {line}\n")).collect::<String>();
        let script = format!(
            "#!/usr/bin/python3\nimport json,os,sys,time\nfrom pathlib import Path\nif sys.argv[1:]==['--version']:\n{version}    sys.exit(0)\n{body}\n"
        );
        fs::write(path.join("rhubarb"), script).unwrap();
        fs::set_permissions(path.join("rhubarb"), fs::Permissions::from_mode(0o755)).unwrap();
        Self(path)
    }
    fn open(&self) -> SpeechBackend {
        SpeechBackend::open(&self.0.join("rhubarb")).unwrap()
    }
    fn analysis(&self, name: &str) -> PathBuf {
        let directory = self.0.join(name);
        fs::create_dir(&directory).unwrap();
        // Process fixtures do not decode audio; the real-backend test below does.
        fs::write(directory.join("input.wav"), b"fixture input, not audio").unwrap();
        directory
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn request() -> AnalyzeSpeech {
    serde_json::from_value(json!({"audio":"speech","directory":"analysis","timeout_seconds":2})).unwrap()
}
fn reaped(path: &Path) {
    let pid = fs::read_to_string(path).unwrap();
    assert!(!Path::new("/proc").join(pid.trim()).exists(), "backend process {} was not killed/reaped", pid.trim());
}

#[test]
fn direct_arguments_logs_and_identity_are_deterministic() {
    let _serial = serialized_process_test();
    let fixture =
        Fixture::new("print(json.dumps({'argv':sys.argv[1:]}))\nprint('{\"event\":\"done\"}',file=sys.stderr)");
    let backend = fixture.open();
    let identity = serde_json::to_value(backend.identity()).unwrap();
    let other = fixture.open();
    assert_eq!(identity, serde_json::to_value(other.identity()).unwrap());
    assert_eq!(identity["binary_sha256"].as_str().unwrap().len(), 64);
    assert_eq!(identity["resources_sha256"].as_str().unwrap().len(), 64);
    let directory = fixture.analysis("directory with spaces");
    fs::write(directory.join("dialogue.txt"), b"Hello. $(untrusted text remains file data)").unwrap();
    let mut request = request();
    request.recognizer = mm3e_editor::speech::Recognizer::Phonetic;
    request.extended_shapes = "GHX".into();
    request.dialogue_hint = Some("Hello. $(untrusted text remains file data)".into());
    let output = backend.run(&directory, &request).unwrap();
    assert_eq!(
        output["argv"],
        json!([
            "--recognizer",
            "phonetic",
            "--exportFormat",
            "json",
            "--extendedShapes",
            "GHX",
            "--threads",
            "2",
            "--machineReadable",
            "--dialogFile",
            directory.join("dialogue.txt"),
            directory.join("input.wav")
        ])
    );
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&fs::read(directory.join("raw.json")).unwrap()).unwrap(),
        output
    );
    assert_eq!(fs::read_to_string(directory.join("status.jsonl")).unwrap(), "{\"event\":\"done\"}\n");
    let raw = fs::read(directory.join("raw.json")).unwrap();
    assert!(backend.run(&directory, &request).is_err());
    assert_eq!(raw, fs::read(directory.join("raw.json")).unwrap(), "repeat must preserve prior output");
}

#[test]
fn timeout_kills_and_reaps_without_losing_partial_logs() {
    let _serial = serialized_process_test();
    let fixture = Fixture::new("Path('child.pid').write_text(str(os.getpid()))\nprint('{',flush=True)\nprint('working',file=sys.stderr,flush=True)\ntime.sleep(60)");
    let backend = fixture.open();
    let directory = fixture.analysis("timeout");
    let mut request = request();
    request.timeout_seconds = 1;
    let start = Instant::now();
    let error = backend.run(&directory, &request).unwrap_err();
    assert_eq!(error.code, "speech_timeout", "{}", error.message);
    assert!(start.elapsed() < Duration::from_secs(5));
    reaped(&directory.join("child.pid"));
    assert_eq!(fs::read(directory.join("raw.json")).unwrap(), b"{\n");
    assert_eq!(fs::read(directory.join("status.jsonl")).unwrap(), b"working\n");
}

#[test]
fn either_stream_is_capped_exactly_and_child_is_reaped() {
    let _serial = serialized_process_test();
    for stream in [1, 2] {
        let fixture = Fixture::new(&format!(
            "Path('child.pid').write_text(str(os.getpid()))\nwhile True:\n    os.write({stream}, b'x'*65536)"
        ));
        let backend = fixture.open();
        let directory = fixture.analysis("overflow");
        let error = backend.run(&directory, &request()).unwrap_err();
        assert_eq!(error.code, "speech_output_limit", "{}", error.message);
        reaped(&directory.join("child.pid"));
        let oversized = if stream == 1 { "raw.json" } else { "status.jsonl" };
        assert_eq!(fs::metadata(directory.join(oversized)).unwrap().len(), 2 * 1024 * 1024);
        assert!(fs::metadata(directory.join("raw.json")).unwrap().len() <= 2 * 1024 * 1024);
        assert!(fs::metadata(directory.join("status.jsonl")).unwrap().len() <= 2 * 1024 * 1024);
    }
}

#[test]
fn exact_output_limit_is_accepted_without_truncation() {
    let _serial = serialized_process_test();
    let fixture = Fixture::new("os.write(1,b'{}'+b' '*(2*1024*1024-2))\nos.write(2,b' '*(2*1024*1024))");
    let backend = fixture.open();
    let directory = fixture.analysis("exact limit");
    assert_eq!(backend.run(&directory, &request()).unwrap(), json!({}));
    assert_eq!(fs::metadata(directory.join("raw.json")).unwrap().len(), 2 * 1024 * 1024);
    assert_eq!(fs::metadata(directory.join("status.jsonl")).unwrap().len(), 2 * 1024 * 1024);
}

#[test]
fn changed_binary_or_resources_are_rejected_before_or_after_execution() {
    let _serial = serialized_process_test();
    for binary in [false, true] {
        let fixture = Fixture::new("print('{}')");
        let backend = fixture.open();
        let directory = fixture.analysis("before");
        let path = fixture.0.join(if binary { "rhubarb" } else { "res/a.txt" });
        let mut bytes = fs::read(&path).unwrap();
        bytes.extend_from_slice(b"\n");
        fs::write(path, bytes).unwrap();
        assert_eq!(backend.run(&directory, &request()).unwrap_err().code, "speech_backend_changed");
        assert!(!directory.join("raw.json").exists(), "changed backend must not launch");
    }
    let fixture =
        Fixture::new("(Path(__file__).parent/'res/a.txt').write_text('changed during analysis')\nprint('{}')");
    let backend = fixture.open();
    let directory = fixture.analysis("after");
    assert_eq!(backend.run(&directory, &request()).unwrap_err().code, "speech_backend_changed");
    assert_eq!(fs::read(directory.join("raw.json")).unwrap(), b"{}\n");
}

#[test]
fn failed_exit_and_invalid_json_preserve_diagnostic_evidence() {
    let _serial = serialized_process_test();
    for (body, expected) in [
        ("print('{}')\nprint('recognizer failed',file=sys.stderr)\nsys.exit(7)", "exited with"),
        ("print('this is not JSON')", "invalid JSON"),
    ] {
        let fixture = Fixture::new(body);
        let directory = fixture.analysis("failed");
        let error = fixture.open().run(&directory, &request()).unwrap_err();
        assert_eq!(error.code, "speech_backend");
        assert!(error.message.contains(expected), "{}", error.message);
        assert!(directory.join("raw.json").exists());
        assert!(directory.join("status.jsonl").exists());
    }
}

#[test]
fn version_probe_has_output_and_time_limits_and_reaps_its_process() {
    let _serial = serialized_process_test();
    for (version, expected) in [
        ("(Path(__file__).parent/'version.pid').write_text(str(os.getpid()))\nwhile True:\n    os.write(1,b'x'*65536)", "speech_output_limit"),
        ("(Path(__file__).parent/'version.pid').write_text(str(os.getpid()))\ntime.sleep(60)", "speech_timeout"),
    ] {
        let fixture = Fixture::with_version("print('{}')", version);
        let start = Instant::now();
        let error = SpeechBackend::open(&fixture.0.join("rhubarb")).err().expect("unbounded version accepted");
        assert_eq!(error.code, expected, "{}", error.message);
        assert!(start.elapsed() < Duration::from_secs(10));
        reaped(&fixture.0.join("version.pid"));
    }
}

#[test]
fn invalid_version_timeout_and_resource_symlinks_are_explicit() {
    let _serial = serialized_process_test();
    let fixture = Fixture::with_version("print('{}')", "print('Rhubarb Lip Sync version 9.9.9')");
    assert!(SpeechBackend::open(&fixture.0.join("rhubarb")).err().unwrap().message.contains("1.14.0"));
    let fixture = Fixture::new("print('{}')");
    let backend = fixture.open();
    let directory = fixture.analysis("invalid");
    let mut invalid = request();
    for timeout in [0, 601] {
        invalid.timeout_seconds = timeout;
        assert_eq!(backend.run(&directory, &invalid).unwrap_err().code, "invalid");
    }
    std::os::unix::fs::symlink(fixture.0.join("res/a.txt"), fixture.0.join("res/link")).unwrap();
    assert!(SpeechBackend::open(&fixture.0.join("rhubarb")).err().unwrap().message.contains("regular files"));
}

#[test]
fn recognizer_terminates_when_owner_is_killed_during_probe_or_analysis() {
    let _serial = serialized_process_test();
    let mut failures = Vec::new();
    for phase in ["probe", "analysis"] {
        let running = "(Path(__file__).parent/'recognizer.pid').write_text(str(os.getpid()))\ntime.sleep(60)";
        let fixture =
            if phase == "probe" { Fixture::with_version("print('{}')", running) } else { Fixture::new(running) };
        fixture.analysis("owner-death");
        // The supervisor is a separate test process and a Linux subreaper so it
        // can collect the orphan after killing its owner. No global process
        // ownership changes are made in this parallel test runner.
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "recognizer_owner_death_process_helper", "--ignored", "--nocapture", "--test-threads=1"])
            .env("MM3E_SPEECH_DEATH_HELPER", "supervisor")
            .env("MM3E_SPEECH_DEATH_ROOT", &fixture.0)
            .output()
            .unwrap();
        if !output.status.success() {
            failures.push(format!(
                "{phase} owner-death verification failed:\n{}\n{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            ));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
#[ignore = "subprocess-only owner-death lifecycle harness"]
fn recognizer_owner_death_process_helper() {
    let Ok(mode) = std::env::var("MM3E_SPEECH_DEATH_HELPER") else { return };
    let root = PathBuf::from(std::env::var_os("MM3E_SPEECH_DEATH_ROOT").unwrap());
    if mode == "owner" {
        let backend = SpeechBackend::open(&root.join("rhubarb")).unwrap();
        let mut request = request();
        request.timeout_seconds = 60;
        backend.run(&root.join("owner-death"), &request).unwrap();
        panic!("owner must be killed while its recognizer is running");
    }
    assert_eq!(mode, "supervisor");
    unsafe extern "C" {
        fn prctl(option: std::ffi::c_int, ...) -> std::ffi::c_int;
        fn kill(pid: std::ffi::c_int, signal: std::ffi::c_int) -> std::ffi::c_int;
        fn waitpid(pid: std::ffi::c_int, status: *mut std::ffi::c_int, options: std::ffi::c_int) -> std::ffi::c_int;
    }
    // PR_SET_CHILD_SUBREAPER is scoped to this isolated supervisor process.
    assert_eq!(unsafe { prctl(36, 1usize, 0usize, 0usize, 0usize) }, 0);
    let mut owner = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "recognizer_owner_death_process_helper", "--ignored", "--nocapture", "--test-threads=1"])
        .env("MM3E_SPEECH_DEATH_HELPER", "owner")
        .env("MM3E_SPEECH_DEATH_ROOT", &root)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .unwrap();
    let start = Instant::now();
    let pid = loop {
        if let Ok(pid) = fs::read_to_string(root.join("recognizer.pid")) {
            if let Ok(pid) = pid.parse::<i32>() {
                break pid;
            }
        }
        if start.elapsed() > Duration::from_secs(3) {
            let _ = owner.kill();
            let _ = owner.wait();
            panic!("recognizer did not reach its running marker");
        }
        std::thread::sleep(Duration::from_millis(5));
    };
    let stat = fs::read_to_string(format!("/proc/{pid}/stat")).unwrap();
    let fields: Vec<_> = stat.rsplit_once(')').unwrap().1.split_whitespace().collect();
    assert_ne!(fields[0], "Z", "recognizer had already exited before owner death");
    assert_eq!(fields[1].parse::<u32>().unwrap(), owner.id(), "recognizer is not the owner's direct child");
    owner.kill().unwrap();
    owner.wait().unwrap();
    let start = Instant::now();
    let mut status = 0;
    let terminated = loop {
        let result = unsafe { waitpid(pid, &mut status, 1) }; // WNOHANG
        if result == pid {
            break true;
        }
        assert_eq!(result, 0, "orphan adoption/wait failed: {}", std::io::Error::last_os_error());
        if start.elapsed() > Duration::from_secs(2) {
            break false;
        }
        std::thread::sleep(Duration::from_millis(5));
    };
    if !terminated {
        // Clean up the intentionally failing before-fix repro, but do not count
        // this supervisor intervention as successful owner-death handling.
        assert_eq!(unsafe { kill(pid, 9) }, 0);
        assert_eq!(unsafe { waitpid(pid, &mut status, 0) }, pid);
    }
    assert!(terminated, "recognizer PID {pid} survived owner SIGKILL and required supervisor cleanup");
    assert_eq!(status & 0x7f, 9, "recognizer did not terminate from SIGKILL: wait status {status}");
    assert!(!Path::new(&format!("/proc/{pid}")).exists(), "supervisor did not reap recognizer");
}

#[test]
#[ignore = "requires the verified local Rhubarb 1.14.0 distribution; actual recognizer acceptance"]
fn real_rhubarb_recognizes_original_speech_and_reports_exact_backend_identity() {
    let _serial = serialized_process_test();
    let executable = Path::new(env!("CARGO_MANIFEST_DIR")).join("../.dependencies/rhubarb-1.14.0-linux/rhubarb");
    let backend = SpeechBackend::open(&executable).unwrap();
    assert_eq!(backend.identity().name, "rhubarb");
    assert_eq!(backend.identity().version, "1.14.0");
    assert_eq!(backend.identity().binary_sha256, "d9aefa0627f843cd306333cc761368c4750ceddfa43236f0e607fa074249390c");
    let fixture = Fixture::new("print('{}')");
    let directory = fixture.analysis("actual recognition");
    fs::write(directory.join("input.wav"), include_bytes!("fixtures/dialogue/hello-reference.wav")).unwrap();
    fs::write(directory.join("dialogue.txt"), b"Hello. We make animated characters.").unwrap();
    let mut request = request();
    request.timeout_seconds = 120;
    request.dialogue_hint = Some("Hello. We make animated characters.".into());
    let result = backend.run(&directory, &request).unwrap();
    assert!(result["mouthCues"].as_array().unwrap().len() > 2, "{result}");
    assert!(result["metadata"]["duration"].as_f64().unwrap() > 0.0);
    assert!(fs::metadata(directory.join("raw.json")).unwrap().len() > 0);
    assert!(fs::metadata(directory.join("status.jsonl")).unwrap().len() > 0);
}

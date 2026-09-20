//! Bounded local Rhubarb adapter. The executable is trusted startup configuration,
//! never document content. Identity checks assume a cooperating local filesystem;
//! they detect persistent changes, not adversarial change-and-restore races.
//! Linux kills the direct recognizer child when its owner dies. Arbitrary
//! descendant processes are outside the trusted Rhubarb execution model.
use crate::{
    protocol::Failure,
    speech::{AnalyzeSpeech, BackendIdentity, Recognizer},
};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File, OpenOptions},
    io::{self, Read, Write},
    path::{Path, PathBuf},
    process::{Command, ExitStatus},
    time::Duration,
};
#[cfg(target_os = "linux")]
use std::{
    os::unix::process::CommandExt,
    process::{Child, Stdio},
    time::Instant,
};

const OUTPUT_LIMIT: usize = 2 * 1024 * 1024;
const VERSION_LIMIT: usize = 16 * 1024;
const VERSION_TIMEOUT: Duration = Duration::from_secs(5);
const RESOURCE_FILE_LIMIT: usize = 4096;
const RESOURCE_BYTES_LIMIT: u64 = 1024 * 1024 * 1024;
const BINARY_BYTES_LIMIT: u64 = 512 * 1024 * 1024;
const VERSION: &str = "1.14.0";

pub struct SpeechBackend {
    executable: PathBuf,
    identity: BackendIdentity,
}

impl SpeechBackend {
    pub fn open(executable: &Path) -> Result<Self, Failure> {
        let executable = executable.canonicalize().map_err(|e| io_failure("resolve speech backend", e))?;
        let identity = identify(&executable)?;
        let mut command = Command::new(&executable);
        command.arg("--version");
        let (status, output, diagnostics) =
            execute(&mut command, Box::new(io::sink()), Box::new(io::sink()), VERSION_TIMEOUT, VERSION_LIMIT)?;
        if !status.success() {
            return Err(backend_failure(format!(
                "speech backend version probe exited with {status}: {}",
                diagnostic(&diagnostics)
            )));
        }
        let text = std::str::from_utf8(&output).map_err(|_| backend_failure("speech backend version is not UTF-8"))?;
        if text.trim() != "Rhubarb Lip Sync version 1.14.0" {
            return Err(backend_failure(format!("speech backend must report Rhubarb Lip Sync version {VERSION}")));
        }
        ensure_identity(&identity, &identify(&executable)?)?;
        Ok(Self { executable, identity })
    }

    pub fn identity(&self) -> &BackendIdentity {
        &self.identity
    }

    pub fn run(&self, directory: &Path, request: &AnalyzeSpeech) -> Result<Value, Failure> {
        if !(1..=600).contains(&request.timeout_seconds) {
            return Err(Failure::invalid("speech timeout_seconds must be in 1..600"));
        }
        let mut shapes = std::collections::BTreeSet::new();
        if request.extended_shapes.chars().any(|shape| !matches!(shape, 'G' | 'H' | 'X') || !shapes.insert(shape)) {
            return Err(Failure::invalid("speech extended_shapes must contain distinct letters from GHX"));
        }
        let directory = directory.canonicalize().map_err(|e| io_failure("resolve speech analysis directory", e))?;
        if !directory.is_dir() {
            return Err(Failure::invalid("speech analysis directory must be a directory"));
        }
        let input = directory.join("input.wav");
        regular_file(&input)?;
        let dialogue = directory.join("dialogue.txt");
        if request.dialogue_hint.is_some() {
            regular_file(&dialogue)?;
        }
        ensure_identity(&self.identity, &identify(&self.executable)?)?;
        // Exclusive creation preserves every previous run and all failed output.
        // Parent authoring owns this fresh directory and its input files.
        let raw = create_log(&directory.join("raw.json"))?;
        let status_log = create_log(&directory.join("status.jsonl"))?;
        let mut command = Command::new(&self.executable);
        command
            .current_dir(&directory)
            .arg("--recognizer")
            .arg(match request.recognizer {
                Recognizer::English => "pocketSphinx",
                Recognizer::Phonetic => "phonetic",
            })
            .args(["--exportFormat", "json", "--extendedShapes"])
            .arg(&request.extended_shapes)
            .args(["--threads", "2", "--machineReadable"]);
        if request.dialogue_hint.is_some() {
            command.arg("--dialogFile").arg(dialogue);
        }
        command.arg(input);
        let result = execute(
            &mut command,
            Box::new(raw),
            Box::new(status_log),
            Duration::from_secs(u64::from(request.timeout_seconds)),
            OUTPUT_LIMIT,
        );
        // The child has been reaped before checking resources, including timeout,
        // excess output, failed exits, and capture errors. Keep original logs.
        let identity_result = identify(&self.executable).and_then(|current| ensure_identity(&self.identity, &current));
        match (result, identity_result) {
            (Err(mut failure), Err(identity)) => {
                failure.message.push_str(&format!("; {}", identity.message));
                Err(failure)
            }
            (Err(failure), _) | (_, Err(failure)) => Err(failure),
            (Ok((status, output, diagnostics)), Ok(())) => {
                if !status.success() {
                    return Err(backend_failure(format!(
                        "speech backend exited with {status}; raw.json and status.jsonl are retained: {}",
                        diagnostic(&diagnostics)
                    )));
                }
                serde_json::from_slice(&output).map_err(|e| {
                    backend_failure(format!("speech backend returned invalid JSON; raw.json is retained: {e}"))
                })
            }
        }
    }
}

fn backend_failure(message: impl Into<String>) -> Failure {
    Failure { code: "speech_backend", message: message.into() }
}
fn io_failure(context: &str, error: io::Error) -> Failure {
    Failure::io(format!("{context}: {error}"))
}
fn diagnostic(bytes: &[u8]) -> String {
    String::from_utf8_lossy(&bytes[..bytes.len().min(1024)]).into_owned()
}
fn create_log(path: &Path) -> Result<File, Failure> {
    OpenOptions::new().write(true).create_new(true).open(path).map_err(|e| io_failure("create speech output log", e))
}
fn regular_file(path: &Path) -> Result<fs::Metadata, Failure> {
    let metadata = fs::symlink_metadata(path).map_err(|e| io_failure("inspect speech file", e))?;
    if !metadata.file_type().is_file() {
        return Err(backend_failure(format!("speech file must be regular and not a symlink: {}", path.display())));
    }
    Ok(metadata)
}
fn file_digest(path: &Path, maximum: u64) -> Result<(u64, [u8; 32]), Failure> {
    let expected = regular_file(path)?.len();
    if expected > maximum {
        return Err(backend_failure("speech backend file exceeds the bounded identity size"));
    }
    let mut file = File::open(path).map_err(|e| io_failure("open speech identity file", e))?;
    let mut digest = Sha256::new();
    let mut count = 0u64;
    let mut buffer = [0u8; 64 * 1024];
    loop {
        let size = file.read(&mut buffer).map_err(|e| io_failure("hash speech identity file", e))?;
        if size == 0 {
            break;
        }
        count += size as u64;
        if count > maximum {
            return Err(backend_failure("speech backend file grew beyond the bounded identity size"));
        }
        digest.update(&buffer[..size]);
    }
    if count != expected || file.metadata().map_err(|e| io_failure("check speech identity file", e))?.len() != count {
        return Err(backend_failure("speech backend file changed while hashing"));
    }
    Ok((count, digest.finalize().into()))
}
fn identify(executable: &Path) -> Result<BackendIdentity, Failure> {
    let (_, binary) = file_digest(executable, BINARY_BYTES_LIMIT)?;
    let resources =
        executable.parent().ok_or_else(|| backend_failure("speech backend has no parent directory"))?.join("res");
    let metadata = fs::symlink_metadata(&resources).map_err(|e| io_failure("inspect speech resources", e))?;
    if !metadata.file_type().is_dir() {
        return Err(backend_failure("speech backend requires a regular res directory beside the executable"));
    }
    let mut files = Vec::new();
    fn visit(directory: &Path, depth: usize, files: &mut Vec<PathBuf>, entries: &mut usize) -> Result<(), Failure> {
        if depth > 32 {
            return Err(backend_failure("speech resources exceed directory depth 32"));
        }
        for entry in fs::read_dir(directory).map_err(|e| io_failure("enumerate speech resources", e))? {
            *entries += 1;
            if *entries > 8192 {
                return Err(backend_failure("speech resources exceed 8192 directory entries"));
            }
            let entry = entry.map_err(|e| io_failure("enumerate speech resource", e))?;
            let kind = entry.file_type().map_err(|e| io_failure("inspect speech resource type", e))?;
            if kind.is_dir() {
                visit(&entry.path(), depth + 1, files, entries)?;
            } else if kind.is_file() {
                files.push(entry.path());
                if files.len() > RESOURCE_FILE_LIMIT {
                    return Err(backend_failure("speech resources exceed 4096 files"));
                }
            } else {
                return Err(backend_failure("speech resources must contain only regular files and directories"));
            }
        }
        Ok(())
    }
    visit(&resources, 0, &mut files, &mut 0)?;
    if files.is_empty() {
        return Err(backend_failure("speech backend res directory is empty"));
    }
    files.sort();
    let mut hash = Sha256::new();
    hash.update(b"mm3e-rhubarb-resources-v1\0");
    hash.update((files.len() as u64).to_le_bytes());
    let mut total = 0u64;
    for file in files {
        let relative = file.strip_prefix(&resources).map_err(|_| backend_failure("speech resource escaped root"))?;
        let name = relative.to_str().ok_or_else(|| backend_failure("speech resource paths must be UTF-8"))?;
        let (length, digest) = file_digest(&file, RESOURCE_BYTES_LIMIT - total)?;
        total += length;
        hash.update((name.len() as u64).to_le_bytes());
        hash.update(name.as_bytes());
        hash.update(length.to_le_bytes());
        hash.update(digest);
    }
    Ok(BackendIdentity {
        name: "rhubarb".into(),
        version: VERSION.into(),
        binary_sha256: binary.iter().map(|byte| format!("{byte:02x}")).collect(),
        resources_sha256: format!("{:x}", hash.finalize()),
    })
}
fn ensure_identity(expected: &BackendIdentity, current: &BackendIdentity) -> Result<(), Failure> {
    if expected.binary_sha256 != current.binary_sha256 || expected.resources_sha256 != current.resources_sha256 {
        Err(Failure {
            code: "speech_backend_changed",
            message: "speech backend executable or resources changed; restart with the intended backend before analyzing audio".into(),
        })
    } else {
        Ok(())
    }
}

/// A guard also covers capture errors: every successfully spawned child is
/// either observed reaped or killed and waited for before this call returns.
#[cfg(target_os = "linux")]
struct Process {
    child: Child,
    reaped: bool,
}
#[cfg(target_os = "linux")]
impl Drop for Process {
    fn drop(&mut self) {
        if !self.reaped {
            let _ = self.child.kill();
            let _ = self.child.wait();
        }
    }
}

#[cfg(target_os = "linux")]
fn nonblocking(reader: &impl std::os::fd::AsRawFd) -> Result<(), Failure> {
    unsafe extern "C" {
        fn fcntl(fd: std::ffi::c_int, command: std::ffi::c_int, ...) -> std::ffi::c_int;
    }
    const GET_FLAGS: std::ffi::c_int = 3;
    const SET_FLAGS: std::ffi::c_int = 4;
    const NONBLOCK: std::ffi::c_int = 0o4000;
    // Linux pipe descriptors created by std::process are live here, and fcntl
    // neither takes ownership nor retains pointers into Rust memory.
    let flags = unsafe { fcntl(reader.as_raw_fd(), GET_FLAGS) };
    if flags < 0 || unsafe { fcntl(reader.as_raw_fd(), SET_FLAGS, flags | NONBLOCK) } < 0 {
        return Err(io_failure("set nonblocking speech output pipe", io::Error::last_os_error()));
    }
    Ok(())
}

#[cfg(target_os = "linux")]
struct Capture<R> {
    reader: R,
    writer: Box<dyn Write>,
    bytes: Vec<u8>,
    eof: bool,
    name: &'static str,
}
#[cfg(target_os = "linux")]
impl<R: Read> Capture<R> {
    fn drain(&mut self, limit: usize) -> Result<bool, Failure> {
        if self.eof {
            return Ok(false);
        }
        let mut progress = false;
        let mut buffer = [0u8; 8192];
        // Fairly service both streams and the deadline even if one continuously
        // emits output. Writes never exceed the cap, including the final chunk.
        for _ in 0..8 {
            match self.reader.read(&mut buffer) {
                Ok(0) => {
                    self.eof = true;
                    self.writer.flush().map_err(|e| io_failure("flush speech output log", e))?;
                    break;
                }
                Ok(size) => {
                    progress = true;
                    let keep = size.min(limit - self.bytes.len());
                    self.writer.write_all(&buffer[..keep]).map_err(|e| io_failure("write speech output log", e))?;
                    self.bytes.extend_from_slice(&buffer[..keep]);
                    if keep != size {
                        return Err(Failure {
                            code: "speech_output_limit",
                            message: format!(
                                "speech backend {} exceeded {limit} bytes; bounded output prefix is retained",
                                self.name
                            ),
                        });
                    }
                }
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => break,
                Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(e) => return Err(io_failure("read speech backend output", e)),
            }
        }
        Ok(progress)
    }
}

#[cfg(target_os = "linux")]
fn bind_owner_lifetime(command: &mut Command) {
    unsafe extern "C" {
        fn prctl(option: std::ffi::c_int, ...) -> std::ffi::c_int;
        fn getppid() -> std::ffi::c_int;
    }
    const SET_PARENT_DEATH_SIGNAL: std::ffi::c_int = 1;
    const SIGKILL: usize = 9;
    const NO_SUCH_PROCESS: std::ffi::c_int = 3;
    let owner = std::process::id() as std::ffi::c_int;
    // This hook runs after fork and before exec for both version probes and
    // analysis. It performs only Linux process syscalls and allocation-free
    // errno construction, without locks or access to inherited Rust state.
    unsafe {
        command.pre_exec(move || {
            if prctl(SET_PARENT_DEATH_SIGNAL, SIGKILL, 0usize, 0usize, 0usize) != 0 {
                return Err(io::Error::last_os_error());
            }
            // Parent death before prctl would not generate a retroactive
            // signal. Refuse exec when already reparented; death after this
            // check is covered by the signal registered above.
            if getppid() != owner {
                return Err(io::Error::from_raw_os_error(NO_SUCH_PROCESS));
            }
            Ok(())
        });
    }
}

#[cfg(target_os = "linux")]
fn execute(
    command: &mut Command,
    stdout: Box<dyn Write>,
    stderr: Box<dyn Write>,
    timeout: Duration,
    limit: usize,
) -> Result<(ExitStatus, Vec<u8>, Vec<u8>), Failure> {
    let started = Instant::now();
    bind_owner_lifetime(command);
    let child = command
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| io_failure("spawn speech backend", e))?;
    let mut process = Process { child, reaped: false };
    let output = process.child.stdout.take().ok_or_else(|| backend_failure("missing speech stdout pipe"))?;
    let diagnostics = process.child.stderr.take().ok_or_else(|| backend_failure("missing speech stderr pipe"))?;
    nonblocking(&output)?;
    nonblocking(&diagnostics)?;
    let mut output = Capture { reader: output, writer: stdout, bytes: Vec::new(), eof: false, name: "stdout" };
    let mut diagnostics =
        Capture { reader: diagnostics, writer: stderr, bytes: Vec::new(), eof: false, name: "stderr" };
    let mut status = None;
    loop {
        if started.elapsed() >= timeout {
            return Err(Failure {
                code: "speech_timeout",
                message: format!(
                    "speech backend exceeded its {} second timeout; output logs are retained",
                    timeout.as_secs()
                ),
            });
        }
        let progress = output.drain(limit)? | diagnostics.drain(limit)?;
        if status.is_none() {
            status = process.child.try_wait().map_err(|e| io_failure("poll speech backend", e))?;
            if status.is_some() {
                process.reaped = true;
            }
        }
        if output.eof && diagnostics.eof {
            if let Some(status) = status {
                return Ok((status, output.bytes, diagnostics.bytes));
            }
        }
        if !progress {
            std::thread::sleep(Duration::from_millis(5).min(timeout.saturating_sub(started.elapsed())));
        }
    }
}

#[cfg(not(target_os = "linux"))]
fn execute(
    _command: &mut Command,
    _stdout: Box<dyn Write>,
    _stderr: Box<dyn Write>,
    _timeout: Duration,
    _limit: usize,
) -> Result<(ExitStatus, Vec<u8>, Vec<u8>), Failure> {
    Err(backend_failure("the bounded local Rhubarb adapter currently requires Linux"))
}

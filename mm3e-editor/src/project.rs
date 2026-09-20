//! Durable project storage for cooperating editor processes.
//!
//! An OS lock is held on a persistent sidecar instead of the project inode, which changes
//! during atomic replacement. Sidecars are never removed: deleting a live lockfile would let
//! another process lock a different inode. A crashed process needs no stale-PID cleanup.
//! These are the storage module's local-root checks, not a hostile-filesystem sandbox.

use crate::{protocol::Failure, storage};
use std::{
    fs::{self, File, OpenOptions, TryLockError},
    path::{Path, PathBuf},
};

const LOCK_PREFIX: &str = ".mm3e-project-lock-";

#[derive(Debug)]
pub struct ProjectStore {
    root: PathBuf,
    path: PathBuf,
    relative: String,
    _lock: File,
    poisoned: bool,
}

impl ProjectStore {
    /// Acquire a nonblocking project lock and then read existing bytes, if any.
    /// The caller must validate/deserialise bytes before installing live state.
    pub fn open(root: &Path, path: &str) -> Result<(Self, Option<Vec<u8>>), Failure> {
        let root = root.canonicalize().map_err(|error| Failure::io(error.to_string()))?;
        if !root.is_dir() {
            return Err(Failure::invalid("project root must be a directory"));
        }
        let mut target = storage::path(&root, path, false)?;
        if !target
            .extension()
            .and_then(|extension| extension.to_str())
            .is_some_and(|extension| extension.eq_ignore_ascii_case("json"))
        {
            return Err(Failure::invalid(
                "native projects require a .json extension; image output paths cannot be projects",
            ));
        }
        match fs::symlink_metadata(&target) {
            Ok(metadata) if !metadata.is_file() => return Err(Failure::invalid("project path must be a regular file")),
            Ok(_) => {
                target = target.canonicalize().map_err(|error| Failure::io(error.to_string()))?;
                if !target.starts_with(&root) {
                    return Err(Failure::invalid("project path escapes the editor root"));
                }
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(Failure::io(error.to_string())),
        }
        let relative = relative_path(&root, &target)?;
        let filename = target
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| Failure::invalid("project filename must be UTF-8"))?;
        if filename.to_ascii_lowercase().starts_with(LOCK_PREFIX) {
            return Err(Failure::invalid("project lock sidecar names are reserved"));
        }
        // Fixed-length names avoid exceeding the filesystem component limit. Hash collisions
        // conservatively serialize unrelated names; they do not admit concurrent writers.
        #[cfg(windows)]
        let filename = filename.to_lowercase();
        let lock_name = format!("{LOCK_PREFIX}{:016x}", mm3e_kit::atoms::hash(filename.as_bytes()));
        let lock_path = target.with_file_name(lock_name);
        let lock_relative = relative_path(&root, &lock_path)?;
        let lock_path = storage::path(&root, &lock_relative, false)?;
        let lock = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&lock_path)
            .map_err(|error| Failure::io(format!("cannot open project lock: {error}")))?;
        if !lock.metadata().map_err(|error| Failure::io(error.to_string()))?.is_file() {
            return Err(Failure::invalid("project lock must be a regular file"));
        }
        match lock.try_lock() {
            Ok(()) => {}
            Err(TryLockError::WouldBlock) => {
                return Err(Failure {
                    code: "project_locked",
                    message: format!("project is already open for writing: {}", target.display()),
                });
            }
            Err(TryLockError::Error(error)) => {
                return Err(Failure::io(format!("cannot acquire project lock: {error}")))
            }
        }
        // Read only after acquiring the lock. A different cooperating process may have
        // installed a complete revision between initial path resolution and acquisition.
        let bytes = match fs::symlink_metadata(&target) {
            Ok(metadata) if metadata.is_file() && !metadata.file_type().is_symlink() => {
                Some(storage::read(&root, &relative)?)
            }
            Ok(_) => return Err(Failure::invalid("project path must remain a regular non-symlink file")),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
            Err(error) => return Err(Failure::io(error.to_string())),
        };
        Ok((Self { root, path: target, relative, _lock: lock, poisoned: false }, bytes))
    }

    /// The rooted project path whose lifetime lock this store owns.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// True when installation or completion of a file write could not be confirmed.
    /// The session must close/reopen this store and reload before allowing more commands.
    pub fn is_poisoned(&self) -> bool {
        self.poisoned
    }

    /// Persist complete serialized state before the caller changes its live document.
    /// Pre-install failures leave the prior file intact. `commit_uncertain` means
    /// installation may have happened, or completion after installation failed.
    pub fn write(&mut self, bytes: &[u8]) -> Result<(), Failure> {
        if self.poisoned {
            return Err(Failure {
                code: "recovery_required",
                message: "project commit outcome is uncertain; close and reopen the project before continuing".into(),
            });
        }
        if bytes.len() as u64 > storage::MAX_DOCUMENT_BYTES {
            return Err(Failure::invalid("serialized project exceeds 64 MiB"));
        }
        match storage::write_observed(&self.root, &self.relative, bytes, true) {
            Ok(_) => Ok(()),
            Err(error) if error.visibility != storage::Visibility::NotInstalled => {
                self.poisoned = true;
                Err(Failure { code: "commit_uncertain", message: error.into_failure().message })
            }
            Err(error) => Err(error.into_failure()),
        }
    }
}

fn relative_path(root: &Path, path: &Path) -> Result<String, Failure> {
    path.strip_prefix(root)
        .ok()
        .and_then(|relative| relative.to_str())
        .map(str::to_owned)
        .ok_or_else(|| Failure::invalid("project path must remain inside its root with UTF-8 components"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        io::{BufRead, BufReader, Write},
        process::{Command, Stdio},
        sync::atomic::{AtomicU64, Ordering},
        time::Duration,
    };

    static NEXT_ROOT: AtomicU64 = AtomicU64::new(0);
    struct Root(PathBuf);
    impl Root {
        fn new() -> Self {
            let path = std::env::temp_dir().join(format!(
                "mm3e-project-{}-{}",
                std::process::id(),
                NEXT_ROOT.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir(&path).unwrap();
            Self(path.canonicalize().unwrap())
        }
    }
    impl Drop for Root {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn exclusive_lock_rejects_other_handles_and_drop_allows_existing_sidecar() {
        let root = Root::new();
        let (mut store, bytes) = ProjectStore::open(&root.0, "hero.json").unwrap();
        assert!(bytes.is_none());
        assert_eq!(ProjectStore::open(&root.0, "hero.json").unwrap_err().code, "project_locked");
        store.write(b"revision one").unwrap();
        store.write(b"revision two").unwrap();
        assert_eq!(store.path(), root.0.join("hero.json"));
        assert!(!store.is_poisoned());
        drop(store);
        let (_reopened, bytes) = ProjectStore::open(&root.0, "hero.json").unwrap();
        assert_eq!(bytes.unwrap(), b"revision two");
        assert_eq!(fs::read_dir(&root.0).unwrap().count(), 2, "project plus persistent lock sidecar");
    }

    #[test]
    fn rejected_write_preserves_previous_file_and_lock() {
        let root = Root::new();
        let (mut store, _) = ProjectStore::open(&root.0, "hero.json").unwrap();
        store.write(b"known good").unwrap();
        let oversized = vec![0; storage::MAX_DOCUMENT_BYTES as usize + 1];
        assert!(store.write(&oversized).is_err());
        assert_eq!(fs::read(store.path()).unwrap(), b"known good");
        assert!(!store.is_poisoned());
        assert_eq!(ProjectStore::open(&root.0, "hero.json").unwrap_err().code, "project_locked");
        store.write(b"next good").unwrap();
        assert_eq!(fs::read(store.path()).unwrap(), b"next good");
    }

    #[test]
    fn malformed_paths_and_reserved_sidecars_fail_without_a_project() {
        let root = Root::new();
        for path in [
            "",
            "../escape.json",
            "/absolute.json",
            ".mm3e-project-lock-anything",
            ".MM3E-PROJECT-LOCK-anything",
            "missing/hero.json",
            "image.png",
            "render.exr",
        ] {
            assert!(ProjectStore::open(&root.0, path).is_err(), "{path}");
        }
        assert_eq!(fs::read_dir(&root.0).unwrap().count(), 0);
    }

    #[cfg(unix)]
    #[test]
    fn symlink_escape_and_project_or_lock_symlinks_are_rejected() {
        use std::os::unix::fs::symlink;
        let root = Root::new();
        let outside = Root::new();
        symlink(&outside.0, root.0.join("escape")).unwrap();
        assert!(ProjectStore::open(&root.0, "escape/hero.json").is_err());
        fs::write(root.0.join("real.json"), b"preserve").unwrap();
        symlink(root.0.join("real.json"), root.0.join("alias.json")).unwrap();
        assert!(ProjectStore::open(&root.0, "alias.json").is_err());
        let lock_name = format!("{LOCK_PREFIX}{:016x}", mm3e_kit::atoms::hash(b"hero.json"));
        symlink(root.0.join("real.json"), root.0.join(lock_name)).unwrap();
        assert!(ProjectStore::open(&root.0, "hero.json").is_err());
        assert_eq!(fs::read(root.0.join("real.json")).unwrap(), b"preserve");
        assert_eq!(fs::read_dir(&outside.0).unwrap().count(), 0);
    }

    #[cfg(unix)]
    #[test]
    fn symlinked_parent_aliases_share_the_same_lock() {
        use std::os::unix::fs::symlink;
        let root = Root::new();
        fs::create_dir(root.0.join("real")).unwrap();
        symlink(root.0.join("real"), root.0.join("alias")).unwrap();
        let (_store, _) = ProjectStore::open(&root.0, "real/hero.json").unwrap();
        assert_eq!(ProjectStore::open(&root.0, "alias/hero.json").unwrap_err().code, "project_locked");
    }

    // Invoked as a separate test process by the test below; ordinary runs return immediately.
    #[test]
    fn lock_holder_process() {
        let Ok(root) = std::env::var("MM3E_PROJECT_LOCK_TEST_ROOT") else { return };
        let (_store, _) = ProjectStore::open(Path::new(&root), "hero.json").unwrap();
        println!("MM3E_LOCK_READY");
        std::io::stdout().flush().unwrap();
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
    }

    #[test]
    fn killed_process_releases_the_os_lock_without_deleting_sidecar() {
        let root = Root::new();
        let mut child = Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "project::tests::lock_holder_process", "--nocapture"])
            .env("MM3E_PROJECT_LOCK_TEST_ROOT", &root.0)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        let stdout = child.stdout.take().unwrap();
        let (sender, receiver) = std::sync::mpsc::channel();
        let reader = std::thread::spawn(move || {
            for line in BufReader::new(stdout).lines() {
                if line.is_ok_and(|line| line.contains("MM3E_LOCK_READY")) {
                    let _ = sender.send(());
                    return;
                }
            }
        });
        let ready = receiver.recv_timeout(Duration::from_secs(10));
        if ready.is_err() {
            let _ = child.kill();
            let _ = child.wait();
            panic!("lock-holder subprocess did not become ready");
        }
        let competing = ProjectStore::open(&root.0, "hero.json");
        child.kill().unwrap();
        child.wait().unwrap();
        reader.join().unwrap();
        assert_eq!(competing.unwrap_err().code, "project_locked");
        let (_reopened, bytes) = ProjectStore::open(&root.0, "hero.json").unwrap();
        assert!(bytes.is_none());
        assert_eq!(fs::read_dir(&root.0).unwrap().count(), 1, "persistent sidecar remains usable after crash");
    }
}

use crate::protocol::Failure;
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::{Component, Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

pub const MAX_DOCUMENT_BYTES: u64 = 64 * 1024 * 1024;
static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

pub fn path(root: &Path, relative: &str, read: bool) -> Result<PathBuf, Failure> {
    let rel = Path::new(relative);
    if rel.as_os_str().is_empty() || !rel.components().all(|c| matches!(c, Component::Normal(_))) {
        return Err(Failure::invalid("file paths must be relative to the editor root, without . or .."));
    }
    let joined = root.join(rel);
    if read {
        let canonical = joined.canonicalize().map_err(|e| Failure::io(e.to_string()))?;
        if !canonical.starts_with(root) {
            return Err(Failure::invalid("path escapes the editor root"));
        }
        Ok(canonical)
    } else {
        let parent = joined.parent().ok_or_else(|| Failure::invalid("missing parent directory"))?;
        let canonical = parent.canonicalize().map_err(|e| Failure::io(e.to_string()))?;
        if !canonical.starts_with(root) {
            return Err(Failure::invalid("path escapes the editor root"));
        }
        if fs::symlink_metadata(&joined).is_ok_and(|m| m.file_type().is_symlink()) {
            return Err(Failure::invalid("output path must not be a symlink"));
        }
        Ok(canonical.join(rel.file_name().ok_or_else(|| Failure::invalid("missing filename"))?))
    }
}

pub fn read(root: &Path, relative: &str) -> Result<Vec<u8>, Failure> {
    let path = path(root, relative, true)?;
    let mut data = vec![];
    File::open(path)
        .map_err(|e| Failure::io(e.to_string()))?
        .take(MAX_DOCUMENT_BYTES + 1)
        .read_to_end(&mut data)
        .map_err(|e| Failure::io(e.to_string()))?;
    if data.len() as u64 > MAX_DOCUMENT_BYTES {
        return Err(Failure::invalid("file exceeds 64 MiB"));
    }
    Ok(data)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Visibility {
    NotInstalled,
    /// The installation syscall returned an error; its external effect is not proven.
    Unknown,
    /// The installation syscall succeeded; cleanup or directory synchronization failed.
    Installed,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum WriteStage {
    Resolve,
    CreateStage,
    WriteData,
    SyncFile,
    Install,
    Cleanup,
    #[cfg(unix)]
    SyncDirectory,
}
impl WriteStage {
    fn description(self) -> &'static str {
        match self {
            Self::Resolve => "resolving the output path",
            Self::CreateStage => "creating the staging file",
            Self::WriteData => "writing the staging file",
            Self::SyncFile => "synchronizing the staging file",
            Self::Install => "installing the output",
            Self::Cleanup => "removing the owned staging link",
            #[cfg(unix)]
            Self::SyncDirectory => "synchronizing the output directory",
        }
    }
}
#[derive(Debug)]
pub(crate) struct WriteFailure {
    pub(crate) visibility: Visibility,
    pub(crate) stage: WriteStage,
    cause: Failure,
}
impl WriteFailure {
    fn before(cause: Failure) -> Self {
        Self { visibility: Visibility::NotInstalled, stage: WriteStage::Resolve, cause }
    }
    pub(crate) fn into_failure(self) -> Failure {
        match self.visibility {
            Visibility::NotInstalled => self.cause,
            Visibility::Unknown => Failure {
                code: "output_state_uncertain",
                message: format!(
                    "output installation result is uncertain after {}: {}; inspect the destination before retrying",
                    self.stage.description(),
                    self.cause.message
                ),
            },
            Visibility::Installed => Failure {
                code: "output_installed_uncertain",
                message: format!(
                    "file installed, but completion is uncertain after {}: {}; inspect the destination before retrying",
                    self.stage.description(),
                    self.cause.message
                ),
            },
        }
    }
}

/// Stage and synchronize before installation. Callers receive explicit uncertainty
/// if installation failed ambiguously or succeeded before a later operation failed.
pub fn write(root: &Path, relative: &str, bytes: &[u8], overwrite: bool) -> Result<PathBuf, Failure> {
    write_observed(root, relative, bytes, overwrite).map_err(WriteFailure::into_failure)
}

/// Typed installation state is authoritative; human-readable error text is not a
/// commit protocol. In particular, an installation error is not proof of rollback.
pub(crate) fn write_observed(
    root: &Path,
    relative: &str,
    bytes: &[u8],
    overwrite: bool,
) -> Result<PathBuf, WriteFailure> {
    let target = path(root, relative, false).map_err(WriteFailure::before)?;
    if !overwrite && target.exists() {
        return Err(WriteFailure::before(Failure::invalid("output exists; set overwrite to true to replace it")));
    }
    let sequence = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let temp = target.with_file_name(format!(".mm3e-{}-{sequence}.tmp", std::process::id()));
    let mut created_temp = false;
    let mut visibility = Visibility::NotInstalled;
    let mut stage = WriteStage::CreateStage;
    let result = (|| -> Result<(), Failure> {
        let mut file =
            OpenOptions::new().write(true).create_new(true).open(&temp).map_err(|e| Failure::io(e.to_string()))?;
        created_temp = true;
        stage = WriteStage::WriteData;
        file.write_all(bytes).map_err(|e| Failure::io(e.to_string()))?;
        stage = WriteStage::SyncFile;
        file.sync_all().map_err(|e| Failure::io(e.to_string()))?;
        drop(file);
        stage = WriteStage::Install;
        visibility = Visibility::Unknown;
        if overwrite {
            fs::rename(&temp, &target).map_err(|e| Failure::io(e.to_string()))?;
            visibility = Visibility::Installed;
        } else {
            // Atomic no-clobber installation on the same filesystem, including competing writers.
            fs::hard_link(&temp, &target).map_err(|e| Failure::io(e.to_string()))?;
            visibility = Visibility::Installed;
            stage = WriteStage::Cleanup;
            fs::remove_file(&temp).map_err(|e| Failure::io(e.to_string()))?;
        }
        #[cfg(unix)]
        {
            stage = WriteStage::SyncDirectory;
            File::open(target.parent().expect("resolved output has parent"))
                .and_then(|f| f.sync_all())
                .map_err(|e| Failure::io(e.to_string()))?;
        }
        Ok(())
    })();
    if result.is_err() && created_temp {
        let _ = fs::remove_file(temp);
    }
    result.map_err(|cause| WriteFailure { visibility, stage, cause })?;
    Ok(target)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn visibility_classification_is_independent_of_message_wording() {
        let cause =
            || Failure::io("file installed, but directory durability is uncertain: deliberately misleading text");
        let error = WriteFailure { visibility: Visibility::NotInstalled, stage: WriteStage::WriteData, cause: cause() }
            .into_failure();
        assert_eq!(error.code, "io");
        for (visibility, code) in
            [(Visibility::Unknown, "output_state_uncertain"), (Visibility::Installed, "output_installed_uncertain")]
        {
            let error = WriteFailure { visibility, stage: WriteStage::Install, cause: Failure::io("plain I/O error") }
                .into_failure();
            assert_eq!(error.code, code);
            assert!(error.message.contains("inspect the destination"));
        }
    }
}

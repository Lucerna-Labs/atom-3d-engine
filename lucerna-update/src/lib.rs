//! Framework-neutral, opt-in application updates.
//!
//! A local application manifest points to a release feed. The feed selects a platform asset and
//! provides its SHA-256 digest. Downloads are staged outside the install directory, verified, and
//! only applied after the user explicitly accepts the update and the running process exits.

use semver::Version;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

const SCHEMA: u32 = 1;
const MAX_FEED_BYTES: usize = 1024 * 1024;
const MAX_ASSET_BYTES: u64 = 4 * 1024 * 1024 * 1024;
const MAX_EXTRACTED_BYTES: u64 = 8 * 1024 * 1024 * 1024;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppManifest {
    pub schema: u32,
    pub app_id: String,
    pub app_name: String,
    pub current_version: String,
    pub feed_url: String,
    pub platform: String,
    #[serde(default)]
    pub check_on_startup: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReleaseManifest {
    pub schema: u32,
    pub version: String,
    #[serde(default)]
    pub notes_url: Option<String>,
    pub assets: BTreeMap<String, ReleaseAsset>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReleaseAsset {
    pub url: String,
    pub sha256: String,
    pub executable: String,
}

#[derive(Clone, Debug)]
pub struct AvailableUpdate {
    pub app: AppManifest,
    pub release: ReleaseManifest,
    pub asset: ReleaseAsset,
}

#[derive(Clone, Debug)]
pub enum UpdateStatus {
    UpToDate,
    Available(Box<AvailableUpdate>),
}

#[derive(Clone, Debug)]
pub struct StagedUpdate {
    pub version: String,
    pub payload_dir: PathBuf,
    pub executable: String,
}

pub fn parse_app_manifest(json: &str) -> Result<AppManifest, String> {
    let manifest: AppManifest =
        serde_json::from_str(json).map_err(|error| format!("invalid app update manifest: {error}"))?;
    validate_app_manifest(&manifest)?;
    Ok(manifest)
}

pub fn check_from_json(json: &str) -> Result<UpdateStatus, String> {
    check(&parse_app_manifest(json)?)
}

pub fn check(app: &AppManifest) -> Result<UpdateStatus, String> {
    validate_app_manifest(app)?;
    let bytes = fetch_bytes(&app.feed_url, MAX_FEED_BYTES as u64)?;
    let release: ReleaseManifest =
        serde_json::from_slice(&bytes).map_err(|error| format!("invalid release feed: {error}"))?;
    validate_release_manifest(&release)?;

    let current = parse_version(&app.current_version, "current_version")?;
    let available = parse_version(&release.version, "release version")?;
    if available <= current {
        return Ok(UpdateStatus::UpToDate);
    }
    let asset = release
        .assets
        .get(&app.platform)
        .cloned()
        .ok_or_else(|| format!("release {} has no '{}' asset", release.version, app.platform))?;
    validate_asset(&asset)?;
    Ok(UpdateStatus::Available(Box::new(AvailableUpdate { app: app.clone(), release, asset })))
}

pub fn stage(update: &AvailableUpdate) -> Result<StagedUpdate, String> {
    validate_asset(&update.asset)?;
    let root = update_root()?.join(safe_component(&update.app.app_id)?).join(safe_component(&update.release.version)?);
    let payload_dir = root.join("payload");
    if root.exists() {
        fs::remove_dir_all(&root).map_err(|error| format!("could not clear old update stage: {error}"))?;
    }
    fs::create_dir_all(&payload_dir).map_err(|error| format!("could not create update stage: {error}"))?;

    let archive = root.join("update.zip");
    download_file(&update.asset.url, &archive, MAX_ASSET_BYTES)?;
    let actual = sha256_file(&archive)?;
    if !actual.eq_ignore_ascii_case(&update.asset.sha256) {
        let _ = fs::remove_file(&archive);
        return Err(format!("update checksum mismatch: expected {}, got {actual}", update.asset.sha256));
    }
    extract_zip(&archive, &payload_dir)?;
    if !payload_dir.join(&update.asset.executable).is_file() {
        return Err(format!("verified update does not contain '{}'", update.asset.executable));
    }
    Ok(StagedUpdate {
        version: update.release.version.clone(),
        payload_dir,
        executable: update.asset.executable.clone(),
    })
}

#[cfg(windows)]
pub fn schedule_install(staged: &StagedUpdate, install_dir: &Path, running_pid: u32) -> Result<(), String> {
    use std::os::windows::process::CommandExt;

    let install_dir = install_dir.canonicalize().map_err(|error| format!("invalid install directory: {error}"))?;
    let executable = install_dir.join(&staged.executable);
    let script = staged.payload_dir.parent().unwrap_or(&staged.payload_dir).join("apply-update.ps1");
    let contents = format!(
        "$ErrorActionPreference = 'Stop'\n\
         $process = Get-Process -Id {running_pid} -ErrorAction SilentlyContinue\n\
         if ($process) {{ Wait-Process -Id {running_pid} }}\n\
         $payload = '{}'\n\
         $install = '{}'\n\
         Get-ChildItem -LiteralPath $payload | Copy-Item -Destination $install -Recurse -Force\n\
         Set-Content -LiteralPath (Join-Path $install '.lucerna-update-complete') -Value '{}'\n\
         Start-Process -FilePath '{}' -WorkingDirectory $install\n\
         Remove-Item -LiteralPath $PSCommandPath -Force\n",
        ps_quote(&staged.payload_dir),
        ps_quote(&install_dir),
        staged.version.replace('\'', "''"),
        ps_quote(&executable),
    );
    fs::write(&script, contents).map_err(|error| format!("could not write update apply script: {error}"))?;
    std::process::Command::new("powershell.exe")
        .args(["-NoProfile", "-NonInteractive", "-ExecutionPolicy", "Bypass", "-File"])
        .arg(&script)
        .creation_flags(0x0800_0000)
        .spawn()
        .map_err(|error| format!("could not launch update installer: {error}"))?;
    Ok(())
}

#[cfg(not(windows))]
pub fn schedule_install(_staged: &StagedUpdate, _install_dir: &Path, _running_pid: u32) -> Result<(), String> {
    Err("automatic replacement is currently implemented for Windows packages only".into())
}

fn validate_app_manifest(manifest: &AppManifest) -> Result<(), String> {
    if manifest.schema != SCHEMA {
        return Err(format!("unsupported app update schema {}", manifest.schema));
    }
    safe_component(&manifest.app_id)?;
    parse_version(&manifest.current_version, "current_version")?;
    if manifest.app_name.trim().is_empty() || manifest.feed_url.trim().is_empty() || manifest.platform.trim().is_empty()
    {
        return Err("app_name, feed_url, and platform must not be empty".into());
    }
    Ok(())
}

fn validate_release_manifest(release: &ReleaseManifest) -> Result<(), String> {
    if release.schema != SCHEMA {
        return Err(format!("unsupported release feed schema {}", release.schema));
    }
    parse_version(&release.version, "release version")?;
    if release.assets.is_empty() {
        return Err("release feed has no assets".into());
    }
    Ok(())
}

fn validate_asset(asset: &ReleaseAsset) -> Result<(), String> {
    if asset.url.trim().is_empty() || asset.executable.trim().is_empty() {
        return Err("release asset URL and executable must not be empty".into());
    }
    safe_relative_path(&asset.executable)?;
    if asset.sha256.len() != 64 || !asset.sha256.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err("release asset SHA-256 must be exactly 64 hexadecimal characters".into());
    }
    Ok(())
}

fn parse_version(value: &str, field: &str) -> Result<Version, String> {
    Version::parse(value.trim_start_matches('v')).map_err(|error| format!("invalid {field} '{value}': {error}"))
}

fn safe_component(value: &str) -> Result<String, String> {
    let trimmed = value.trim();
    if trimmed.is_empty() || !trimmed.chars().all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '.' | '-' | '_')) {
        return Err(format!("unsafe update path component '{value}'"));
    }
    Ok(trimmed.to_owned())
}

fn safe_relative_path(value: &str) -> Result<(), String> {
    let path = Path::new(value);
    if path.is_absolute() || path.components().any(|part| !matches!(part, std::path::Component::Normal(_))) {
        return Err(format!("unsafe executable path '{value}'"));
    }
    Ok(())
}

fn update_root() -> Result<PathBuf, String> {
    let base = std::env::var_os("LOCALAPPDATA").map(PathBuf::from).unwrap_or_else(std::env::temp_dir);
    Ok(base.join("LucernaLabs").join("Updates"))
}

fn fetch_bytes(url: &str, max_bytes: u64) -> Result<Vec<u8>, String> {
    if let Some(path) = url.strip_prefix("file://") {
        let bytes = fs::read(path).map_err(|error| format!("could not read update feed: {error}"))?;
        if bytes.len() as u64 > max_bytes {
            return Err("update feed exceeds the size limit".into());
        }
        return Ok(bytes);
    }
    let response = ureq::get(url)
        .header("User-Agent", "Lucerna-Update/1")
        .call()
        .map_err(|error| format!("could not fetch update feed: {error}"))?;
    response
        .into_body()
        .with_config()
        .limit(max_bytes)
        .read_to_vec()
        .map_err(|error| format!("could not read update feed: {error}"))
}

fn download_file(url: &str, destination: &Path, max_bytes: u64) -> Result<(), String> {
    if let Some(path) = url.strip_prefix("file://") {
        let metadata = fs::metadata(path).map_err(|error| format!("could not inspect update asset: {error}"))?;
        if metadata.len() > max_bytes {
            return Err("update asset exceeds the size limit".into());
        }
        fs::copy(path, destination).map_err(|error| format!("could not copy update asset: {error}"))?;
        return Ok(());
    }
    let mut response = ureq::get(url)
        .header("User-Agent", "Lucerna-Update/1")
        .call()
        .map_err(|error| format!("could not download update asset: {error}"))?;
    let mut source = response.body_mut().as_reader().take(max_bytes + 1);
    let mut output = File::create(destination).map_err(|error| format!("could not create update archive: {error}"))?;
    let copied = io::copy(&mut source, &mut output).map_err(|error| format!("could not save update asset: {error}"))?;
    output.flush().map_err(|error| format!("could not flush update asset: {error}"))?;
    if copied > max_bytes {
        let _ = fs::remove_file(destination);
        return Err("update asset exceeds the size limit".into());
    }
    Ok(())
}

fn sha256_file(path: &Path) -> Result<String, String> {
    let mut file = File::open(path).map_err(|error| format!("could not open update archive: {error}"))?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 64 * 1024];
    loop {
        let read = file.read(&mut buffer).map_err(|error| format!("could not hash update archive: {error}"))?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

fn extract_zip(archive: &Path, destination: &Path) -> Result<(), String> {
    let file = File::open(archive).map_err(|error| format!("could not open update ZIP: {error}"))?;
    let mut zip = zip::ZipArchive::new(file).map_err(|error| format!("invalid update ZIP: {error}"))?;
    let mut extracted_bytes = 0u64;
    for index in 0..zip.len() {
        let mut entry = zip.by_index(index).map_err(|error| format!("could not read update ZIP entry: {error}"))?;
        let relative = entry.enclosed_name().ok_or_else(|| format!("unsafe path in update ZIP: {}", entry.name()))?;
        if entry.unix_mode().is_some_and(|mode| mode & 0o170000 == 0o120000) {
            return Err(format!("symbolic links are not allowed in update ZIPs: {}", entry.name()));
        }
        extracted_bytes = extracted_bytes
            .checked_add(entry.size())
            .filter(|total| *total <= MAX_EXTRACTED_BYTES)
            .ok_or_else(|| "update ZIP exceeds the extracted-size limit".to_string())?;
        let output = destination.join(relative);
        if entry.is_dir() {
            fs::create_dir_all(&output).map_err(|error| format!("could not create update directory: {error}"))?;
            continue;
        }
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent).map_err(|error| format!("could not create update directory: {error}"))?;
        }
        let mut target = File::create(&output).map_err(|error| format!("could not create staged file: {error}"))?;
        io::copy(&mut entry, &mut target).map_err(|error| format!("could not extract staged file: {error}"))?;
    }
    Ok(())
}

#[cfg(windows)]
fn ps_quote(path: &Path) -> String {
    let raw = path.to_string_lossy();
    let powershell_path = raw
        .strip_prefix(r"\\?\UNC\")
        .map(|rest| format!(r"\\{rest}"))
        .or_else(|| raw.strip_prefix(r"\\?\").map(str::to_owned))
        .unwrap_or_else(|| raw.into_owned());
    powershell_path.replace('\'', "''")
}

#[cfg(test)]
mod tests {
    use super::*;
    use zip::write::SimpleFileOptions;

    #[test]
    fn rejects_downgrade_and_selects_new_platform_asset() {
        let dir = std::env::temp_dir().join(format!("lucerna-update-test-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let feed = dir.join("release.json");
        fs::write(
            &feed,
            r#"{"schema":1,"version":"1.2.0","assets":{"windows-x86_64":{"url":"file://unused.zip","sha256":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","executable":"App.exe"}}}"#,
        )
        .unwrap();
        let app = AppManifest {
            schema: 1,
            app_id: "com.lucerna.test".into(),
            app_name: "Test".into(),
            current_version: "1.1.0".into(),
            feed_url: format!("file://{}", feed.display()),
            platform: "windows-x86_64".into(),
            check_on_startup: true,
        };
        assert!(matches!(check(&app).unwrap(), UpdateStatus::Available(_)));
        let mut current = app;
        current.current_version = "1.2.0".into();
        assert!(matches!(check(&current).unwrap(), UpdateStatus::UpToDate));
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn stages_a_verified_release_archive() {
        let id = format!("com.lucerna.stage-test-{}", std::process::id());
        let fixture = std::env::temp_dir().join(format!("lucerna-update-stage-fixture-{}", std::process::id()));
        fs::create_dir_all(&fixture).unwrap();
        let archive = fixture.join("release.zip");
        let mut zip = zip::ZipWriter::new(File::create(&archive).unwrap());
        zip.start_file("TestApp.exe", SimpleFileOptions::default()).unwrap();
        zip.write_all(b"verified fixture executable").unwrap();
        zip.finish().unwrap();

        let update = AvailableUpdate {
            app: AppManifest {
                schema: 1,
                app_id: id.clone(),
                app_name: "Stage Test".into(),
                current_version: "1.0.0".into(),
                feed_url: "file://unused".into(),
                platform: "windows-x86_64".into(),
                check_on_startup: false,
            },
            release: ReleaseManifest { schema: 1, version: "1.1.0".into(), notes_url: None, assets: BTreeMap::new() },
            asset: ReleaseAsset {
                url: format!("file://{}", archive.display()),
                sha256: sha256_file(&archive).unwrap(),
                executable: "TestApp.exe".into(),
            },
        };

        let staged = stage(&update).unwrap();
        assert_eq!(fs::read(staged.payload_dir.join("TestApp.exe")).unwrap(), b"verified fixture executable");
        let _ = fs::remove_dir_all(update_root().unwrap().join(id));
        let _ = fs::remove_dir_all(fixture);
    }

    #[cfg(windows)]
    #[test]
    fn applies_only_after_the_running_process_exits() {
        let fixture = std::env::temp_dir().join(format!("lucerna-update-apply-fixture-{}", std::process::id()));
        let payload = fixture.join("stage").join("payload");
        let install = fixture.join("install");
        fs::create_dir_all(&payload).unwrap();
        fs::create_dir_all(&install).unwrap();
        let system_executable = PathBuf::from(std::env::var_os("WINDIR").unwrap()).join("System32").join("where.exe");
        fs::copy(&system_executable, payload.join("TestApp.exe")).unwrap();
        let staged = StagedUpdate { version: "1.1.0".into(), payload_dir: payload, executable: "TestApp.exe".into() };
        let mut blocker = std::process::Command::new("powershell.exe")
            .args(["-NoProfile", "-Command", "Start-Sleep -Milliseconds 500"])
            .spawn()
            .unwrap();

        schedule_install(&staged, &install, blocker.id()).unwrap();
        blocker.wait().unwrap();
        let marker = install.join(".lucerna-update-complete");
        for _ in 0..50 {
            if marker.is_file() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        assert_eq!(fs::read_to_string(marker).unwrap().trim(), "1.1.0");
        assert_eq!(
            fs::metadata(install.join("TestApp.exe")).unwrap().len(),
            fs::metadata(system_executable).unwrap().len()
        );
        std::thread::sleep(std::time::Duration::from_millis(250));
        let _ = fs::remove_dir_all(fixture);
    }
}

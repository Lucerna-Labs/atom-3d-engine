use lucerna_release_client::{check, parse_app_manifest, schedule_install, stage, UpdateStatus};
use std::io::{self, Write};
use std::path::PathBuf;

fn main() {
    if let Err(error) = run() {
        eprintln!("Update failed: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let mut args = std::env::args_os().skip(1);
    let command = args.next().and_then(|value| value.into_string().ok()).unwrap_or_else(|| "help".into());
    let manifest_path = args.next().map(PathBuf::from);
    if command == "help" || manifest_path.is_none() {
        println!("Usage: lucerna-release-client <check|install> <lucerna-update.json> [install-directory]");
        return Ok(());
    }
    let manifest_path = manifest_path.unwrap();
    let manifest_text =
        std::fs::read_to_string(&manifest_path).map_err(|error| format!("could not read manifest: {error}"))?;
    let manifest = parse_app_manifest(&manifest_text)?;
    match check(&manifest)? {
        UpdateStatus::UpToDate => println!("{} {} is current.", manifest.app_name, manifest.current_version),
        UpdateStatus::Available(update) if command == "check" => {
            println!(
                "{} {} is available (installed {}).",
                manifest.app_name, update.release.version, manifest.current_version
            );
        }
        UpdateStatus::Available(update) if command == "install" => {
            print!("Install {} {} now? [y/N] ", manifest.app_name, update.release.version);
            io::stdout().flush().map_err(|error| error.to_string())?;
            let mut answer = String::new();
            io::stdin().read_line(&mut answer).map_err(|error| error.to_string())?;
            if !matches!(answer.trim().to_ascii_lowercase().as_str(), "y" | "yes") {
                println!("Update skipped.");
                return Ok(());
            }
            let staged = stage(&update)?;
            let install_dir = args
                .next()
                .map(PathBuf::from)
                .or_else(|| std::env::current_exe().ok().and_then(|path| path.parent().map(PathBuf::from)))
                .ok_or_else(|| "could not determine install directory".to_string())?;
            schedule_install(&staged, &install_dir, std::process::id())?;
            println!("Verified update staged. It will install after this process exits.");
        }
        UpdateStatus::Available(_) => return Err(format!("unknown command '{command}'")),
    }
    Ok(())
}

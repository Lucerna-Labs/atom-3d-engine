use mm3e_editor::{
    protocol::{Failure, Request, Response},
    Editor,
};
use std::{
    io::{self, BufRead, Read, Write},
    path::PathBuf,
};

const MAX_REQUEST: usize = 4 * 1024 * 1024;

fn run() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    let mut root = std::env::current_dir().map_err(|e| e.to_string())?;
    let mut project = None;
    let mut speech_backend = None;
    let mut no_speech_backend = false;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--root" => root = PathBuf::from(args.next().ok_or("--root requires a directory")?),
            "--no-speech-backend" => no_speech_backend = true,
            "--speech-backend" => {
                speech_backend = Some(PathBuf::from(args.next().ok_or("--speech-backend requires an executable path")?))
            }
            "--project" => project = Some(args.next().ok_or("--project requires a relative native project path")?),
            "--help" | "-h" => {
                println!("mm3e-editor [--root DIRECTORY] [--project FILE] [--speech-backend EXECUTABLE | --no-speech-backend]\nOne JSON request per stdin line; one JSON response per stdout line.\nSend {{\"id\":\"hello\",\"command\":{{\"op\":\"describe\"}}}} for the complete schema.\nThe root directory must already exist. File paths are relative to this root.\n--project holds an OS writer lock and durably saves every mutation; omit it for a transient session.");
                return Ok(());
            }
            _ => return Err(format!("unknown argument {arg}")),
        }
    }
    if no_speech_backend && speech_backend.is_some() {
        return Err("cannot combine --speech-backend with --no-speech-backend".into());
    }
    let mut editor = match project {
        Some(path) => Editor::open_project(&root, &path),
        None => Editor::new(&root),
    }
    .map_err(|e| e.message)?;
    if let Some(path) = speech_backend {
        editor.set_speech_backend(&path).map_err(|e| e.message)?;
    } else if !no_speech_backend {
        #[cfg(target_os = "linux")]
        if let Some(path) = std::env::current_exe()
            .ok()
            .and_then(|exe| exe.parent().and_then(|p| p.parent()).map(|p| p.join("tools/rhubarb/rhubarb")))
            .filter(|p| p.is_file())
        {
            editor.try_optional_speech_backend(&path);
        }
    }
    let stdin = io::stdin();
    let mut input = stdin.lock();
    let stdout = io::stdout();
    let mut output = stdout.lock();
    loop {
        let mut bytes = vec![];
        let length =
            input.by_ref().take((MAX_REQUEST + 1) as u64).read_until(b'\n', &mut bytes).map_err(|e| e.to_string())?;
        if length == 0 {
            break;
        }
        let response = if length > MAX_REQUEST {
            // Drain just this oversized request so the next request remains synchronized.
            if bytes.last() != Some(&b'\n') {
                input.skip_until(b'\n').map_err(|e| e.to_string())?;
            }
            Response {
                id: None,
                ok: false,
                revision: editor.revision(),
                result: None,
                error: Some(Failure::invalid("request exceeds 4 MiB")),
            }
        } else if bytes.iter().all(u8::is_ascii_whitespace) {
            continue;
        } else {
            match serde_json::from_slice::<Request>(&bytes) {
                Ok(request) => editor.handle(request),
                Err(e) => Response {
                    id: None,
                    ok: false,
                    revision: editor.revision(),
                    result: None,
                    error: Some(Failure { code: "invalid_request", message: e.to_string() }),
                },
            }
        };
        serde_json::to_writer(&mut output, &response).map_err(|e| e.to_string())?;
        output.write_all(b"\n").and_then(|_| output.flush()).map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("mm3e-editor: {error}");
        std::process::exit(1);
    }
}

use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tauri::{AppHandle, Emitter, Manager, State};

use crate::processor::{self, ProcessorOptions, ProcessorSummary, ProgressEvent};
use crate::processor::sidecar::SidecarPaths;

/// Shared between `start_processing` (which clones the Arc and hands it to
/// the processor) and `cancel_processing` (which flips it to true).
#[derive(Default)]
pub struct AppState {
    pub cancel_flag: Arc<AtomicBool>,
}

#[derive(Debug, serde::Deserialize)]
pub struct StartRequest {
    /// One or more ZIP archives or unzipped folders to merge into a single
    /// processing pass.
    pub input_paths: Vec<String>,
    pub output_path: String,
    pub overlay_photos: bool,
    pub overlay_videos: bool,
}

/// Open a local folder in the OS file manager (Finder on macOS).
#[tauri::command]
pub fn open_folder(path: String) -> Result<(), String> {
    let mut cmd = {
        #[cfg(target_os = "macos")]
        { std::process::Command::new("open") }
        #[cfg(target_os = "windows")]
        { std::process::Command::new("explorer") }
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        { std::process::Command::new("xdg-open") }
    };
    cmd.arg(&path)
        .spawn()
        .map(|_| ())
        .map_err(|e| format!("impossible d'ouvrir le dossier : {e}"))
}

/// Start the processing pipeline. Progress events are emitted as
/// `"progress"` events on the app handle. The cancel flag stored in
/// `AppState` is reset to `false` here and an `Arc` clone is handed to
/// the processor so the user can flip it back to `true` later.
#[tauri::command]
pub async fn start_processing(
    app: AppHandle,
    state: State<'_, AppState>,
    request: StartRequest,
) -> Result<ProcessorSummary, String> {
    let sidecars = resolve_sidecars(&app).map_err(|e| e.to_string())?;

    let options = ProcessorOptions {
        input_paths: request.input_paths.iter().map(PathBuf::from).collect(),
        output_path: PathBuf::from(&request.output_path),
        overlay_photos: request.overlay_photos,
        overlay_videos: request.overlay_videos,
        ffmpeg_path: sidecars.ffmpeg.clone(),
    };

    // Reset the cancel flag so a previous cancel doesn't bleed into this run.
    state.cancel_flag.store(false, Ordering::SeqCst);
    let cancel = state.cancel_flag.clone();

    let app_clone = app.clone();
    let on_progress: processor::ProgressCallback = Arc::new(move |event: ProgressEvent| {
        let _ = app_clone.emit("progress", &event);
    });

    tauri::async_runtime::spawn_blocking(move || processor::run(options, on_progress, cancel))
        .await
        .map_err(|e| format!("spawn error: {e}"))?
        .map_err(|e| e.to_string())
}

/// Set the shared cancel flag to true. The running processor picks it up
/// on its next loop iteration and returns `ProcessorError::Cancelled`.
#[tauri::command]
pub fn cancel_processing(state: State<'_, AppState>) {
    state.cancel_flag.store(true, Ordering::SeqCst);
}

/// Resolve sidecar binary paths.
///
/// In the bundled app, externalBin sidecars land next to the main executable
/// (Contents/MacOS on macOS). resource_dir() is tried as a secondary location.
/// In development, we fall back to system PATH.
///
/// ffmpeg is the only sidecar we use now (for video QuickTime metadata
/// rewriting and optional overlay compositing). Photo EXIF is handled
/// in-process by the `little_exif` crate.
fn resolve_sidecars(app: &AppHandle) -> Result<SidecarPaths, String> {
    let triple = std::env::var("TAURI_ENV_TARGET_TRIPLE")
        .unwrap_or_else(|_| current_target_triple());

    let mut dirs: Vec<std::path::PathBuf> = Vec::new();
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            dirs.push(parent.to_path_buf());
        }
    }
    if let Ok(res) = app.path().resource_dir() {
        dirs.push(res);
    }
    for extra in &["/opt/homebrew/bin", "/usr/local/bin"] {
        dirs.push(std::path::PathBuf::from(extra));
    }

    let find_in_dirs = |name: &str| -> Option<PathBuf> {
        for dir in &dirs {
            if let Some(p) = find_binary(dir, name, &triple) {
                return Some(p);
            }
        }
        None
    };

    let ffmpeg = find_in_dirs("ffmpeg").or_else(|| which("ffmpeg"));
    Ok(SidecarPaths { ffmpeg })
}

fn find_binary(bin_dir: &std::path::Path, name: &str, triple: &str) -> Option<PathBuf> {
    // Tauri v2 strips the triple suffix in the final bundle (exiftool-aarch64-apple-darwin
    // becomes just "exiftool" in Contents/MacOS/). Check the plain name first so the
    // production bundle works, then the suffixed name for dev/manual placement.
    let ext = if cfg!(windows) { ".exe" } else { "" };
    for candidate_name in &[
        format!("{name}{ext}"),
        format!("{name}-{triple}{ext}"),
    ] {
        let candidate = bin_dir.join(candidate_name);
        if candidate.exists() { return Some(candidate); }
    }
    None
}

fn which(name: &str) -> Option<PathBuf> {
    std::env::var("PATH").ok()?.split(':').find_map(|dir| {
        let c = std::path::Path::new(dir).join(name);
        if c.exists() { Some(c) } else { None }
    })
}

fn current_target_triple() -> String {
    // Compile-time target triple as fallback.
    std::env::consts::ARCH.to_string()
        + "-"
        + std::env::consts::OS
}

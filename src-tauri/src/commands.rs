use std::path::PathBuf;
use std::sync::Arc;
use tauri::{AppHandle, Emitter, Manager};

use crate::processor::{self, ProcessorOptions, ProcessorSummary, ProgressEvent};
use crate::processor::sidecar::SidecarPaths;

#[derive(Debug, serde::Deserialize)]
pub struct StartRequest {
    pub input_path: String,
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

/// Start the processing pipeline.  Progress events are emitted as
/// `"progress"` events on the app handle.
#[tauri::command]
pub async fn start_processing(
    app: AppHandle,
    request: StartRequest,
) -> Result<ProcessorSummary, String> {
    let sidecars = resolve_sidecars(&app, request.overlay_videos)
        .map_err(|e| e.to_string())?;

    let options = ProcessorOptions {
        input_path: PathBuf::from(&request.input_path),
        output_path: PathBuf::from(&request.output_path),
        overlay_photos: request.overlay_photos,
        overlay_videos: request.overlay_videos,
        exiftool_path: sidecars.exiftool.clone(),
        ffmpeg_path: sidecars.ffmpeg.clone(),
    };

    let app_clone = app.clone();
    let on_progress: processor::ProgressCallback = Arc::new(move |event: ProgressEvent| {
        let _ = app_clone.emit("progress", &event);
    });

    tauri::async_runtime::spawn_blocking(move || processor::run(options, on_progress))
        .await
        .map_err(|e| format!("spawn error: {e}"))?
        .map_err(|e| e.to_string())
}

/// Resolve sidecar binary paths.
///
/// In the bundled app, externalBin sidecars land next to the main executable
/// (Contents/MacOS on macOS). resource_dir() is tried as a secondary location.
/// In development, we fall back to system PATH.
fn resolve_sidecars(app: &AppHandle, need_ffmpeg: bool) -> Result<SidecarPaths, String> {
    let triple = std::env::var("TAURI_ENV_TARGET_TRIPLE")
        .unwrap_or_else(|_| current_target_triple());

    // Build a list of candidate directories to search.
    let mut dirs: Vec<std::path::PathBuf> = Vec::new();

    // 1. Directory containing the current executable (externalBin destination on macOS/Windows).
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            dirs.push(parent.to_path_buf());
        }
    }

    // 2. Resource directory (alternative Tauri v2 placement, and dev fallback).
    if let Ok(res) = app.path().resource_dir() {
        dirs.push(res);
    }

    let find_in_dirs = |name: &str| -> Option<PathBuf> {
        for dir in &dirs {
            if let Some(p) = find_binary(dir, name, &triple) {
                return Some(p);
            }
        }
        None
    };

    let exiftool = find_in_dirs("exiftool")
        .or_else(|| which("exiftool"))
        .ok_or_else(|| "exiftool introuvable".to_string())?;

    let ffmpeg = if need_ffmpeg {
        find_in_dirs("ffmpeg").or_else(|| which("ffmpeg"))
    } else {
        None
    };

    Ok(SidecarPaths { exiftool, ffmpeg })
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

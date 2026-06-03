use std::path::{Path, PathBuf};
use std::process::Command;
use crate::processor::error::ProcessorError;

pub type Result<T> = std::result::Result<T, ProcessorError>;

/// Resolved paths to the external binaries.
#[derive(Debug, Clone)]
pub struct SidecarPaths {
    pub exiftool: PathBuf,
    pub ffmpeg: Option<PathBuf>,
}

impl SidecarPaths {
    /// Look up tools from the system PATH (used in tests and CLI mode).
    pub fn from_system() -> Self {
        Self {
            exiftool: which("exiftool").unwrap_or_else(|| PathBuf::from("exiftool")),
            ffmpeg: which("ffmpeg"),
        }
    }

    pub fn exiftool_available(&self) -> bool {
        self.exiftool.exists() || which(&self.exiftool).is_some()
    }

    pub fn ffmpeg_available(&self) -> bool {
        self.ffmpeg
            .as_ref()
            .map(|p| p.exists() || which(p).is_some())
            .unwrap_or(false)
    }
}

fn which(name: impl AsRef<Path>) -> Option<PathBuf> {
    let name = name.as_ref();
    if name.is_absolute() {
        return if name.exists() { Some(name.to_path_buf()) } else { None };
    }
    std::env::var("PATH").ok()?.split(':').find_map(|dir| {
        let candidate = Path::new(dir).join(name);
        if candidate.exists() { Some(candidate) } else { None }
    })
}

/// Run exiftool with the given arguments. Returns stdout.
pub fn run_exiftool(paths: &SidecarPaths, args: &[&str]) -> Result<String> {
    let output = Command::new(&paths.exiftool)
        .args(args)
        .output()
        .map_err(|e| ProcessorError::io(&paths.exiftool, e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        Err(ProcessorError::Sidecar {
            tool: "exiftool".to_string(),
            code: output.status.code().unwrap_or(-1),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        })
    }
}

/// Run exiftool, ignore "1 image files updated" style exit codes.
/// Exiftool exits 1 on warnings that are non-fatal; we treat that as ok
/// and only fail on exit ≥ 2 or missing output.
pub fn run_exiftool_lenient(paths: &SidecarPaths, args: &[&str]) -> Result<String> {
    let output = Command::new(&paths.exiftool)
        .args(args)
        .output()
        .map_err(|e| ProcessorError::io(&paths.exiftool, e))?;

    let code = output.status.code().unwrap_or(-1);
    if code <= 1 {
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        Err(ProcessorError::Sidecar {
            tool: "exiftool".to_string(),
            code,
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        })
    }
}

/// Run ffmpeg with the given arguments.
pub fn run_ffmpeg(paths: &SidecarPaths, args: &[&str]) -> Result<()> {
    let ffmpeg = paths.ffmpeg.as_ref().ok_or_else(|| {
        ProcessorError::SidecarNotFound("ffmpeg".to_string())
    })?;

    let output = Command::new(ffmpeg)
        .args(args)
        .output()
        .map_err(|e| ProcessorError::io(ffmpeg, e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(ProcessorError::Sidecar {
            tool: "ffmpeg".to_string(),
            code: output.status.code().unwrap_or(-1),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        })
    }
}

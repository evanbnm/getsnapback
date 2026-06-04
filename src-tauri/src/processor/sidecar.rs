use std::path::{Path, PathBuf};
use std::process::Command;
use crate::processor::error::ProcessorError;

pub type Result<T> = std::result::Result<T, ProcessorError>;

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

/// Build a `Command` for an external binary, with `CREATE_NO_WINDOW` on
/// Windows so the user never sees a black flash when we spawn a process.
pub fn make_command(program: &Path) -> Command {
    let cmd = Command::new(program);
    #[cfg(windows)]
    let mut cmd = cmd;
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }
    cmd
}

/// Resolved paths to the external binaries. Only ffmpeg is a sidecar now —
/// EXIF + QuickTime metadata read/write is handled in-process by Rust crates.
#[derive(Debug, Clone, Default)]
pub struct SidecarPaths {
    /// Required only when video overlay compositing is enabled OR when we
    /// rewrite QuickTime dates on a video.
    pub ffmpeg: Option<PathBuf>,
}

impl SidecarPaths {
    /// Look up ffmpeg on the system PATH (used in tests and CLI mode).
    pub fn from_system() -> Self {
        Self {
            ffmpeg: which("ffmpeg"),
        }
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

/// Run ffmpeg with the given arguments. Errors carry the stderr for diagnostics.
pub fn run_ffmpeg(paths: &SidecarPaths, args: &[&str]) -> Result<()> {
    let ffmpeg = paths.ffmpeg.as_ref().ok_or_else(|| {
        ProcessorError::SidecarNotFound("ffmpeg".to_string())
    })?;

    let output = make_command(ffmpeg)
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

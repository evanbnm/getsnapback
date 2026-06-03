use std::path::Path;

use crate::processor::error::ProcessorError;
use crate::processor::sidecar::{SidecarPaths, run_ffmpeg};

pub type Result<T> = std::result::Result<T, ProcessorError>;

/// Composite `overlay_path` (PNG) over every frame of `main_path` (video)
/// and write the result to `output_path`.
///
/// The overlay is scaled to the video's native resolution.
/// Audio tracks are copied without re-encoding.
/// Video is re-encoded to H.264 (libx264, CRF 18, preset medium).
///
/// Returns `Ok(true)` on success, `Ok(false)` on ffmpeg failure so the caller
/// can fall back to copying the main as-is.
pub fn composite(
    main_path: &Path,
    overlay_path: &Path,
    output_path: &Path,
    sidecars: &SidecarPaths,
) -> Result<bool> {
    // scale overlay to video resolution, then overlay at (0,0).
    let filter = "[1:v]scale=rw:rh[o];[0:v][o]overlay=0:0[v]";

    let result = run_ffmpeg(sidecars, &[
        "-y",
        "-loglevel", "error",
        "-i", main_path.to_str().unwrap_or(""),
        "-i", overlay_path.to_str().unwrap_or(""),
        "-filter_complex", filter,
        "-map", "[v]",
        "-map", "0:a?",         // audio optional
        "-c:v", "libx264",
        "-crf", "18",
        "-preset", "medium",
        "-c:a", "copy",
        output_path.to_str().unwrap_or(""),
    ]);

    match result {
        Ok(()) => Ok(true),
        Err(ProcessorError::Sidecar { .. }) => Ok(false),
        Err(e) => Err(e),
    }
}

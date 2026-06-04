use std::path::Path;

use crate::processor::error::ProcessorError;
use crate::processor::sidecar::{SidecarPaths, run_ffmpeg};
use crate::processor::video_encoder;

pub type Result<T> = std::result::Result<T, ProcessorError>;

/// Composite `overlay_path` (PNG) over every frame of `main_path` (video)
/// and write the result to `output_path`.
///
/// The overlay is scaled to the video's native resolution.
/// Audio tracks are copied without re-encoding.
/// Video is re-encoded using the fastest available H.264 encoder
/// (VideoToolbox on macOS, NVENC/QSV/AMF on Windows, libx264 fallback).
///
/// Returns `Ok(true)` on success, `Ok(false)` on ffmpeg failure so the caller
/// can fall back to copying the main as-is.
pub fn composite(
    main_path: &Path,
    overlay_path: &Path,
    output_path: &Path,
    sidecars: &SidecarPaths,
) -> Result<bool> {
    let ffmpeg = sidecars.ffmpeg.as_ref().ok_or_else(|| {
        ProcessorError::SidecarNotFound("ffmpeg".to_string())
    })?;
    let encoder = video_encoder::pick(ffmpeg);

    // scale overlay to video resolution, then overlay at (0,0).
    let filter = "[1:v]scale=rw:rh[o];[0:v][o]overlay=0:0[v]";

    let mut args: Vec<&str> = vec![
        "-y",
        "-loglevel", "error",
        "-i", main_path.to_str().unwrap_or(""),
        "-i", overlay_path.to_str().unwrap_or(""),
        "-filter_complex", filter,
        "-map", "[v]",
        "-map", "0:a?",
    ];
    args.extend_from_slice(encoder.args());
    args.extend_from_slice(&[
        "-c:a", "copy",
        output_path.to_str().unwrap_or(""),
    ]);

    let result = run_ffmpeg(sidecars, &args);

    match result {
        Ok(()) => Ok(true),
        Err(ProcessorError::Sidecar { .. }) => Ok(false),
        Err(e) => Err(e),
    }
}

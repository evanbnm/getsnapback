use std::path::Path;
use std::sync::Mutex;
use once_cell::sync::Lazy;

use crate::processor::sidecar::make_command;

/// Hardware-or-software H.264 encoder picked at runtime by probing ffmpeg.
///
/// `args()` returns the encoder-specific argv slice to splice into a full
/// ffmpeg command. The choice is cached per ffmpeg binary path so the probe
/// runs at most once per session (and once per binary if the user swaps it).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum H264Encoder {
    VideoToolbox, // macOS hardware
    Nvenc,        // NVIDIA hardware (Win/Linux)
    Qsv,          // Intel QuickSync (Win/Linux)
    Amf,          // AMD hardware (Win)
    Libx264,      // Software fallback
}

impl H264Encoder {
    /// ffmpeg argv fragment for this encoder, tuned for "fast but visually
    /// close to libx264 -crf 20".
    pub fn args(self) -> &'static [&'static str] {
        match self {
            H264Encoder::VideoToolbox => &[
                "-c:v", "h264_videotoolbox",
                "-q:v", "65",
                "-allow_sw", "1",
            ],
            H264Encoder::Nvenc => &[
                "-c:v", "h264_nvenc",
                "-preset", "p4",
                "-rc", "vbr",
                "-cq", "23",
            ],
            H264Encoder::Qsv => &[
                "-c:v", "h264_qsv",
                "-preset", "fast",
                "-global_quality", "23",
            ],
            H264Encoder::Amf => &[
                "-c:v", "h264_amf",
                "-quality", "balanced",
                "-rc", "cqp",
                "-qp_i", "22", "-qp_p", "24",
            ],
            H264Encoder::Libx264 => &[
                "-c:v", "libx264",
                "-preset", "fast",
                "-crf", "20",
            ],
        }
    }
}

/// Probe order per platform. The first encoder that successfully encodes a
/// 1-frame synthetic clip wins. libx264 is always the final fallback.
fn probe_order() -> &'static [H264Encoder] {
    #[cfg(target_os = "macos")]
    { &[H264Encoder::VideoToolbox, H264Encoder::Libx264] }
    #[cfg(target_os = "windows")]
    { &[H264Encoder::Nvenc, H264Encoder::Qsv, H264Encoder::Amf, H264Encoder::Libx264] }
    #[cfg(all(not(target_os = "macos"), not(target_os = "windows")))]
    { &[H264Encoder::Nvenc, H264Encoder::Qsv, H264Encoder::Libx264] }
}

static CACHE: Lazy<Mutex<Option<(std::path::PathBuf, H264Encoder)>>> =
    Lazy::new(|| Mutex::new(None));

/// Pick the best available H.264 encoder for this ffmpeg binary.
/// The result is cached for subsequent calls.
pub fn pick(ffmpeg: &Path) -> H264Encoder {
    if let Some((cached_path, enc)) = CACHE.lock().unwrap().as_ref() {
        if cached_path == ffmpeg {
            return *enc;
        }
    }

    let picked = probe_order()
        .iter()
        .copied()
        .find(|e| works(ffmpeg, *e))
        .unwrap_or(H264Encoder::Libx264);

    log::info!("Selected H.264 encoder: {:?}", picked);
    *CACHE.lock().unwrap() = Some((ffmpeg.to_path_buf(), picked));
    picked
}

/// Try to encode a tiny synthetic clip with `enc`. Returns true on exit 0.
/// Uses lavfi `color` source so no input file is needed. The probe resolution
/// must be at least ~256x256 — VideoToolbox refuses smaller inputs with an
/// opaque "Error setting bitrate property" message.
fn works(ffmpeg: &Path, enc: H264Encoder) -> bool {
    let mut cmd = make_command(ffmpeg);
    cmd.args(["-hide_banner", "-loglevel", "error",
              "-f", "lavfi", "-i", "color=black:s=320x240:d=0.5:r=10"]);
    cmd.args(enc.args());
    cmd.args(["-f", "null", "-"]);
    matches!(cmd.output(), Ok(o) if o.status.success())
}

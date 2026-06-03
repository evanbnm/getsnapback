pub mod date;
pub mod dedup;
pub mod error;
pub mod overlay_img;
pub mod overlay_vid;
pub mod scan;
pub mod sidecar;

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use serde::{Deserialize, Serialize};

pub use error::ProcessorError;
use scan::{FileKind, FileRole, SnapFile, build_overlay_pairs, scan_directory};
use sidecar::SidecarPaths;

pub type Result<T> = std::result::Result<T, ProcessorError>;

// ─── Public types ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorOptions {
    /// ZIP archive or folder containing the Snapchat export.
    pub input_path: PathBuf,
    /// Destination folder for processed files.
    pub output_path: PathBuf,
    /// Composite photo overlays (always recommended).
    pub overlay_photos: bool,
    /// Composite video overlays via ffmpeg (slow, optional).
    pub overlay_videos: bool,
    /// Absolute path to the exiftool binary.
    pub exiftool_path: PathBuf,
    /// Absolute path to the ffmpeg binary (required only if overlay_videos).
    pub ffmpeg_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressEvent {
    pub phase: u8,
    pub phase_label: String,
    pub processed: u64,
    pub total: u64,
    pub current_file: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorSummary {
    pub photos_dated: u64,
    pub videos_dated: u64,
    pub overlays_photo: u64,
    pub overlays_video: u64,
    pub dedup_content: u64,
    pub dedup_uuid: u64,
    pub errors: Vec<String>,
    pub output_path: PathBuf,
}

pub type ProgressCallback = Arc<dyn Fn(ProgressEvent) + Send + Sync>;

// ─── Entry point ─────────────────────────────────────────────────────────────

/// Run the full processing pipeline.  Emits `ProgressEvent`s via `on_progress`.
pub fn run(options: ProcessorOptions, on_progress: ProgressCallback) -> Result<ProcessorSummary> {
    let sidecars = SidecarPaths {
        exiftool: options.exiftool_path.clone(),
        ffmpeg: options.ffmpeg_path.clone(),
    };

    // ── Resolve input ─────────────────────────────────────────────────────────
    let _temp_dir; // keeps temp dir alive for the whole run
    let working_dir: PathBuf;

    if options.input_path.is_file() {
        // Assume ZIP
        let tmp = tempfile::tempdir()
            .map_err(|e| ProcessorError::io(&options.input_path, e))?;
        extract_zip(&options.input_path, tmp.path())?;
        working_dir = tmp.path().to_path_buf();
        _temp_dir = Some(tmp);
    } else if options.input_path.is_dir() {
        working_dir = options.input_path.clone();
        _temp_dir = None;
    } else {
        return Err(ProcessorError::InputNotFound(options.input_path.clone()));
    }

    // ── Output directory ──────────────────────────────────────────────────────
    let out_dir = &options.output_path;
    std::fs::create_dir_all(out_dir)
        .map_err(|e| ProcessorError::io(out_dir, e))?;

    // ── Scan ──────────────────────────────────────────────────────────────────
    let files = scan_directory(&working_dir);
    let pairs = build_overlay_pairs(&files);

    // Set of main paths that have an overlay.
    let mains_with_overlay: HashSet<PathBuf> =
        pairs.iter().map(|(m, _)| m.path.clone()).collect();

    let total_mains = files.iter().filter(|f| f.role == FileRole::Main).count() as u64;

    let mut summary = ProcessorSummary {
        photos_dated: 0,
        videos_dated: 0,
        overlays_photo: 0,
        overlays_video: 0,
        dedup_content: 0,
        dedup_uuid: 0,
        errors: Vec::new(),
        output_path: out_dir.clone(),
    };

    // ── Phase 1 — Non-overlay mains: copy + date ──────────────────────────────
    on_progress(ProgressEvent {
        phase: 1,
        phase_label: "phase_dating".to_string(),
        processed: 0,
        total: total_mains,
        current_file: None,
    });

    let standalone_mains: Vec<&SnapFile> = files
        .iter()
        .filter(|f| f.role == FileRole::Main && !mains_with_overlay.contains(&f.path))
        .collect();

    for (i, snap) in standalone_mains.iter().enumerate() {
        let fname = file_name(&snap.path);
        on_progress(ProgressEvent {
            phase: 1,
            phase_label: "phase_dating".to_string(),
            processed: i as u64,
            total: total_mains,
            current_file: Some(fname.clone()),
        });

        let dst = out_dir.join(&fname);
        if let Err(e) = copy_file(&snap.path, &dst) {
            summary.errors.push(format!("{fname}: {e}"));
            continue;
        }

        match apply_date(snap, &dst, &sidecars) {
            Ok(dated) => match snap.kind {
                FileKind::Image => {
                    if dated { summary.photos_dated += 1; }
                }
                // Count videos regardless of whether the date was already
                // present (Keep strategy) — they were checked and are ready.
                FileKind::Video => summary.videos_dated += 1,
                FileKind::Other => {}
            },
            Err(e) => summary.errors.push(format!("{fname} (date): {e}")),
        }
    }

    // ── Phase 2a — Photo overlays ─────────────────────────────────────────────
    let photo_pairs: Vec<_> = pairs
        .iter()
        .filter(|(m, _)| m.kind == FileKind::Image)
        .collect();

    on_progress(ProgressEvent {
        phase: 2,
        phase_label: "phase_overlay_photo".to_string(),
        processed: 0,
        total: photo_pairs.len() as u64,
        current_file: None,
    });

    for (i, (main, overlay)) in photo_pairs.iter().enumerate() {
        let fname = file_name(&main.path);
        on_progress(ProgressEvent {
            phase: 2,
            phase_label: "phase_overlay_photo".to_string(),
            processed: i as u64,
            total: photo_pairs.len() as u64,
            current_file: Some(fname.clone()),
        });

        if !options.overlay_photos {
            // Option disabled: just copy the main.
            if let Err(e) = copy_and_date(main, out_dir, &sidecars) {
                summary.errors.push(format!("{fname}: {e}"));
            }
            continue;
        }

        let dst = out_dir.join(&fname);
        match overlay_img::composite(&main.path, &overlay.path, &dst) {
            Ok(true) => {
                if let Err(e) = date::copy_image_dates(&main.path, &dst, &sidecars) {
                    summary.errors.push(format!("{fname} (copy dates): {e}"));
                } else {
                    summary.overlays_photo += 1;
                }
            }
            Ok(false) => unreachable!("composite no longer returns Ok(false)"),
            Err(e) => {
                summary.errors.push(format!(
                    "{fname}: overlay illisible ({}) — {e}",
                    file_name(&overlay.path)
                ));
                let _ = copy_and_date(main, out_dir, &sidecars);
            }
        }
    }

    // ── Phase 2b — Video overlays ─────────────────────────────────────────────
    let video_pairs: Vec<_> = pairs
        .iter()
        .filter(|(m, _)| m.kind == FileKind::Video)
        .collect();

    on_progress(ProgressEvent {
        phase: 3,
        phase_label: "phase_overlay_video".to_string(),
        processed: 0,
        total: video_pairs.len() as u64,
        current_file: None,
    });

    for (i, (main, overlay)) in video_pairs.iter().enumerate() {
        let fname = file_name(&main.path);
        on_progress(ProgressEvent {
            phase: 3,
            phase_label: "phase_overlay_video".to_string(),
            processed: i as u64,
            total: video_pairs.len() as u64,
            current_file: Some(fname.clone()),
        });

        if !options.overlay_videos {
            if let Err(e) = copy_and_date(main, out_dir, &sidecars) {
                summary.errors.push(format!("{fname}: {e}"));
            }
            continue;
        }

        let dst = out_dir.join(&fname);
        match overlay_vid::composite(&main.path, &overlay.path, &dst, &sidecars) {
            Ok(true) => {
                if let Err(e) = date::copy_video_dates(&main.path, &dst, &sidecars) {
                    summary.errors.push(format!("{fname} (copy dates): {e}"));
                } else {
                    summary.overlays_video += 1;
                }
            }
            Ok(false) => {
                summary.errors.push(format!(
                    "{fname}: incrustation vidéo échouée, snap conservé tel quel"
                ));
                if let Err(e) = copy_and_date(main, out_dir, &sidecars) {
                    summary.errors.push(format!("{fname}: {e}"));
                }
            }
            Err(e) => {
                summary.errors.push(format!("{fname} (overlay vidéo): {e}"));
                let _ = copy_and_date(main, out_dir, &sidecars);
            }
        }
    }

    // ── Phase 3 — Dedup content ───────────────────────────────────────────────
    on_progress(ProgressEvent {
        phase: 4,
        phase_label: "phase_dedup_content".to_string(),
        processed: 0,
        total: 0,
        current_file: None,
    });

    match dedup::find_content_duplicates(out_dir) {
        Ok(groups) => {
            for (_, paths) in &groups {
                let keep = dedup::oldest_by_filename(paths);
                for path in paths {
                    if path == keep { continue; }
                    if let Err(e) = std::fs::remove_file(path) {
                        summary.errors.push(format!("dedup delete {}: {e}", file_name(path)));
                    } else {
                        summary.dedup_content += 1;
                    }
                }
            }
        }
        Err(e) => summary.errors.push(format!("dedup content scan: {e}")),
    }

    // ── Phase 4 — Dedup UUID ──────────────────────────────────────────────────
    on_progress(ProgressEvent {
        phase: 5,
        phase_label: "phase_dedup_uuid".to_string(),
        processed: 0,
        total: 0,
        current_file: None,
    });

    let uuid_groups = dedup::find_uuid_duplicates(out_dir);
    for (_, paths) in &uuid_groups {
        // paths is already sorted oldest-first by find_uuid_duplicates.
        for path in paths.iter().skip(1) {
            if let Err(e) = std::fs::remove_file(path) {
                summary.errors.push(format!("dedup delete {}: {e}", file_name(path)));
            } else {
                summary.dedup_uuid += 1;
            }
        }
    }

    Ok(summary)
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

fn file_name(path: &Path) -> String {
    path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_string()
}

fn copy_file(src: &Path, dst: &Path) -> Result<()> {
    std::fs::copy(src, dst).map(|_| ()).map_err(|e| ProcessorError::io(dst, e))
}

/// Copy `snap` to `out_dir` and write the correct date metadata.
/// Returns `Ok(dated)` where `dated` is `true` if a date was actually written.
fn copy_and_date(snap: &SnapFile, out_dir: &Path, sidecars: &SidecarPaths) -> Result<bool> {
    let dst = out_dir.join(file_name(&snap.path));
    copy_file(&snap.path, &dst)?;
    apply_date(snap, &dst, sidecars)
}

/// Write date metadata to `dst` according to the snap's kind and date_str.
/// Returns `Ok(true)` if a date was written, `Ok(false)` if nothing was needed.
fn apply_date(snap: &SnapFile, dst: &Path, sidecars: &SidecarPaths) -> Result<bool> {
    let date_str = match &snap.date_str {
        Some(d) => d,
        None => return Ok(false),
    };

    match snap.kind {
        FileKind::Image => {
            let strategy = date::strategy_for_image(dst, date_str)?;
            date::apply_image_date(dst, &strategy, sidecars)?;
            Ok(true)
        }
        FileKind::Video => {
            let strategy = date::strategy_for_video(&snap.path, date_str, sidecars)?;
            if strategy == date::DateStrategy::Keep {
                return Ok(false);
            }
            date::apply_video_date(dst, &strategy, sidecars)?;
            Ok(true)
        }
        FileKind::Other => Ok(false),
    }
}

// ─── ZIP extraction ───────────────────────────────────────────────────────────

fn extract_zip(zip_path: &Path, dest: &Path) -> Result<()> {
    let file = std::fs::File::open(zip_path)
        .map_err(|e| ProcessorError::io(zip_path, e))?;
    let mut archive = zip::ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i)?;
        let out_path = dest.join(entry.name());

        if entry.is_dir() {
            std::fs::create_dir_all(&out_path)
                .map_err(|e| ProcessorError::io(&out_path, e))?;
        } else {
            if let Some(parent) = out_path.parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| ProcessorError::io(parent, e))?;
            }
            let mut out_file = std::fs::File::create(&out_path)
                .map_err(|e| ProcessorError::io(&out_path, e))?;
            std::io::copy(&mut entry, &mut out_file)
                .map_err(|e| ProcessorError::io(&out_path, e))?;
        }
    }
    Ok(())
}

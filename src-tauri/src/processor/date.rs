use std::path::Path;
use chrono::{DateTime, Datelike, Local, NaiveDate, NaiveDateTime, TimeZone, Utc};

use crate::processor::error::ProcessorError;
use crate::processor::sidecar::{run_ffmpeg, SidecarPaths};

pub type Result<T> = std::result::Result<T, ProcessorError>;

// ─── Public API ──────────────────────────────────────────────────────────────

/// How we will stamp the date on a given file.
#[derive(Debug, Clone, PartialEq)]
pub enum DateStrategy {
    /// Copy the real file-modification datetime into EXIF (image) or skip (video).
    UseReal,
    /// Write date from filename at 12:00:00 (local time).
    UseNoon(NaiveDate),
    /// QuickTime date already present, do nothing.
    Keep,
}

/// Determine the `DateStrategy` for a **photo** file.
///
/// If the file's modification-day equals the filename date, system time is
/// trustworthy (direct download), use it. Otherwise it's a Snapchat
/// "Mes données" export where mtime = extraction time, use noon.
pub fn strategy_for_image(path: &Path, date_str: &str) -> Result<DateStrategy> {
    let filename_date = parse_date(date_str)?;

    let mtime = path
        .metadata()
        .map_err(|e| ProcessorError::io(path, e))?
        .modified()
        .map_err(|e| ProcessorError::io(path, e))?;

    let mod_date: DateTime<Local> = mtime.into();
    let mod_day = NaiveDate::from_ymd_opt(
        mod_date.year(),
        mod_date.month(),
        mod_date.day(),
    )
    .unwrap_or(NaiveDate::MIN);

    if mod_day == filename_date {
        Ok(DateStrategy::UseReal)
    } else {
        Ok(DateStrategy::UseNoon(filename_date))
    }
}

/// Determine the `DateStrategy` for a **video** file. Snapchat usually
/// preserves QuickTime CreateDate, only write a new one when absent.
pub fn strategy_for_video(path: &Path, date_str: &str) -> Result<DateStrategy> {
    if video_has_creation_time(path) {
        Ok(DateStrategy::Keep)
    } else {
        Ok(DateStrategy::UseNoon(parse_date(date_str)?))
    }
}

/// Apply a `DateStrategy` to a **photo** file in-place. Uses `little_exif`,
/// no external process spawned.
pub fn apply_image_date(path: &Path, strategy: &DateStrategy) -> Result<()> {
    match strategy {
        DateStrategy::UseReal => {
            // Build the EXIF date from the file's mtime, in local time.
            let mtime = path
                .metadata()
                .map_err(|e| ProcessorError::io(path, e))?
                .modified()
                .map_err(|e| ProcessorError::io(path, e))?;
            let local: DateTime<Local> = mtime.into();
            let stamp = local.format("%Y:%m:%d %H:%M:%S").to_string();
            write_image_exif_dates(path, &stamp)?;
        }
        DateStrategy::UseNoon(date) => {
            let stamp = format!("{} 12:00:00", date.format("%Y:%m:%d"));
            write_image_exif_dates(path, &stamp)?;
        }
        DateStrategy::Keep => {}
    }
    Ok(())
}

/// Apply a `DateStrategy` to a **video** file in-place. Uses ffmpeg in
/// stream-copy mode (no re-encode) to rewrite the moov atom's creation_time.
pub fn apply_video_date(
    path: &Path,
    strategy: &DateStrategy,
    sidecars: &SidecarPaths,
) -> Result<()> {
    if let DateStrategy::UseNoon(date) = strategy {
        // Interpret "noon on this date" as local clock time, convert to UTC.
        // Mirrors the old exiftool `-api QuickTimeUTC=1` semantics.
        let utc_iso = local_noon_to_utc_iso(*date)?;
        write_video_creation_time(path, &utc_iso, sidecars)?;
    }
    // UseReal and Keep do nothing on videos.
    Ok(())
}

/// Copy EXIF DateTimeOriginal + CreateDate from `src` to `dst` (photos).
/// Used after overlay compositing to preserve the original capture date.
pub fn copy_image_dates(src: &Path, dst: &Path) -> Result<()> {
    let (dt_orig, create) = read_image_exif_dates(src);
    if dt_orig.is_none() && create.is_none() {
        return Ok(());
    }

    use little_exif::exif_tag::ExifTag;
    use little_exif::metadata::Metadata;

    let mut m = Metadata::new_from_path(dst).unwrap_or_else(|_| Metadata::new());
    if let Some(v) = dt_orig {
        m.set_tag(ExifTag::DateTimeOriginal(v));
    }
    if let Some(v) = create {
        m.set_tag(ExifTag::CreateDate(v));
    }
    m.write_to_file(dst).map_err(|e| crate_io_err(dst, e))?;
    Ok(())
}

/// Copy QuickTime CreateDate from `src` to `dst` (videos). Reads the moov
/// atom of src, converts to ISO 8601 UTC, and rewrites dst's moov via ffmpeg.
pub fn copy_video_dates(src: &Path, dst: &Path, sidecars: &SidecarPaths) -> Result<()> {
    let Some(secs_1904) = read_video_creation_time(src) else {
        return Ok(()); // src has no creation date to copy
    };
    let utc_iso = moov_seconds_to_utc_iso(secs_1904);
    write_video_creation_time(dst, &utc_iso, sidecars)
}

// ─── Photo EXIF helpers (little_exif) ────────────────────────────────────────

fn write_image_exif_dates(path: &Path, stamp: &str) -> Result<()> {
    use little_exif::exif_tag::ExifTag;
    use little_exif::metadata::Metadata;

    let mut m = Metadata::new_from_path(path).unwrap_or_else(|_| Metadata::new());
    m.set_tag(ExifTag::DateTimeOriginal(stamp.to_string()));
    m.set_tag(ExifTag::CreateDate(stamp.to_string()));
    m.write_to_file(path).map_err(|e| crate_io_err(path, e))?;
    Ok(())
}

fn read_image_exif_dates(path: &Path) -> (Option<String>, Option<String>) {
    use little_exif::exif_tag::ExifTag;
    use little_exif::metadata::Metadata;

    let Ok(m) = Metadata::new_from_path(path) else {
        return (None, None);
    };
    let dt_orig = m
        .get_tag(&ExifTag::DateTimeOriginal(String::new()))
        .next()
        .and_then(|t| match t {
            ExifTag::DateTimeOriginal(v) => Some(v.clone()),
            _ => None,
        });
    let create = m
        .get_tag(&ExifTag::CreateDate(String::new()))
        .next()
        .and_then(|t| match t {
            ExifTag::CreateDate(v) => Some(v.clone()),
            _ => None,
        });
    (dt_orig, create)
}

// ─── Video QuickTime helpers (mp4 + ffmpeg) ──────────────────────────────────

/// Number of seconds between QuickTime epoch (1904-01-01 UTC) and Unix epoch.
const SECS_1904_TO_1970: u64 = 2_082_844_800;

fn video_has_creation_time(path: &Path) -> bool {
    read_video_creation_time(path)
        .map(|secs| secs > SECS_1904_TO_1970) // anything from 1970 onward is real
        .unwrap_or(false)
}

/// Read the moov atom's `creation_time` (seconds since 1904-01-01 UTC).
/// Returns `None` if the file isn't a parseable MP4/MOV or has no date.
fn read_video_creation_time(path: &Path) -> Option<u64> {
    let file = std::fs::File::open(path).ok()?;
    let size = file.metadata().ok()?.len();
    let reader = std::io::BufReader::new(file);
    let mp4 = mp4::Mp4Reader::read_header(reader, size).ok()?;
    let ct = mp4.moov.mvhd.creation_time;
    if ct == 0 {
        None
    } else {
        Some(ct)
    }
}

fn moov_seconds_to_utc_iso(secs_1904: u64) -> String {
    let unix_secs = secs_1904.saturating_sub(SECS_1904_TO_1970) as i64;
    let dt = Utc.timestamp_opt(unix_secs, 0).single().unwrap_or_else(|| {
        Utc.timestamp_opt(0, 0).single().unwrap()
    });
    dt.format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

fn local_noon_to_utc_iso(date: NaiveDate) -> Result<String> {
    let naive = date
        .and_hms_opt(12, 0, 0)
        .ok_or_else(|| date_err("invalid noon"))?;
    let local = single_local(naive)?;
    Ok(local.with_timezone(&Utc).format("%Y-%m-%dT%H:%M:%SZ").to_string())
}

fn single_local(naive: NaiveDateTime) -> Result<DateTime<Local>> {
    Local
        .from_local_datetime(&naive)
        .single()
        .ok_or_else(|| date_err("ambiguous local datetime"))
}

/// Rewrite the moov creation_time of `path` to `utc_iso` via ffmpeg
/// stream-copy, then atomically rename the temp file back over `path`.
fn write_video_creation_time(
    path: &Path,
    utc_iso: &str,
    sidecars: &SidecarPaths,
) -> Result<()> {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("file");
    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("mp4");
    let pid = std::process::id();
    let tmp = parent.join(format!(".{stem}.getsnapback-tmp-{pid}.{ext}"));

    let result = run_ffmpeg(
        sidecars,
        &[
            "-y",
            "-loglevel",
            "error",
            "-i",
            path.to_str().unwrap_or(""),
            "-c",
            "copy",
            "-map_metadata",
            "0",
            "-metadata",
            &format!("creation_time={utc_iso}"),
            tmp.to_str().unwrap_or(""),
        ],
    );

    match result {
        Ok(()) => std::fs::rename(&tmp, path).map_err(|e| ProcessorError::io(path, e)),
        Err(e) => {
            let _ = std::fs::remove_file(&tmp);
            Err(e)
        }
    }
}

// ─── Generic helpers ─────────────────────────────────────────────────────────

pub fn parse_date(date_str: &str) -> Result<NaiveDate> {
    let invalid = || date_err(&format!("invalid date: {date_str}"));
    if date_str.len() != 10 {
        return Err(invalid());
    }
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d").map_err(|_| invalid())
}

fn date_err(msg: &str) -> ProcessorError {
    ProcessorError::io(
        Path::new("(date)"),
        std::io::Error::new(std::io::ErrorKind::InvalidData, msg.to_string()),
    )
}

fn crate_io_err<E: std::fmt::Display>(path: &Path, e: E) -> ProcessorError {
    ProcessorError::io(
        path,
        std::io::Error::new(std::io::ErrorKind::Other, e.to_string()),
    )
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_date() {
        let d = parse_date("2023-05-15").unwrap();
        assert_eq!(d.year(), 2023);
        assert_eq!(d.month(), 5);
        assert_eq!(d.day(), 15);
    }

    #[test]
    fn parse_invalid_date_fails() {
        assert!(parse_date("2023-5-15").is_err());
        assert!(parse_date("not-a-date").is_err());
    }

    #[test]
    fn strategy_image_noon_when_different_day() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let strategy = strategy_for_image(tmp.path(), "2019-01-01").unwrap();
        assert_eq!(
            strategy,
            DateStrategy::UseNoon(NaiveDate::from_ymd_opt(2019, 1, 1).unwrap())
        );
    }

    #[test]
    fn strategy_image_real_when_same_day() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let today = Local::now().format("%Y-%m-%d").to_string();
        let strategy = strategy_for_image(tmp.path(), &today).unwrap();
        assert_eq!(strategy, DateStrategy::UseReal);
    }

    #[test]
    fn moov_epoch_conversion() {
        // 2018-07-15 12:23:00 UTC = 1531657380 unix = 3614502180 since 1904
        let iso = moov_seconds_to_utc_iso(3_614_502_180);
        assert_eq!(iso, "2018-07-15T12:23:00Z");
    }
}

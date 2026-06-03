use std::path::Path;
use chrono::{DateTime, Local, NaiveDate, Datelike};

use crate::processor::error::ProcessorError;
use crate::processor::sidecar::{SidecarPaths, run_exiftool_lenient};

pub type Result<T> = std::result::Result<T, ProcessorError>;

/// How we will stamp the date on a given file.
#[derive(Debug, Clone, PartialEq)]
pub enum DateStrategy {
    /// Copy the real file-modification datetime into EXIF (image) or skip (video).
    UseReal,
    /// Write date from filename at 12:00:00.
    UseNoon(NaiveDate),
    /// QuickTime date is already present — do nothing.
    Keep,
}

/// Determine the `DateStrategy` for a **photo** file.
///
/// If the file's modification-date day equals the filename date, the system
/// time is trustworthy (direct download) → use it.
/// Otherwise it's a "Mes données" export where mtime = extraction time → noon.
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

/// Determine the `DateStrategy` for a **video** file.
///
/// Snapchat usually preserves QuickTime:CreateDate.  Only write it when absent.
pub fn strategy_for_video(
    path: &Path,
    date_str: &str,
    sidecars: &SidecarPaths,
) -> Result<DateStrategy> {
    let out = run_exiftool_lenient(
        sidecars,
        &["-s3", "-QuickTime:CreateDate", path.to_str().unwrap_or("")],
    )?;

    if out.trim().is_empty() {
        let date = parse_date(date_str)?;
        Ok(DateStrategy::UseNoon(date))
    } else {
        Ok(DateStrategy::Keep)
    }
}

/// Apply a `DateStrategy` to a **photo** file in-place.
pub fn apply_image_date(
    path: &Path,
    strategy: &DateStrategy,
    sidecars: &SidecarPaths,
) -> Result<()> {
    let p = path.to_str().unwrap_or("");
    match strategy {
        DateStrategy::UseReal => {
            run_exiftool_lenient(sidecars, &[
                "-q",
                "-overwrite_original",
                "-EXIF:DateTimeOriginal<FileModifyDate",
                "-EXIF:CreateDate<FileModifyDate",
                p,
            ])?;
        }
        DateStrategy::UseNoon(date) => {
            let stamp = format!("{} 12:00:00", date.format("%Y:%m:%d"));
            run_exiftool_lenient(sidecars, &[
                "-q",
                "-overwrite_original",
                &format!("-EXIF:DateTimeOriginal={stamp}"),
                &format!("-EXIF:CreateDate={stamp}"),
                p,
            ])?;
        }
        DateStrategy::Keep => {}
    }
    Ok(())
}

/// Apply a `DateStrategy` to a **video** file in-place.
pub fn apply_video_date(
    path: &Path,
    strategy: &DateStrategy,
    sidecars: &SidecarPaths,
) -> Result<()> {
    let p = path.to_str().unwrap_or("");
    match strategy {
        DateStrategy::UseNoon(date) => {
            let stamp = format!("{} 12:00:00", date.format("%Y:%m:%d"));
            run_exiftool_lenient(sidecars, &[
                "-q",
                "-overwrite_original",
                "-api", "QuickTimeUTC",
                &format!("-QuickTime:CreateDate={stamp}"),
                &format!("-QuickTime:ModifyDate={stamp}"),
                &format!("-QuickTime:TrackCreateDate={stamp}"),
                &format!("-QuickTime:MediaCreateDate={stamp}"),
                p,
            ])?;
        }
        _ => {} // UseReal / Keep → nothing to do for video
    }
    Ok(())
}

/// Copy DateTimeOriginal + CreateDate from `src` to `dst` (photo).
pub fn copy_image_dates(src: &Path, dst: &Path, sidecars: &SidecarPaths) -> Result<()> {
    run_exiftool_lenient(sidecars, &[
        "-q",
        "-overwrite_original",
        "-TagsFromFile", src.to_str().unwrap_or(""),
        "-DateTimeOriginal",
        "-CreateDate",
        dst.to_str().unwrap_or(""),
    ])?;
    Ok(())
}

/// Copy QuickTime date tags from `src` to `dst` (video).
pub fn copy_video_dates(src: &Path, dst: &Path, sidecars: &SidecarPaths) -> Result<()> {
    run_exiftool_lenient(sidecars, &[
        "-q",
        "-overwrite_original",
        "-api", "QuickTimeUTC",
        "-TagsFromFile", src.to_str().unwrap_or(""),
        "-QuickTime:CreateDate",
        "-QuickTime:ModifyDate",
        "-QuickTime:TrackCreateDate",
        "-QuickTime:MediaCreateDate",
        dst.to_str().unwrap_or(""),
    ])?;
    Ok(())
}

// ─── helpers ─────────────────────────────────────────────────────────────────

pub fn parse_date(date_str: &str) -> Result<NaiveDate> {
    // Enforce strict YYYY-MM-DD (10 chars, zero-padded) before handing to chrono.
    let invalid = || {
        ProcessorError::io(
            date_str,
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("invalid date: {date_str}"),
            ),
        )
    };
    if date_str.len() != 10 {
        return Err(invalid());
    }
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d").map_err(|_| invalid())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

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
        // Create a temp file. Its mtime will be now (today).
        // Set date_str to an old date → strategy should be UseNoon.
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let strategy = strategy_for_image(tmp.path(), "2019-01-01").unwrap();
        assert_eq!(strategy, DateStrategy::UseNoon(NaiveDate::from_ymd_opt(2019, 1, 1).unwrap()));
    }

    #[test]
    fn strategy_image_real_when_same_day() {
        // Create a temp file. Its mtime is today.
        // Set date_str to today → strategy should be UseReal.
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let strategy = strategy_for_image(tmp.path(), &today).unwrap();
        assert_eq!(strategy, DateStrategy::UseReal);
    }

    // Integration test: requires exiftool on PATH. Run with:
    //   RUST_LOG=debug cargo test -- --ignored
    #[test]
    #[ignore]
    fn apply_image_noon_writes_exif() {
        let src = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/sample.jpg");
        let tmp = tempfile::Builder::new().suffix(".jpg").tempfile().unwrap();
        std::fs::copy(&src, tmp.path()).unwrap();

        let sidecars = SidecarPaths::from_system();
        apply_image_date(
            tmp.path(),
            &DateStrategy::UseNoon(NaiveDate::from_ymd_opt(2020, 6, 15).unwrap()),
            &sidecars,
        )
        .unwrap();

        // Read back and verify
        let out = run_exiftool_lenient(&sidecars, &[
            "-s3", "-EXIF:DateTimeOriginal", tmp.path().to_str().unwrap(),
        ])
        .unwrap();
        assert!(out.trim().starts_with("2020:06:15 12:00:00"));
    }
}

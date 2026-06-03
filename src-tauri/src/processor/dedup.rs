use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::io::{BufReader, Read};
use sha2::{Sha256, Digest};

use crate::processor::error::ProcessorError;

pub type Result<T> = std::result::Result<T, ProcessorError>;

// ─── Content deduplication ───────────────────────────────────────────────────

/// SHA-256 of a file's content.
pub fn file_hash(path: &Path) -> Result<String> {
    let f = std::fs::File::open(path).map_err(|e| ProcessorError::io(path, e))?;
    let mut reader = BufReader::new(f);
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 64 * 1024];
    loop {
        let n = reader.read(&mut buf).map_err(|e| ProcessorError::io(path, e))?;
        if n == 0 { break; }
        hasher.update(&buf[..n]);
    }
    Ok(hex::encode(hasher.finalize()))
}

/// Given a list of paths that share the same hash, return the one to KEEP
/// (the one with the oldest date in its filename, i.e. the lexicographically
/// smallest filename, since dates are `YYYY-MM-DD` prefixed).
///
/// Falls back to the first path if none have a parseable date prefix.
pub fn oldest_by_filename<'a>(paths: &'a [PathBuf]) -> &'a PathBuf {
    paths
        .iter()
        .min_by_key(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .map(|s| s.to_string())
                .unwrap_or_default()
        })
        .unwrap_or(&paths[0])
}

/// Scan `dir` for duplicate content.  Returns a map `hash → [paths]` where
/// each value vec has length ≥ 2.
pub fn find_content_duplicates(dir: &Path) -> Result<HashMap<String, Vec<PathBuf>>> {
    let mut by_hash: HashMap<String, Vec<PathBuf>> = HashMap::new();

    for entry in walkdir::WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let hash = file_hash(entry.path())?;
        by_hash.entry(hash).or_default().push(entry.path().to_path_buf());
    }

    by_hash.retain(|_, v| v.len() > 1);
    Ok(by_hash)
}

// ─── UUID deduplication ──────────────────────────────────────────────────────

/// Extract the Snapchat UUID from a filename such as
/// `2023-05-15_UUID-main.jpg`.  Returns `None` if the filename doesn't match.
pub fn extract_uuid(filename: &str) -> Option<String> {
    // Strip extension
    let stem = filename.rsplit_once('.').map(|(s, _)| s).unwrap_or(filename);
    // Must start with date prefix YYYY-MM-DD_
    if stem.len() < 11 || stem.as_bytes()[10] != b'_' {
        return None;
    }
    let after_date = &stem[11..]; // "UUID-main" or "UUID-overlay"
    let uuid = if let Some(u) = after_date.strip_suffix("-main") {
        u
    } else if let Some(u) = after_date.strip_suffix("-overlay") {
        u
    } else {
        return None;
    };
    if uuid.is_empty() { None } else { Some(uuid.to_string()) }
}

/// Find all UUIDs that appear more than once in `dir` (across different dates).
/// Returns a map `uuid → sorted vec of paths` (sorted = oldest first by name).
pub fn find_uuid_duplicates(dir: &Path) -> HashMap<String, Vec<PathBuf>> {
    let mut by_uuid: HashMap<String, Vec<PathBuf>> = HashMap::new();

    for entry in walkdir::WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let fname = entry
            .file_name()
            .to_str()
            .unwrap_or("")
            .to_string();
        if let Some(uuid) = extract_uuid(&fname) {
            by_uuid.entry(uuid).or_default().push(entry.path().to_path_buf());
        }
    }

    // Sort each group by filename (= date prefix) so index 0 is the oldest.
    for paths in by_uuid.values_mut() {
        paths.sort_by_key(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .map(|s| s.to_string())
                .unwrap_or_default()
        });
    }

    by_uuid.retain(|_, v| v.len() > 1);
    by_uuid
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── extract_uuid ──────────────────────────────────────────────────────────

    #[test]
    fn uuid_from_main() {
        assert_eq!(
            extract_uuid("2023-05-15_a1b2c3-main.jpg"),
            Some("a1b2c3".to_string())
        );
    }

    #[test]
    fn uuid_from_overlay() {
        assert_eq!(
            extract_uuid("2023-05-15_a1b2c3-overlay.png"),
            Some("a1b2c3".to_string())
        );
    }

    #[test]
    fn uuid_from_full_uuid() {
        assert_eq!(
            extract_uuid("2023-05-15_a1b2c3d4-e5f6-7890-abcd-ef1234567890-main.mp4"),
            Some("a1b2c3d4-e5f6-7890-abcd-ef1234567890".to_string())
        );
    }

    #[test]
    fn uuid_none_for_random_file() {
        assert!(extract_uuid("random.jpg").is_none());
        assert!(extract_uuid("2023-5-15_abc-main.jpg").is_none()); // bad date format
    }

    // ── oldest_by_filename ────────────────────────────────────────────────────

    #[test]
    fn keeps_oldest_by_date_prefix() {
        let paths = vec![
            PathBuf::from("/out/2023-06-01_abc-main.jpg"),
            PathBuf::from("/out/2023-05-15_abc-main.jpg"), // older
            PathBuf::from("/out/2024-01-01_abc-main.jpg"),
        ];
        let kept = oldest_by_filename(&paths);
        assert_eq!(kept, &PathBuf::from("/out/2023-05-15_abc-main.jpg"));
    }

    // ── file_hash ─────────────────────────────────────────────────────────────

    #[test]
    fn identical_content_same_hash() {
        let dir = tempfile::tempdir().unwrap();
        let a = dir.path().join("a.bin");
        let b = dir.path().join("b.bin");
        std::fs::write(&a, b"hello world").unwrap();
        std::fs::write(&b, b"hello world").unwrap();
        assert_eq!(file_hash(&a).unwrap(), file_hash(&b).unwrap());
    }

    #[test]
    fn different_content_different_hash() {
        let dir = tempfile::tempdir().unwrap();
        let a = dir.path().join("a.bin");
        let b = dir.path().join("b.bin");
        std::fs::write(&a, b"hello").unwrap();
        std::fs::write(&b, b"world").unwrap();
        assert_ne!(file_hash(&a).unwrap(), file_hash(&b).unwrap());
    }

    // ── find_content_duplicates ───────────────────────────────────────────────

    #[test]
    fn detects_duplicate_files() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("2023-05-15_abc-main.jpg"), b"same").unwrap();
        std::fs::write(dir.path().join("2023-06-01_abc-main.jpg"), b"same").unwrap();
        std::fs::write(dir.path().join("2023-07-01_xyz-main.jpg"), b"different").unwrap();

        let dups = find_content_duplicates(dir.path()).unwrap();
        assert_eq!(dups.len(), 1); // one hash group with duplicates
        let (_, paths) = dups.iter().next().unwrap();
        assert_eq!(paths.len(), 2);
    }

    // ── find_uuid_duplicates ──────────────────────────────────────────────────

    #[test]
    fn detects_same_uuid_different_date() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("2023-05-15_myuuid-main.jpg"), b"a").unwrap();
        std::fs::write(dir.path().join("2023-06-01_myuuid-main.jpg"), b"b").unwrap();
        std::fs::write(dir.path().join("2023-07-01_otheruuid-main.jpg"), b"c").unwrap();

        let dups = find_uuid_duplicates(dir.path());
        assert_eq!(dups.len(), 1);
        let paths = &dups["myuuid"];
        // Oldest is first after sorting
        assert!(paths[0].to_str().unwrap().contains("2023-05-15"));
    }
}

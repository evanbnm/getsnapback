use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone, PartialEq)]
pub enum FileKind {
    Image, // jpg / jpeg / png
    Video, // mp4 / mov / m4v
    Other,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FileRole {
    Main,
    Overlay,
}

#[derive(Debug, Clone)]
pub struct SnapFile {
    pub path: PathBuf,
    /// "YYYY-MM-DD" extracted from filename, if present.
    pub date_str: Option<String>,
    /// Snapchat UUID extracted from filename, if present.
    pub uuid: Option<String>,
    pub kind: FileKind,
    pub role: FileRole,
}

impl SnapFile {
    /// Returns `None` for files that don't match the Snapchat naming convention
    /// (`YYYY-MM-DD_UUID-(main|overlay).ext`).
    pub fn from_path(path: &Path) -> Option<Self> {
        let stem = path.file_stem()?.to_str()?;
        let ext = path.extension()?.to_str()?.to_lowercase();

        let kind = match ext.as_str() {
            "jpg" | "jpeg" | "png" => FileKind::Image,
            "mp4" | "mov" | "m4v" => FileKind::Video,
            _ => FileKind::Other,
        };

        // Determine role by suffix of stem.
        let (prefix, role) = if let Some(p) = stem.strip_suffix("-main") {
            (p, FileRole::Main)
        } else if let Some(p) = stem.strip_suffix("-overlay") {
            (p, FileRole::Overlay)
        } else {
            return None;
        };

        // prefix is "YYYY-MM-DD_UUID"
        if prefix.len() < 11 {
            return None;
        }

        let date_str = &prefix[..10];
        let date_ok = date_str
            .chars()
            .enumerate()
            .all(|(i, c)| if i == 4 || i == 7 { c == '-' } else { c.is_ascii_digit() });
        if !date_ok || prefix.as_bytes().get(10) != Some(&b'_') {
            return None;
        }

        let uuid = prefix[11..].to_string();
        let date_str = date_str.to_string();

        Some(Self {
            path: path.to_path_buf(),
            date_str: Some(date_str),
            uuid: if uuid.is_empty() { None } else { Some(uuid) },
            kind,
            role,
        })
    }
}

/// Walk `dir` recursively and return all recognisable Snapchat files.
pub fn scan_directory(dir: &Path) -> Vec<SnapFile> {
    WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| SnapFile::from_path(e.path()))
        .collect()
}

/// For every Main file that has a matching Overlay, return the pair.
/// Matching = same date_str + same uuid.
pub fn build_overlay_pairs<'a>(
    files: &'a [SnapFile],
) -> Vec<(&'a SnapFile, &'a SnapFile)> {
    let overlays: Vec<&SnapFile> = files.iter().filter(|f| f.role == FileRole::Overlay).collect();
    let mut pairs = Vec::new();
    for overlay in overlays {
        if let Some(main) = files.iter().find(|f| {
            f.role == FileRole::Main
                && f.date_str == overlay.date_str
                && f.uuid == overlay.uuid
        }) {
            pairs.push((main, overlay));
        }
    }
    pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(name: &str) -> PathBuf {
        PathBuf::from(name)
    }

    #[test]
    fn parse_jpeg_main() {
        let f = SnapFile::from_path(&p("2023-05-15_a1b2c3d4-e5f6-7890-abcd-ef1234567890-main.jpg")).unwrap();
        assert_eq!(f.date_str, Some("2023-05-15".to_string()));
        assert_eq!(f.uuid, Some("a1b2c3d4-e5f6-7890-abcd-ef1234567890".to_string()));
        assert_eq!(f.role, FileRole::Main);
        assert_eq!(f.kind, FileKind::Image);
    }

    #[test]
    fn parse_png_overlay() {
        let f = SnapFile::from_path(&p("2023-05-15_abc123-overlay.png")).unwrap();
        assert_eq!(f.role, FileRole::Overlay);
        assert_eq!(f.kind, FileKind::Image);
    }

    #[test]
    fn parse_mp4_main() {
        let f = SnapFile::from_path(&p("2021-12-31_deadbeef-main.mp4")).unwrap();
        assert_eq!(f.kind, FileKind::Video);
        assert_eq!(f.role, FileRole::Main);
        assert_eq!(f.date_str, Some("2021-12-31".to_string()));
    }

    #[test]
    fn parse_mov_main() {
        let f = SnapFile::from_path(&p("2020-01-01_abc-main.mov")).unwrap();
        assert_eq!(f.kind, FileKind::Video);
    }

    #[test]
    fn reject_no_role_suffix() {
        assert!(SnapFile::from_path(&p("2023-05-15_abc.jpg")).is_none());
    }

    #[test]
    fn reject_no_date_prefix() {
        assert!(SnapFile::from_path(&p("random_file-main.jpg")).is_none());
    }

    #[test]
    fn reject_bad_date_format() {
        assert!(SnapFile::from_path(&p("2023-5-15_abc-main.jpg")).is_none());
    }

    #[test]
    fn overlay_pairs_matched_correctly() {
        let files = vec![
            SnapFile::from_path(&p("2023-05-15_abc-main.jpg")).unwrap(),
            SnapFile::from_path(&p("2023-05-15_abc-overlay.png")).unwrap(),
            SnapFile::from_path(&p("2023-05-16_xyz-main.jpg")).unwrap(), // no overlay
        ];
        let pairs = build_overlay_pairs(&files);
        assert_eq!(pairs.len(), 1);
        assert_eq!(pairs[0].0.date_str, Some("2023-05-15".to_string()));
    }

    #[test]
    fn overlay_pairs_uuid_must_match() {
        let files = vec![
            SnapFile::from_path(&p("2023-05-15_abc-main.jpg")).unwrap(),
            SnapFile::from_path(&p("2023-05-15_xyz-overlay.png")).unwrap(), // different UUID
        ];
        assert!(build_overlay_pairs(&files).is_empty());
    }
}

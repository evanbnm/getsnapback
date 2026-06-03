use std::path::Path;
use image::{DynamicImage, GenericImageView, imageops, ImageReader};

use crate::processor::error::ProcessorError;

pub type Result<T> = std::result::Result<T, ProcessorError>;

/// Open an image detecting its format from file content (magic bytes), not
/// the file extension.  Required because Snapchat exports WebP overlays with
/// a `.png` extension.
fn open_any(path: &Path) -> std::result::Result<DynamicImage, image::ImageError> {
    ImageReader::open(path)?
        .with_guessed_format()?
        .decode()
}

/// Composite `overlay_path` on top of `main_path` and save the result to
/// `output_path`.
///
/// Both images are opened by magic-byte detection so the actual file format
/// (JPEG, PNG, WebP, …) is handled correctly regardless of extension.
/// The overlay is resized to exactly the main image's dimensions before
/// compositing.
///
/// Returns `Ok(true)` on success, `Err` on any failure (caller falls back to
/// copying the main as-is).
pub fn composite(main_path: &Path, overlay_path: &Path, output_path: &Path) -> Result<bool> {
    let main = open_any(main_path).map_err(|e| ProcessorError::image(main_path, e))?;
    let (w, h) = main.dimensions();

    let overlay = open_any(overlay_path).map_err(|e| ProcessorError::image(overlay_path, e))?;

    // Resize overlay to main dimensions (exact fill).
    let overlay_resized = overlay.resize_exact(w, h, imageops::FilterType::Lanczos3);

    // Composite: alpha-blend overlay on top of main.
    let mut canvas = main.to_rgba8();
    imageops::overlay(&mut canvas, &overlay_resized.to_rgba8(), 0, 0);

    // Save output in the same format as the main image.
    let ext = main_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let result = DynamicImage::ImageRgba8(canvas);
    if ext == "png" {
        result
            .save_with_format(output_path, image::ImageFormat::Png)
            .map_err(|e| ProcessorError::image(output_path, e))?;
    } else {
        // JPEG (or anything else): strip alpha, encode at high quality.
        let rgb = result.to_rgb8();
        let mut out_file =
            std::fs::File::create(output_path).map_err(|e| ProcessorError::io(output_path, e))?;
        let mut enc = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut out_file, 95);
        enc.encode_image(&rgb)
            .map_err(|e| ProcessorError::image(output_path, e))?;
    }

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composite_png_overlay_on_jpeg() {
        use image::{RgbImage, RgbaImage};
        let dir = tempfile::tempdir().unwrap();

        let main_path = dir.path().join("main.jpg");
        RgbImage::from_pixel(4, 4, image::Rgb([255, 255, 255]))
            .save(&main_path)
            .unwrap();

        // 2×2 semi-transparent red PNG upscaled to 4×4
        let overlay_path = dir.path().join("overlay.png");
        RgbaImage::from_pixel(2, 2, image::Rgba([255, 0, 0, 128]))
            .save(&overlay_path)
            .unwrap();

        let output_path = dir.path().join("output.jpg");
        assert!(composite(&main_path, &overlay_path, &output_path).unwrap());
        assert!(output_path.exists());
        assert_eq!(open_any(&output_path).unwrap().dimensions(), (4, 4));
    }

    #[test]
    fn corrupt_overlay_returns_err() {
        let dir = tempfile::tempdir().unwrap();
        let main_path = dir.path().join("main.jpg");
        image::RgbImage::from_pixel(4, 4, image::Rgb([200, 200, 200]))
            .save(&main_path)
            .unwrap();

        // Write garbage bytes as "overlay" — should error, not panic.
        let overlay_path = dir.path().join("overlay.png");
        std::fs::write(&overlay_path, b"not an image").unwrap();

        let output_path = dir.path().join("output.jpg");
        assert!(composite(&main_path, &overlay_path, &output_path).is_err());
        assert!(!output_path.exists());
    }
}

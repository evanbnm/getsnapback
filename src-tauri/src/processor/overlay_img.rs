use std::io::BufWriter;
use std::path::Path;
use image::{DynamicImage, GenericImageView, imageops, ImageReader, RgbaImage};

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
/// Performance notes:
/// - We skip resizing entirely when the overlay already matches the main's
///   dimensions (the common case for Snapchat paired exports).
/// - When a resize is needed we use the Triangle (bilinear) filter rather
///   than Lanczos3. The visual difference for Snap overlays (text, drawings,
///   stickers) is imperceptible, and Triangle is 5-8× faster.
/// - For JPEG output we composite the overlay directly into the main's RGB
///   buffer with a manual alpha blend, avoiding a full RGBA round-trip.
pub fn composite(main_path: &Path, overlay_path: &Path, output_path: &Path) -> Result<bool> {
    let main = open_any(main_path).map_err(|e| ProcessorError::image(main_path, e))?;
    let (w, h) = main.dimensions();

    let overlay = open_any(overlay_path).map_err(|e| ProcessorError::image(overlay_path, e))?;
    let overlay_rgba: RgbaImage = if overlay.dimensions() == (w, h) {
        overlay.to_rgba8()
    } else {
        let src = overlay.to_rgba8();
        imageops::resize(&src, w, h, imageops::FilterType::Triangle)
    };

    let ext = main_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let out_file = std::fs::File::create(output_path)
        .map_err(|e| ProcessorError::io(output_path, e))?;
    let mut writer = BufWriter::new(out_file);

    if ext == "png" {
        // PNG output keeps the alpha channel.
        let mut canvas = main.to_rgba8();
        imageops::overlay(&mut canvas, &overlay_rgba, 0, 0);
        DynamicImage::ImageRgba8(canvas)
            .write_to(&mut writer, image::ImageFormat::Png)
            .map_err(|e| ProcessorError::image(output_path, e))?;
    } else {
        // JPEG (or anything else): alpha-blend directly into RGB to skip the
        // RGBA8 → RGB8 round-trip.
        let mut rgb = main.to_rgb8();
        blend_rgba_over_rgb(&mut rgb, &overlay_rgba);
        let mut enc = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut writer, 95);
        enc.encode_image(&rgb)
            .map_err(|e| ProcessorError::image(output_path, e))?;
    }

    Ok(true)
}

/// Source-over alpha blend of `over` onto `under`, in-place on `under`.
/// Both images must already share dimensions (caller guarantees it via the
/// resize step). Hot inner loop: keep allocation-free.
fn blend_rgba_over_rgb(under: &mut image::RgbImage, over: &RgbaImage) {
    debug_assert_eq!(under.dimensions(), over.dimensions());
    let pixels = (under.width() as usize) * (under.height() as usize);
    let under_raw = under.as_mut();
    let over_raw = over.as_raw();

    for i in 0..pixels {
        let a = over_raw[i * 4 + 3] as u32;
        if a == 0 {
            continue;
        }
        if a == 255 {
            under_raw[i * 3]     = over_raw[i * 4];
            under_raw[i * 3 + 1] = over_raw[i * 4 + 1];
            under_raw[i * 3 + 2] = over_raw[i * 4 + 2];
            continue;
        }
        let inv = 255 - a;
        for c in 0..3 {
            let s = over_raw[i * 4 + c] as u32;
            let d = under_raw[i * 3 + c] as u32;
            under_raw[i * 3 + c] = ((s * a + d * inv + 127) / 255) as u8;
        }
    }
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

    #[test]
    fn opaque_overlay_pixels_replace_under() {
        use image::{Rgb, Rgba};
        let mut under = image::RgbImage::from_pixel(2, 2, Rgb([10, 20, 30]));
        let over = image::RgbaImage::from_pixel(2, 2, Rgba([200, 100, 50, 255]));
        blend_rgba_over_rgb(&mut under, &over);
        for p in under.pixels() {
            assert_eq!(p.0, [200, 100, 50]);
        }
    }

    #[test]
    fn transparent_overlay_keeps_under() {
        use image::{Rgb, Rgba};
        let mut under = image::RgbImage::from_pixel(2, 2, Rgb([10, 20, 30]));
        let over = image::RgbaImage::from_pixel(2, 2, Rgba([200, 100, 50, 0]));
        blend_rgba_over_rgb(&mut under, &over);
        for p in under.pixels() {
            assert_eq!(p.0, [10, 20, 30]);
        }
    }
}

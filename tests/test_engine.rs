use upscayl::config::{ImageFormat, UpscaylOptions};
use upscayl::engine::UpscaylEngine;
use image::{ImageReader, Rgba, RgbaImage};
use tempfile::tempdir;

fn create_sample_image(path: &std::path::Path, width: u32, height: u32) {
    let mut img = RgbaImage::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let r = ((x * 255) / width) as u8;
            let g = ((y * 255) / height) as u8;
            let b = 128;
            img.put_pixel(x, y, Rgba([r, g, b, 255]));
        }
    }
    img.save(path).unwrap();
}

#[test]
fn test_upscale_standard_4x() {
    let dir = tempdir().unwrap();
    let input = dir.path().join("test_input.png");
    let output = dir.path().join("test_output.png");

    create_sample_image(&input, 64, 64);

    let options = UpscaylOptions {
        model: "upscayl-standard-4x".to_string(),
        scale: Some(4),
        force_software: true,
        ..Default::default()
    };

    let result = UpscaylEngine::upscale_file(&input, &output, &options, None).unwrap();

    assert_eq!(result.original_width, 64);
    assert_eq!(result.original_height, 64);
    assert_eq!(result.upscaled_width, 256);
    assert_eq!(result.upscaled_height, 256);
    assert!(output.exists());

    let (saved_w, saved_h) = ImageReader::open(&output).unwrap().into_dimensions().unwrap();
    assert_eq!(saved_w, 256);
    assert_eq!(saved_h, 256);
}

#[test]
fn test_upscale_custom_dimensions() {
    let dir = tempdir().unwrap();
    let input = dir.path().join("input_rect.png");
    let output = dir.path().join("output_custom.png");

    create_sample_image(&input, 100, 50);

    let options = UpscaylOptions {
        custom_width: Some(300),
        force_software: true,
        ..Default::default()
    };

    let result = UpscaylEngine::upscale_file(&input, &output, &options, None).unwrap();

    assert_eq!(result.upscaled_width, 300);
    assert_eq!(result.upscaled_height, 150); // Preserves 2:1 aspect ratio
}

#[test]
fn test_format_conversion_jpeg_webp() {
    let dir = tempdir().unwrap();
    let input = dir.path().join("input.png");
    let output_jpg = dir.path().join("output.jpg");
    let output_webp = dir.path().join("output.webp");

    create_sample_image(&input, 32, 32);

    let mut options = UpscaylOptions {
        format: ImageFormat::Jpeg,
        force_software: true,
        ..Default::default()
    };

    UpscaylEngine::upscale_file(&input, &output_jpg, &options, None).unwrap();
    assert!(output_jpg.exists());

    options.format = ImageFormat::Webp;
    UpscaylEngine::upscale_file(&input, &output_webp, &options, None).unwrap();
    assert!(output_webp.exists());
}

#[test]
fn test_double_upscale() {
    let dir = tempdir().unwrap();
    let input = dir.path().join("input_small.png");
    let output = dir.path().join("output_double.png");

    create_sample_image(&input, 16, 16);

    let options = UpscaylOptions {
        double_upscale: true,
        force_software: true,
        ..Default::default()
    };

    let result = UpscaylEngine::upscale_file(&input, &output, &options, None).unwrap();

    // 16x16 with 4x double pass -> 16 * 4 * 4 = 256x256
    assert_eq!(result.upscaled_width, 256);
    assert_eq!(result.upscaled_height, 256);
}

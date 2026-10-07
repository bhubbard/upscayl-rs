use upscayl::config::UpscaylOptions;
use upscayl::batch::BatchProcessor;
use image::{Rgba, RgbaImage};
use tempfile::tempdir;

#[test]
fn test_batch_processing_directory() {
    let in_dir = tempdir().unwrap();
    let out_dir = tempdir().unwrap();

    // Create 3 sample images
    for i in 1..=3 {
        let mut img = RgbaImage::new(30, 30);
        for y in 0..30 {
            for x in 0..30 {
                img.put_pixel(x, y, Rgba([i * 60, i * 40, 100, 255]));
            }
        }
        let p = in_dir.path().join(format!("image_{}.png", i));
        img.save(p).unwrap();
    }

    let options = UpscaylOptions {
        model: "upscayl-standard-4x".to_string(),
        scale: Some(4),
        force_software: true,
        ..Default::default()
    };

    let summary = BatchProcessor::process_directory(in_dir.path(), out_dir.path(), &options, None).unwrap();

    assert_eq!(summary.total_images, 3);
    assert_eq!(summary.succeeded, 3);
    assert_eq!(summary.failed, 0);

    let mut output_files = Vec::new();
    for entry in std::fs::read_dir(out_dir.path()).unwrap().flatten() {
        output_files.push(entry.file_name().to_string_lossy().to_string());
    }
    assert_eq!(output_files.len(), 3);
}

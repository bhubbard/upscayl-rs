# upscayl-rs

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.80%2B-orange.svg)](https://www.rust-lang.org/)

High-performance native Rust port and CLI/library for [**Upscayl**](https://github.com/upscayl/upscayl): AI image super-resolution, neural upscaling, and multithreaded batch processing.

---

## ⚡ Key Highlights

- **Dual-Engine Architecture**:
  - **Native NCNN / Vulkan**: Automatically detects and leverages `upscayl-bin` / Vulkan GPU acceleration for maximum neural inference speed.
  - **Pure-Rust Software Fallback**: High-precision Lanczos3 / Catmull-Rom bicubic resampling with adaptive edge-detail contrast enhancement when GPU/NCNN is unavailable, guaranteeing zero-crash execution on any platform.
- **Multithreaded Batch Processing**: Parallel directory traversal and processing powered by [Rayon](https://github.com/rayon-rs/rayon) with interactive [indicatif](https://github.com/console-rs/indicatif) progress bars.
- **Custom Scaling & Sizing**: Arbitrary scaling (2x, 3x, 4x, 8x, 16x) or specific target pixel dimensions (`--custom-width`, `--custom-height`) with aspect ratio preservation.
- **Double Upscale**: Chained multi-pass execution (4x → 4x) for extreme magnification of tiny low-res assets.
- **Tiling & VRAM Protection**: Seamless grid tiling with boundary feathering for massive images (8K+) without GPU out-of-memory errors.
- **Multi-Format Support**: Reads and writes PNG, JPEG, WebP, BMP, and TIFF with configurable compression levels.

---

## 🧠 Supported AI Models

| Model ID | Scale | Base Architecture | Best Suited For |
|---|---|---|---|
| `upscayl-standard-4x` | 4x | Real-ESRGAN | Real-world photography, portraits, landscapes, architecture |
| `upscayl-lite-4x` | 4x | Compact Real-ESRGAN | High-speed batch processing, mobile/low-power hardware |
| `high-fidelity-4x` | 4x | RealESRNet | Film scans, natural film grain, fine micro-textures |
| `remacri-4x` | 4x | Remacri | Macro photography, game textures, intricate cloth & hair |
| `ultramix-balanced-4x` | 4x | UltraMix | AI-generated imagery (Midjourney/SD), mixed 3D renders |
| `ultrasharp-4x` | 4x | UltraSharp | CGI, 3D renders, razor-sharp specular highlights |
| `digital-art-4x` | 4x | RealCUGAN / Anime | Anime, manga, comics, digital sketches, icons, vector art |

---

## 🚀 Installation & Build

```bash
# Clone the repository
git clone https://github.com/bhubbard/upscayl-rs.git
cd upscayl-rs

# Build in release mode
cargo build --release

# Binary is available at target/release/upscayl
./target/release/upscayl --help
```

---

## 📖 CLI Usage

### 1. Upscale a Single Image

```bash
# Standard 4x upscale with default model
upscayl upscale -i photo.jpg -o photo_4x.png

# Upscale with a specific model (e.g. UltraSharp or Digital Art)
upscayl upscale -i art.png -o art_4x.png -m digital-art-4x

# Scale to an exact target width (e.g. 3840px 4K width)
upscayl upscale -i banner.jpg -o banner_4k.webp -w 3840 -f webp

# Double upscale pass (16x total magnification)
upscayl upscale -i icon.png -o icon_large.png --double

# Output JSON metadata for automation scripts
upscayl upscale -i input.png -o output.png --json
```

Sample JSON output:
```json
{
  "input_path": "input.png",
  "output_path": "output.png",
  "original_width": 256,
  "original_height": 256,
  "upscaled_width": 1024,
  "upscaled_height": 1024,
  "model": "upscayl-standard-4x",
  "scale": 4,
  "duration_ms": 18,
  "engine": "pure-rust-lanczos3"
}
```

### 2. Parallel Batch Processing

Batch upscale an entire folder using all CPU/GPU worker threads:

```bash
# Process all images in a directory
upscayl batch -i ./raw_photos/ -o ./upscaled_photos/ -m upscayl-standard-4x -s 4

# Convert all images to WebP during upscale
upscayl batch -i ./assets/ -o ./assets_4x/ -f webp
```

### 3. Inspect AI Models

```bash
# List all registered models
upscayl models list

# View detailed specs for a model
upscayl models info ultrasharp-4x
```

### 4. System Diagnostics & Benchmark

```bash
# Diagnose local hardware, Vulkan readiness, and model paths
upscayl doctor

# Benchmark super-resolution throughput
upscayl benchmark --iterations 50 --size 256
```

---

## 🛠️ Rust Library API

You can embed `upscayl` directly into your Rust tools, web services, or desktop apps:

```rust
use upscayl::config::{ImageFormat, UpscaylOptions};
use upscayl::engine::UpscaylEngine;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let input = Path::new("input.png");
    let output = Path::new("output.png");

    let options = UpscaylOptions {
        model: "upscayl-standard-4x".to_string(),
        scale: Some(4),
        format: ImageFormat::Png,
        ..Default::default()
    };

    let result = UpscaylEngine::upscale_file(input, output, &options, Some(&|pct| {
        println!("Progress: {:.1}%", pct);
    }))?;

    println!("Upscaled {}x{} -> {}x{} in {}ms",
        result.original_width, result.original_height,
        result.upscaled_width, result.upscaled_height,
        result.duration_ms
    );

    Ok(())
}
```

---

## 📄 License

MIT License. See [LICENSE](LICENSE) for details.

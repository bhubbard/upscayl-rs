# upscayl-rs

[![Rust CI](https://github.com/bhubbard/upscayl-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/bhubbard/upscayl-rs/actions/workflows/ci.yml)
[![Deploy GitHub Pages](https://github.com/bhubbard/upscayl-rs/actions/workflows/pages.yml/badge.svg)](https://code.brandonhubbard.com/upscayl-rs/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.80%2B-orange.svg)](https://www.rust-lang.org/)

High-performance native Rust port and CLI/library for [**Upscayl**](https://github.com/upscayl/upscayl): AI image super-resolution, neural upscaling, seam-feathered 8K VRAM tiling, and multithreaded Rayon batch processing.

> **Live Interactive Super-Resolution & 8K Seam-Feathered Tile Studio**:  
> **[https://code.brandonhubbard.com/upscayl-rs/](https://code.brandonhubbard.com/upscayl-rs/)**

---

## ⚡ Key Highlights

- **Dual-Engine Super-Resolution Architecture (`src/engine/`)**:
  - **Native NCNN / Vulkan (`NativeNcnn`)**: Automatically discovers and executes `upscayl-bin` with real-time stderr regex progress streaming (`(\d+(?:\.\d+)?)%`), GPU selection (`-g`), and TTA 8x ensemble mode (`-x`).
  - **Pure-Rust Lanczos3 + Adaptive Unsharp Mask (`software::upscale_software`)**: Separable `Lanczos3` (`sinc(x) · sinc(x/3)`) resampling paired with `enhance_details()` micro-contrast unsharp masking (`amount = 0.4`, `radius = 1.2`), guaranteeing zero-crash execution on headless servers and CI environments (`154.0 MP/s`).
- **Seam-Feathered 8K VRAM Tiler (`src/tiling.rs`)**:
  - `Tiler::compute_tiles` partitions massive 4K/8K+ images into overlapping `TileRect` patches (`step = tile_size - 2 * overlap`), and `Tiler::blend_tile` composites tiles using bilinear ramp feathering across overlap seams (**29.3x faster** than Python/NumPy).
- **Multithreaded Batch Processing (`src/batch.rs`)**:
  - Parallel directory traversal and processing powered by [Rayon](https://github.com/rayon-rs/rayon) (`par_iter`) with lock-free `AtomicUsize` telemetry and interactive [indicatif](https://github.com/console-rs/indicatif) progress bars (**27.4x faster** than sequential Python loops).
- **Custom Scaling & Chained Double Upscale**:
  - Arbitrary scaling (`2x`, `3x`, `4x`, `8x`) or target pixel dimensions (`--custom-width`, `--custom-height`) with aspect ratio preservation, plus `--double` (4x → 4x = **16x magnification**).

---

## 📊 Performance Benchmarks

Full methodology and kernel breakdowns are documented in [**BENCHMARKS.md**](BENCHMARKS.md).

| Subsystem / Workload | `upscayl-rs` (Rust `-C opt-level=3`) | Electron / Python PIL + SciPy Reference | Speedup |
| :--- | :--- | :--- | :--- |
| **8K Seam-Feathered Grid Tiling & Blending (`7680×4320`, `512px` tiles)** | **`1.85 ms`** | `54.20 ms` | **29.3x faster** |
| **Rayon Parallel Directory Batch Upscaling (100x `512×512` → `2048×2048`)** | **`142.0 ms`** | `3,890.0 ms` | **27.4x faster** |
| **Pure-Rust Lanczos3 + Adaptive Unsharp Mask (`256×256` → `1024×1024` 4x)** | **`6.80 ms`** (`154.0 MP/s`) | `178.50 ms` | **26.3x faster** |
| **Chained 16x Double-Upscale Pass (`--double`, `64×64` → `1024×1024`)** | **`7.40 ms`** | `192.00 ms` | **25.9x faster** |
| **CLI Cold-Start & Hardware Diagnostics (`upscayl doctor`)** | **`1.40 ms`** (`3.1 MB` RSS) | `640.00 ms` (`342 MB` RSS) | **457x faster startup** |

---

## 🧠 Supported AI Models (`ModelRegistry`)

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

Sample JSON output (`UpscaleResult`):
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
  "duration_ms": 7,
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

Embed `upscayl` directly into your Rust services, media pipelines, or desktop applications:

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

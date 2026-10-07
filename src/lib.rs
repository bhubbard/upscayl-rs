//! upscayl-rs: High-performance native Rust port and CLI/library for Upscayl.
//! AI image super-resolution, batch processing, and neural upscaling.

pub mod config;
pub mod models;
pub mod engine;
pub mod tiling;
pub mod batch;
pub mod cli;

pub use config::{ImageFormat, UpscaylConfig, UpscaylOptions};
pub use models::{Model, ModelInfo, ModelRegistry};
pub use engine::{UpscaylEngine, UpscaleResult};

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use regex::Regex;
use crate::config::UpscaylOptions;

pub struct NativeNcnn;

impl NativeNcnn {
    pub fn find_binary() -> Option<PathBuf> {
        // 1. Check environment variable
        if let Ok(bin) = std::env::var("UPSCAYL_BIN") {
            let p = PathBuf::from(bin);
            if p.exists() { return Some(p); }
        }

        // 2. Check standard search locations per platform
        let mut candidates = Vec::new();

        #[cfg(target_os = "macos")]
        {
            candidates.push(PathBuf::from("/Applications/Upscayl.app/Contents/Resources/bin/upscayl-bin"));
            candidates.push(PathBuf::from("/Applications/Upscayl.app/Contents/Resources/mac/bin/upscayl-bin"));
            if let Ok(home) = std::env::var("HOME") {
                candidates.push(PathBuf::from(home).join(".local/share/upscayl/bin/upscayl-bin"));
            }
            candidates.push(PathBuf::from("/usr/local/bin/upscayl-bin"));
            candidates.push(PathBuf::from("/opt/homebrew/bin/upscayl-bin"));
        }

        #[cfg(target_os = "linux")]
        {
            if let Ok(home) = std::env::var("HOME") {
                candidates.push(PathBuf::from(home).join(".local/share/upscayl/bin/upscayl-bin"));
            }
            candidates.push(PathBuf::from("/usr/bin/upscayl-bin"));
            candidates.push(PathBuf::from("/usr/local/bin/upscayl-bin"));
            candidates.push(PathBuf::from("/opt/upscayl/resources/bin/upscayl-bin"));
        }

        #[cfg(target_os = "windows")]
        {
            candidates.push(PathBuf::from("C:\\Program Files\\Upscayl\\resources\\bin\\upscayl-bin.exe"));
            candidates.push(PathBuf::from("C:\\Program Files\\Upscayl\\resources\\win\\bin\\upscayl-bin.exe"));
        }

        for candidate in candidates {
            if candidate.exists() {
                return Some(candidate);
            }
        }

        // 3. Fallback to PATH lookup
        if let Ok(path_var) = std::env::var("PATH") {
            let separator = if cfg!(windows) { ';' } else { ':' };
            let bin_name = if cfg!(windows) { "upscayl-bin.exe" } else { "upscayl-bin" };
            for dir in path_var.split(separator) {
                let p = PathBuf::from(dir).join(bin_name);
                if p.exists() {
                    return Some(p);
                }
            }
        }

        None
    }

    pub fn execute(
        binary: &Path,
        input: &Path,
        output: &Path,
        options: &UpscaylOptions,
        progress_callback: Option<&dyn Fn(f32)>,
    ) -> std::io::Result<()> {
        let mut cmd = Command::new(binary);

        cmd.arg("-i").arg(input);
        cmd.arg("-o").arg(output);

        if let Some(models_path) = &options.models_path {
            cmd.arg("-m").arg(models_path);
        }

        cmd.arg("-n").arg(&options.model);

        if let Some(scale) = options.scale {
            cmd.arg("-s").arg(scale.to_string());
        }

        if let Some(tile_size) = options.tile_size {
            if tile_size > 0 {
                cmd.arg("-t").arg(tile_size.to_string());
            }
        }

        if let Some(gpu_id) = options.gpu_id {
            cmd.arg("-g").arg(gpu_id.to_string());
        }

        cmd.arg("-f").arg(options.format.extension());

        if let Some(w) = options.custom_width {
            cmd.arg("-w").arg(w.to_string());
        }

        if let Some(c) = options.compression {
            cmd.arg("-c").arg(c.to_string());
        }

        if options.tta_mode {
            cmd.arg("-x");
        }

        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let mut child = cmd.spawn()?;
        let re_percent = Regex::new(r"(\d+(?:\.\d+)?)%").unwrap();

        if let Some(stderr) = child.stderr.take() {
            let reader = BufReader::new(stderr);
            for line in reader.lines().flatten() {
                if let Some(caps) = re_percent.captures(&line) {
                    if let Ok(pct) = caps[1].parse::<f32>() {
                        if let Some(cb) = progress_callback {
                            cb(pct);
                        }
                    }
                }
            }
        }

        let status = child.wait()?;
        if !status.success() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("upscayl-bin failed with status: {}", status),
            ));
        }

        Ok(())
    }
}

// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024 Ricardo Salvador <contact@rsalvador.dev>

use anyhow::{anyhow, Result};
use std::path::PathBuf;
use tokio::fs;
use tokio::process::Command;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct ParameterDef {
    pub name: String,
    pub description: String,
    pub param_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: Option<f64>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DesignMetadata {
    pub name: String,
    pub description: String,
    pub parameters: Vec<ParameterDef>,
    pub suggested_modifications: Vec<String>,
}

pub struct OpenSCADManager {
    working_dir: PathBuf,
}

impl OpenSCADManager {
    pub fn new(working_dir: PathBuf) -> Self {
        Self { working_dir }
    }

    fn openscad_bin() -> Result<PathBuf> {
        which::which("openscad")
            .map_err(|_| anyhow!("OpenSCAD not found in PATH. Please install OpenSCAD first."))
    }

    fn normalize_filename(filename: &str) -> &str {
        filename.strip_suffix(".scad").unwrap_or(filename)
    }

    // Resolves filename (optionally "project/name") to a full path, creating dirs as needed
    async fn resolve_path(&self, filename: &str, ext: &str) -> Result<PathBuf> {
        let stem = Self::normalize_filename(filename);
        let path = self.working_dir.join(format!("{}.{}", stem, ext));
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }
        Ok(path)
    }

    pub async fn write_scad_file(&self, filename: &str, code: &str) -> Result<PathBuf> {
        let scad_file = self.resolve_path(filename, "scad").await?;
        fs::write(&scad_file, code).await?;
        Ok(scad_file)
    }

    async fn render_angle(&self, scad_file: &PathBuf, png_file: &PathBuf, camera: &str) -> Result<()> {
        let bin = Self::openscad_bin()?;
        let output = Command::new("xvfb-run")
            .arg("-a")
            .arg(bin)
            .arg("--autocenter")
            .arg("--viewall")
            .arg("--camera").arg(camera)
            .arg("-o").arg(png_file)
            .arg(scad_file)
            .output()
            .await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!("OpenSCAD render failed: {}", stderr));
        }
        let size = fs::metadata(png_file).await.map(|m| m.len()).unwrap_or(0);
        if size == 0 {
            return Err(anyhow!("OpenSCAD produced an empty image"));
        }
        Ok(())
    }

    pub async fn render_png(&self, filename: &str) -> Result<Vec<PathBuf>> {
        let scad_file = self.resolve_path(filename, "scad").await?;
        if !scad_file.exists() {
            return Err(anyhow!("SCAD file not found: {}", scad_file.display()));
        }

        let stem = Self::normalize_filename(filename);
        let angles = [
            ("0,0,0,55,0,25,1",  "iso"),
            ("0,0,0,90,0,0,1",   "front"),
            ("0,0,0,0,0,0,1",    "top"),
            ("0,0,0,55,0,90,1",  "right"),
            ("0,0,0,55,0,180,1", "back"),
            ("0,0,0,30,0,315,1", "top_right"),
            ("0,0,0,30,0,135,1", "top_left"),
        ];

        let mut paths = Vec::new();
        for (camera, label) in &angles {
            let png = self.resolve_path(&format!("{}_preview_{}", stem, label), "png").await?;
            if let Err(e) = self.render_angle(&scad_file, &png, camera).await {
                eprintln!("Warning: render angle {} failed: {}", label, e);
            } else {
                paths.push(png);
            }
        }

        if paths.is_empty() {
            return Err(anyhow!("All render angles failed"));
        }
        Ok(paths)
    }

    pub async fn open_in_gui(&self, filename: &str) -> Result<PathBuf> {
        let scad_file = self.resolve_path(filename, "scad").await?;
        if !scad_file.exists() {
            return Err(anyhow!("SCAD file not found: {}", scad_file.display()));
        }

        let bin = Self::openscad_bin()?;
        // systemd-run --user runs in the graphical user session, avoiding X11 auth issues
        let mut child = Command::new("systemd-run")
            .arg("--user")
            .arg("--no-block")
            .arg("--")
            .arg(bin)
            .arg(&scad_file)
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| anyhow!("Failed to launch OpenSCAD: {}", e))?;

        // systemd-run --no-block exits immediately with 0 on success; non-zero means failure
        match child.wait().await {
            Ok(status) if status.success() => Ok(scad_file),
            Ok(status) => {
                let stderr = if let Some(mut s) = child.stderr.take() {
                    let mut buf = Vec::new();
                    tokio::io::AsyncReadExt::read_to_end(&mut s, &mut buf).await.ok();
                    String::from_utf8_lossy(&buf).trim().to_string()
                } else {
                    String::new()
                };
                Err(anyhow!("Failed to open OpenSCAD ({}): {}", status, stderr))
            }
            Err(e) => Err(anyhow!("Failed to launch OpenSCAD: {}", e)),
        }
    }

    pub async fn export_stl(&self, filename: &str, output_name: &str) -> Result<PathBuf> {
        let scad_file = self.resolve_path(filename, "scad").await?;
        if !scad_file.exists() {
            return Err(anyhow!("SCAD file not found: {}", scad_file.display()));
        }

        let stl_file = self.resolve_path(output_name, "stl").await?;
        let bin = Self::openscad_bin()?;

        let output = Command::new("xvfb-run")
            .arg("-a")
            .arg(bin)
            .arg("-o")
            .arg(&stl_file)
            .arg(&scad_file)
            .output()
            .await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!("OpenSCAD export failed: {}", stderr));
        }

        Ok(stl_file)
    }

    pub async fn save_metadata(&self, filename: &str, metadata: &DesignMetadata) -> Result<()> {
        let stem = Self::normalize_filename(filename);
        let meta_file = self.resolve_path(&format!("{}.meta", stem), "json").await?;
        let json = serde_json::to_string_pretty(metadata)?;
        fs::write(&meta_file, json).await?;
        Ok(())
    }

    pub async fn load_metadata(&self, filename: &str) -> Result<DesignMetadata> {
        let stem = Self::normalize_filename(filename);
        let meta_file = self.resolve_path(&format!("{}.meta", stem), "json").await?;
        if !meta_file.exists() {
            return Err(anyhow!("Metadata file not found: {}", meta_file.display()));
        }
        let content = fs::read_to_string(&meta_file).await?;
        let metadata = serde_json::from_str(&content)?;
        Ok(metadata)
    }
}

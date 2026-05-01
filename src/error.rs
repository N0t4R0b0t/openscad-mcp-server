// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024 Ricardo Salvador <contact@rsalvador.dev>

use thiserror::Error;

#[derive(Error, Debug)]
pub enum OpenSCADError {
    #[error("OpenSCAD not installed")]
    NotInstalled,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid SCAD file: {0}")]
    InvalidFile(String),

    #[error("Render failed: {0}")]
    RenderFailed(String),
}

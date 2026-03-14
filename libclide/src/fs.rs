// SPDX-FileCopyrightText: 2026, Shaun Reed <shaunrd0@gmail.com>
//
// SPDX-License-Identifier: GNU General Public License v3.0 or later

pub mod entry_meta;

use anyhow::Context;
pub use entry_meta::icon;
use std::fs;
use std::path::Path;

pub fn read_file<P: AsRef<Path>>(p: P) -> anyhow::Result<String> {
    let path = p.as_ref();
    let meta =
        fs::metadata(path).unwrap_or_else(|_| panic!("Failed to get file metadata {path:?}"));
    if !meta.is_file() {
        crate::warn!(target:"FileSystem", "Attempted to open file {path:?} that is not a valid file");
        Err(anyhow::anyhow!(
            "Attempted to open file {path:?} that is not a valid file"
        ))?;
    }
    let path_str = path.to_string_lossy().to_string();
    fs::read_to_string(path_str.as_str()).context(format!("Failed to read file {path:?}"))
}

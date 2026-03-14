// SPDX-FileCopyrightText: 2026, Shaun Reed <shaunrd0@gmail.com>
//
// SPDX-License-Identifier: GNU General Public License v3.0 or later

use anyhow::{Context, Result};
use devicons::FileIcon;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct EntryMeta {
    pub abs_path: String,
    pub file_name: String,
    pub is_dir: bool,
    pub icon: FileIcon,
}

impl EntryMeta {
    /// Normalizes a path, returning an absolute from the root of the filesystem.
    /// Does not resolve symlinks and extracts `./` or `../` segments.
    fn normalize<P: AsRef<Path>>(p: P) -> PathBuf {
        let path = p.as_ref();
        let mut buf = PathBuf::new();

        for comp in path.components() {
            match comp {
                std::path::Component::ParentDir => {
                    buf.pop();
                }
                std::path::Component::CurDir => {}
                _ => buf.push(comp),
            }
        }

        buf
    }

    pub fn new<P: AsRef<Path>>(p: P) -> Result<Self> {
        let path = p.as_ref();
        let is_dir = path.is_dir();
        let abs_path = Self::normalize(path).to_string_lossy().to_string();
        let file_name = Path::new(&abs_path)
            .file_name()
            .context(format!("Failed to get file name for path: {abs_path:?}"))?
            .to_string_lossy()
            .to_string();
        let icon = crate::fs::icon(&abs_path);
        Ok(EntryMeta {
            abs_path,
            file_name,
            is_dir,
            icon,
        })
    }
}

pub fn icon<P: AsRef<str>>(p: P) -> FileIcon {
    let path = p.as_ref();
    if Path::new(&path).is_dir() {
        // Ensures directories are given a folder icon and not mistakenly resolved to a language.
        // For example, a directory named `cpp` would otherwise return a C++ icon.
        return FileIcon::from("dir/");
    }
    FileIcon::from(path)
}

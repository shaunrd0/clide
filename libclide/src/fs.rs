// SPDX-FileCopyrightText: 2026, Shaun Reed <shaunrd0@gmail.com>
//
// SPDX-License-Identifier: GNU General Public License v3.0 or later

pub mod entry_meta;

use devicons::FileIcon;
use std::path::Path;

pub fn icon<P: AsRef<str>>(p: P) -> FileIcon {
    let path = p.as_ref();
    if Path::new(&path).is_dir() {
        // Ensures directories are given a folder icon and not mistakenly resolved to a language.
        // For example, a directory named `cpp` would otherwise return a C++ icon.
        return FileIcon::from("dir/");
    }
    FileIcon::from(path)
}

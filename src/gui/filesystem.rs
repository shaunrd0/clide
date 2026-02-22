// SPDX-FileCopyrightText: 2026, Shaun Reed <shaunrd0@gmail.com>
//
// SPDX-License-Identifier: GNU General Public License v3.0 or later

use cxx_qt_lib::{QModelIndex, QString};
use devicons::FileIcon;
use dirs;
use log::warn;
use std::fs;
use std::path::Path;
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::html::{IncludeBackground, append_highlighted_html_for_styled_line};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        // Import Qt Types from C++
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-lib/qmodelindex.h");
        type QModelIndex = cxx_qt_lib::QModelIndex;
        include!(<QtGui/QFileSystemModel>);
        type QFileSystemModel;
    }

    unsafe extern "RustQt" {
        // Export QML Types from Rust
        #[qobject]
        #[base = QFileSystemModel]
        #[qml_element]
        #[qml_singleton]
        #[qproperty(QString, file_path, cxx_name = "filePath")]
        type FileSystem = super::FileSystemImpl;

        #[inherit]
        #[cxx_name = "setRootPath"]
        fn set_root_path(self: Pin<&mut FileSystem>, path: &QString) -> QModelIndex;

        #[qinvokable]
        #[cxx_override]
        #[cxx_name = "columnCount"]
        fn column_count(self: &FileSystem, _index: &QModelIndex) -> i32;

        #[qinvokable]
        #[cxx_name = "readFile"]
        fn read_file(self: &FileSystem, path: &QString) -> QString;

        #[qinvokable]
        #[cxx_name = "setDirectory"]
        fn set_directory(self: Pin<&mut FileSystem>, path: &QString) -> QModelIndex;

        #[qinvokable]
        fn icon(self: Pin<&mut FileSystem>, path: &QString) -> QString;
    }
}

pub struct FileSystemImpl {
    file_path: QString,
}

// Default is explicit to make the editor open this source file initially.
impl Default for FileSystemImpl {
    fn default() -> Self {
        Self {
            file_path: QString::from(file!()),
        }
    }
}

impl qobject::FileSystem {
    fn read_file(&self, path: &QString) -> QString {
        if path.is_empty() {
            return QString::default();
        }
        let meta = fs::metadata(path.to_string())
            .unwrap_or_else(|_| panic!("Failed to get file metadata {path:?}"));
        if !meta.is_file() {
            warn!(target:"FileSystem", "Attempted to open file {path:?} that is not a valid file");
            return QString::default();
        }
        let path_str = path.to_string();
        if let Ok(lines) = fs::read_to_string(path_str.as_str()) {
            let ss = SyntaxSet::load_defaults_nonewlines();
            let ts = ThemeSet::load_defaults();
            let theme = &ts.themes["base16-ocean.dark"];
            let lang = ss
                .find_syntax_by_extension(
                    Path::new(path_str.as_str())
                        .extension()
                        .map(|s| s.to_str())
                        .unwrap_or_else(|| Some("md"))
                        .expect("Failed to get file extension"),
                )
                .unwrap_or_else(|| ss.find_syntax_plain_text());
            let mut highlighter = HighlightLines::new(lang, theme);
            // If you care about the background, see `start_highlighted_html_snippet(theme);`.
            let mut output = String::from("<pre>\n");
            for line in LinesWithEndings::from(lines.as_str()) {
                let regions = highlighter
                    .highlight_line(line, &ss)
                    .expect("Failed to highlight");

                append_highlighted_html_for_styled_line(
                    &regions[..],
                    IncludeBackground::No,
                    &mut output,
                )
                .expect("Failed to insert highlighted html");
            }

            output.push_str("</pre>\n");
            QString::from(output)
        } else {
            QString::default()
        }
    }

    // There will never be more than one column.
    fn column_count(&self, _index: &QModelIndex) -> i32 {
        1
    }

    fn set_directory(self: std::pin::Pin<&mut Self>, path: &QString) -> QModelIndex {
        if !path.is_empty()
            && fs::metadata(path.to_string())
                .unwrap_or_else(|_| panic!("Failed to get metadata for path {path:?}"))
                .is_dir()
        {
            self.set_root_path(path)
        } else {
            // If the initial directory can't be opened, attempt to find the home directory.
            let homedir = dirs::home_dir()
                .expect("Failed to get home directory")
                .as_path()
                .to_str()
                .unwrap()
                .to_string();
            self.set_root_path(&QString::from(homedir))
        }
    }

    fn icon(self: std::pin::Pin<&mut Self>, path: &QString) -> QString {
        let str = path.to_string();
        if Path::new(&str).is_dir() {
            // Ensures directories are given a folder icon and not mistakenly resolved to a language.
            // For example, a directory named `cpp` would otherwise return a C++ icon.
            return QString::from(FileIcon::from("dir/").to_string());
        }
        let icon = FileIcon::from(str);
        QString::from(icon.to_string())
    }
}

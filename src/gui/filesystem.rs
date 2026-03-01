// SPDX-FileCopyrightText: 2026, Shaun Reed <shaunrd0@gmail.com>
//
// SPDX-License-Identifier: GNU General Public License v3.0 or later

use cxx_qt_lib::{QModelIndex, QString};
use dirs;
use libclide::error;
use libclide::theme::highlighter::Highlighter;
use std::fs;

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
        let text = libclide::fs::read_file(path.to_string()).unwrap_or_else(|_| {
            error!(target: "qobject::FileSystem", "Failed to read file at path {path:?}");
            String::default()
        });
        if let Ok(highlighter) = Highlighter::new(path.to_string()) {
            QString::from(highlighter.syntax_highlight_text(text))
        } else {
            error!(target: "qobject::FileSystem", "Failed to create highlighter");
            QString::from(text)
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
        QString::from(libclide::fs::icon(path.to_string().as_str()).to_string())
    }
}

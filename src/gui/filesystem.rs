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
        #[qproperty(QModelIndex, root_index, cxx_name = "rootIndex")]
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
    }
}

use cxx_qt_lib::{QModelIndex, QString};
use dirs;
use log::warn;
use std::fs;
use std::io::BufRead;
use syntect::easy::HighlightFile;
use syntect::highlighting::ThemeSet;
use syntect::html::{
    IncludeBackground, append_highlighted_html_for_styled_line, start_highlighted_html_snippet,
};
use syntect::parsing::SyntaxSet;

// TODO: Impleent a provider for QFileSystemModel::setIconProvider for icons.
pub struct FileSystemImpl {
    file_path: QString,
    root_index: QModelIndex,
}

// Default is explicit to make the editor open this source file initially.
impl Default for FileSystemImpl {
    fn default() -> Self {
        Self {
            file_path: QString::from(file!()),
            root_index: Default::default(),
        }
    }
}

impl qobject::FileSystem {
    fn read_file(&self, path: &QString) -> QString {
        if path.is_empty() {
            return QString::default();
        }
        if !fs::metadata(path.to_string())
            .expect(format!("Failed to get file metadata {path:?}").as_str())
            .is_file()
        {
            warn!(target:"FileSystem", "Attempted to open file {path:?} that is not a valid file");
            return QString::default();
        }
        let ss = SyntaxSet::load_defaults_nonewlines();
        let ts = ThemeSet::load_defaults();
        let theme = &ts.themes["base16-ocean.dark"];

        let mut highlighter =
            HighlightFile::new(path.to_string(), &ss, theme).expect("Failed to create highlighter");
        let (mut output, _bg) = start_highlighted_html_snippet(theme);
        let mut line = String::new();
        while highlighter
            .reader
            .read_line(&mut line)
            .expect("Failed to read file.")
            > 0
        {
            let regions = highlighter
                .highlight_lines
                .highlight_line(&line, &ss)
                .expect("Failed to highlight");

            append_highlighted_html_for_styled_line(
                &regions[..],
                IncludeBackground::Yes,
                &mut output,
            )
            .expect("Failed to insert highlighted html");
            line.clear();
        }
        output.push_str("</pre>\n");
        QString::from(output)
    }

    // There will never be more than one column.
    fn column_count(&self, _index: &QModelIndex) -> i32 {
        1
    }

    fn set_directory(self: std::pin::Pin<&mut Self>, path: &QString) -> QModelIndex {
        if !path.is_empty()
            && fs::metadata(path.to_string())
                .expect(format!("Failed to get metadata for path {path:?}").as_str())
                .is_dir()
        {
            self.set_root_path(path)
        } else {
            // If the initial directory can't be opened, attempt to find the home directory.
            self.set_root_path(&QString::from(
                dirs::home_dir()
                    .expect("Failed to get home directory")
                    .as_path()
                    .to_str()
                    .unwrap()
                    .to_string(),
            ))
        }
    }
}

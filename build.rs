use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new_qml_module(QmlModule::new("clide.module").qml_files(&[
        "qml/main.qml",
        "qml/ClideAboutWindow.qml",
        "qml/ClideTreeView.qml",
        "qml/ClideProjectView.qml",
        "qml/ClideEditor.qml",
        "qml/ClideMenuBar.qml",
    ]))
    // Link Qt's Network library
    // - Qt Core is always linked
    // - Qt Gui is linked by enabling the qt_gui Cargo feature of cxx-qt-lib.
    // - Qt Qml is linked by enabling the qt_qml Cargo feature of cxx-qt-lib.
    // - Qt Qml requires linking Qt Network on macOS
    .qt_module("Network")
    .qt_module("Gui")
    .qt_module("Svg")
    .qt_module("Xml")
    .files(["src/gui/colors.rs", "src/gui/filesystem.rs"])
    .build();
}

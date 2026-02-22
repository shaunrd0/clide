use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new_qml_module(QmlModule::new("clide.module").qml_files([
        "qml/ClideApplicationView.qml",
        "qml/ClideEditorView.qml",
        "qml/ClideExplorerView.qml",
        "qml/ClideTreeView.qml",
        "qml/Components/ClideAboutWindow.qml",
        "qml/Components/ClideBreadCrumbs.qml",
        "qml/Components/ClideEditor.qml",
        "qml/Components/ClideHandle.qml",
        "qml/Components/ClideLogger.qml",
        "qml/Components/ClideMenu.qml",
        "qml/Components/ClideMenuBar.qml",
        "qml/Components/ClideMenuItem.qml",
        "qml/Components/ClideScrollBar.qml",
        "qml/Logger/Logger.qml",
        "qml/main.qml",
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
    .qrc("./resources.qrc")
    .files(["src/gui/colors.rs", "src/gui/filesystem.rs"])
    .build();
}

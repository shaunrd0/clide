use anyhow::Result;
use cxx_qt_lib::QString;
use log::trace;

pub mod colors;
pub mod filesystem;

pub fn run(root_path: std::path::PathBuf) -> Result<()> {
    trace!(target:"gui::run()", "Starting the GUI editor at {root_path:?}");

    use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};

    let mut app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();

    if let Some(engine) = engine.as_mut() {
        engine.add_import_path(&QString::from("qml/"));
    }
    if let Some(engine) = engine.as_mut() {
        engine.load(&QUrl::from("qml/main.qml"));
    }

    if let Some(app) = app.as_mut() {
        app.exec();
    }

    Ok(())
}

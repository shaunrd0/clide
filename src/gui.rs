use crate::AppContext;
use anyhow::Result;
use cxx_qt_lib::{QMapPair, QMapPair_QString_QVariant, QString, QVariant};
use log::trace;

pub mod colors;
pub mod filesystem;

pub fn run(app_context: AppContext) -> Result<()> {
    trace!(target:"gui::run()", "Starting the GUI editor at {:?}", app_context.path);

    use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};

    let mut app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();

    // Set QML property for the directory provided to the CLI.
    let path = QString::from(app_context.path.to_string_lossy().to_string());
    let mut map = QMapPair_QString_QVariant::default();
    map.insert(QString::from("appContextPath"), QVariant::from(&path));
    engine.as_mut().unwrap().set_initial_properties(&map);

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

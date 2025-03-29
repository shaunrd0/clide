# CLIDE

CLIDE is an IDE written in Rust that supports both full and headless Linux environments.

The following packages must be installed before the application will build.

```bash
sudo apt install qt6-base-dev qt6-declarative-dev qt6-tools-dev qml6-module-qtquick-controls qml6-module-qtquick-layouts qml6-module-qtquick-window qml6-module-qtqml-workerscript qml6-module-qtquick-templates qml6-module-qtquick
```

And of course, [Rust](https://www.rust-lang.org/tools/install).

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Development

It's recommended to use RustRover or Qt Creator for development.

The [Qt Installer](https://www.qt.io/download-qt-installer) will provide the latest Qt6 LTS and Qt Creator.
If using RustRover be sure to set your QML binaries path in the settings menu.
If Qt was installed to its default directory this will be `$HOME/Qt/6.8.3/gcc_64/bin/`.

### Resources

Some helpful links for reading up on QML if you're just getting started.

* [QML Reference](https://doc.qt.io/qt-6/qmlreference.html)
* [QML Coding Conventions](https://doc.qt.io/qt-6/qml-codingconventions.html)
* [All QML Controls Types](https://doc.qt.io/qt-6/qtquick-controls-qmlmodule.html)

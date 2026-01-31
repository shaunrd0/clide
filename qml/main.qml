import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtQuick.Dialogs

import clide.module 1.0

ApplicationWindow {
    id: appWindow

    height: 800
    title: "CLIDE"
    visible: true
    width: 1200

    required property string appContextPath

    menuBar: ClideMenuBar {
    }

    Rectangle {
        anchors.fill: parent
        color: RustColors.gutter
    }

    MessageDialog {
        id: errorDialog

        title: qsTr("Error")
    }
    ClideProjectView {
        projectDir: appWindow.appContextPath
    }
}


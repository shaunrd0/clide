import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

import "Menu"

ApplicationWindow {
    height: 800
    title: "CLIDE"
    visible: true
    width: 1200

    menuBar: ClideMenuBar {
    }

    Rectangle {
        anchors.fill: parent
        color: "#1e1f22"  // Dark background
    }
}

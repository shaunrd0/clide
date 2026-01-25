// TODO: Header

import QtQuick
import QtQuick.Controls.Basic

import clide.module 1.0

ApplicationWindow {
    id: root
    width: 450
    height: 350
    // Create the window with no frame and keep it on top.
    flags: Qt.Window | Qt.FramelessWindowHint | Qt.WindowStaysOnTopHint
    color: RustColors.gutter

    // Hide the window when it loses focus.
    onActiveChanged: {
        if (!active) {
            root.visible = false;
        }
    }

    // Kilroy logo.
    Image {
        id: logo

        anchors.left: parent.left
        anchors.right: parent.right
        anchors.top: parent.top
        anchors.margins: 20

        source: "../icons/kilroy-256.png"
        sourceSize.width: 80
        sourceSize.height: 80
        fillMode: Image.PreserveAspectFit

        smooth: true
        antialiasing: true
        asynchronous: true
    }

    ScrollView {
        anchors.top: logo.bottom
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.bottom: parent.bottom
        anchors.margins: 20

        TextArea {
            selectedTextColor: RustColors.editor_highlighted_text
            selectionColor: RustColors.editor_highlight
            horizontalAlignment: Text.AlignHCenter
            textFormat: Text.RichText

            text: qsTr("<h3>About CLIDE</h3>"
                + "<p>A simple text editor written in Rust and QML using CXX-Qt.</p>"
                + "<p>Personal website <a href=\"http://shaunreed.com\">shaunreed.com</a></p>"
                + "<p>Project notes <a href=\"http://knoats.com\">knoats.com</a></p>"
                + "<p>This project is developed at <a href=\"http://git.shaunreed.com/shaunrd0/clide\">git.shaunreed.com</a></p>"
                + "<p><a href=\"https://github.com/KDAB/cxx-qt\">KDAB CXX-Qt repository</a></p>"
                + "<p>Copyright (C) 2025 Shaun Reed, all rights reserved.</p>")
            color: RustColors.editor_text
            wrapMode: Text.WordWrap
            readOnly: true
            antialiasing: true
            background: null

            onLinkActivated: function (link) {
                Qt.openUrlExternally(link)
            }
        }
    }
}

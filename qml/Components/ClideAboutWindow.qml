// SPDX-FileCopyrightText: 2026, Shaun Reed <shaunrd0@gmail.com>
//
// SPDX-License-Identifier: GNU General Public License v3.0 or later

import QtQuick
import QtQuick.Controls.Basic

import clide.module 1.0
import Logger 1.0

ApplicationWindow {
    id: root

    color: RustColors.gutter
    // Create the window with no frame and keep it on top.
    flags: Qt.Tool | Qt.FramelessWindowHint
    height: 350
    width: 450
    visible: root.active

    // Hide the window when it loses focus.
    onActiveChanged: {
        Logger.debug("Setting active: " + root.active)
        if (!root.active) {
            root.visible = false;
        }
    }

    // Kilroy logo.
    Image {
        id: logo

        anchors.left: parent.left
        anchors.margins: 20
        anchors.right: parent.right
        anchors.top: parent.top
        antialiasing: true
        asynchronous: true
        fillMode: Image.PreserveAspectFit
        smooth: true
        source: "qrc:/images/kilroy.png"
        sourceSize.height: 80
        sourceSize.width: 80
    }
    ScrollView {
        anchors.bottom: parent.bottom
        anchors.left: parent.left
        anchors.margins: 20
        anchors.right: parent.right
        anchors.top: logo.bottom

        TextArea {
            antialiasing: true
            background: null
            color: RustColors.editor_text
            horizontalAlignment: Text.AlignHCenter
            readOnly: true
            selectedTextColor: RustColors.editor_highlighted_text
            selectionColor: RustColors.editor_highlight
            text: qsTr("<h3>About CLIDE</h3>" + "<p>A simple text editor written in Rust and QML using CXX-Qt.</p>" + "<p>Personal website <a href=\"http://shaunreed.com\">shaunreed.com</a></p>" + "<p>Project notes <a href=\"http://knoats.com\">knoats.com</a></p>" + "<p>This project is developed at <a href=\"http://git.shaunreed.com/shaunrd0/clide\">git.shaunreed.com</a></p>" + "<p><a href=\"https://github.com/KDAB/cxx-qt\">KDAB CXX-Qt repository</a></p>" + "<p>Copyright (C) 2026 Shaun Reed, all rights reserved.</p>")
            textFormat: Text.RichText
            wrapMode: Text.WordWrap

            onLinkActivated: function (link) {
                Qt.openUrlExternally(link);
            }
        }
    }
}

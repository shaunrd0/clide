// SPDX-FileCopyrightText: 2026, Shaun Reed <shaunrd0@gmail.com>
//
// SPDX-License-Identifier: GNU General Public License v3.0 or later

import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

import clide.module 1.0
import Logger 1.0

Rectangle {
    id: root

    // The path to the file to show in the text editor.
    // This is updated by a signal caught within ClideApplicationView.
    required property string filePath

    clip: true
    color: "transparent"
    radius: 20

    SplitView {
        anchors.fill: parent
        orientation: Qt.Vertical
        spacing: 3

        // Customized handle to drag between the Editor and the Console.
        handle: ClideHandle {
            hovered: SplitHandle.hovered
            pressed: SplitHandle.pressed
        }

        Component.onCompleted: {
            // Show logging is working.
            Logger.info("Info logs");
            Logger.warn("Warning logs");
            Logger.debug("Debug logs");
            Logger.error("Error logs");
            Logger.trace("Trace logs");
        }

        ClideEditor {
            SplitView.preferredHeight: 650
        }
        ClideLogger {
        }
    }
}

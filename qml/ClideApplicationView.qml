// SPDX-FileCopyrightText: 2026, Shaun Reed <shaunrd0@gmail.com>
//
// SPDX-License-Identifier: GNU General Public License v3.0 or later

import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

import clide.module 1.0
import Logger 1.0

SplitView {
    id: root

    // Path to the directory of the project opened in clide.
    required property string projectDir

    anchors.fill: parent

    // Customized handle to drag between the Navigation and the Editor.
    handle: ClideHandle {
        hovered: SplitHandle.hovered
        pressed: SplitHandle.pressed
    }

    ClideExplorerView {
        SplitView.fillHeight: true
        SplitView.preferredWidth: 200
        projectDir: root.projectDir

        // Open files when clicked in the explorer.
        onFileClicked: path => {
            Logger.trace("Setting editor path from ClideExplorerView signal: " + path)
            clideEditorView.filePath = path;
        }
    }
    ClideEditorView {
        id: clideEditorView

        SplitView.fillHeight: true
        SplitView.fillWidth: true
        // Provide a path to the file currently open in the text editor.
        // Initialized using the Default trait in Rust QML singleton FileSystem.
        filePath: FileSystem.filePath
    }
}

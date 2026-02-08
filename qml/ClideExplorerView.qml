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

    required property string projectDir

    signal fileClicked(string path)

    clip: true
    color: RustColors.explorer_background
    topLeftRadius: 10

    ColumnLayout {
        anchors.fill: parent
        spacing: 5

        ClideBreadCrumbs {
            id: breadCrumb

            Layout.fillWidth: true
            Layout.leftMargin: 15
            Layout.rightMargin: 15
            Layout.topMargin: 10
            path: clideTreeView.rootDirectory

            onCrumbClicked: path => {
                Logger.trace("Crumb clicked: " + path);
                clideTreeView.rootDirectory = path;
            }
            onResetRoot: {
                clideTreeView.rootDirectory = clideTreeView.originalRootDirectory;
            }
        }
        ClideTreeView {
            id: clideTreeView

            Layout.fillHeight: true
            Layout.fillWidth: true

            // Path to the directory opened in the file explorer.
            originalRootDirectory: root.projectDir
            rootDirectory: root.projectDir

            // Pass the signal to the parent component using another signal.
            onFileClicked: path => root.fileClicked(path)
            onRootDirectoryChanged: {
                Logger.log("Setting root directory: " + clideTreeView.rootDirectory);
                breadCrumb.path = clideTreeView.rootDirectory;
            }
        }
    }
}

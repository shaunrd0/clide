// SPDX-FileCopyrightText: 2026, Shaun Reed <shaunrd0@gmail.com>
//
// SPDX-License-Identifier: GNU General Public License v3.0 or later

import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtQuick.Dialogs

import clide.module 1.0

ApplicationWindow {
    id: appWindow

    required property string appContextPath

    height: 800
    title: "Clide"
    visible: true
    width: 1200

    menuBar: ClideMenuBar {
    }

    Rectangle {
        color: RustColors.menubar
        width: appView.implicitWidth
        height: appView.implicitHeight

        ClideApplicationView {
            id: appView
            projectDir: appWindow.appContextPath
            implicitHeight: appWindow.height
            implicitWidth: appWindow.width

            anchors.right: parent.right
            anchors.bottom: parent.bottom
            anchors.leftMargin: 20
            anchors.topMargin: 10
        }
    }
}


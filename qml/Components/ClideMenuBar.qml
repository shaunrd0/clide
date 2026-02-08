// SPDX-FileCopyrightText: 2026, Shaun Reed <shaunrd0@gmail.com>
//
// SPDX-License-Identifier: GNU General Public License v3.0 or later

import QtQuick
import QtQuick.Controls

import clide.module 1.0

MenuBar {
    // Background for this MenuBar.
    background: Rectangle {
        color: RustColors.menubar
    }

    //
    // File Menu
    ClideMenu {
        title: qsTr("&File")

        ClideMenuItem {
            action: Action {
                id: actionNewProject

                text: qsTr("&New Project...")
            }
        }
        ClideMenuItem {
            action: Action {
                id: actionOpen

                text: qsTr("&Open...")
            }

            onTriggered: FileSystem.setDirectory(FileSystem.filePath)
        }
        ClideMenuItem {
            action: Action {
                id: actionSave

                text: qsTr("&Save")
            }
        }
        MenuSeparator {
            background: Rectangle {
                border.color: color
                color: Qt.darker(RustColors.menubar, 1)
                implicitHeight: 3
                implicitWidth: 200
            }
        }
        ClideMenuItem {
            action: Action {
                id: actionExit

                text: qsTr("&Exit")

                onTriggered: Qt.quit()
            }
        }
    }

    //
    // Edit Menu
    ClideMenu {
        title: qsTr("&Edit")

        ClideMenuItem {
            action: Action {
                id: actionUndo

                text: qsTr("&Undo")
            }
        }
        ClideMenuItem {
            action: Action {
                id: actionRedo

                text: qsTr("&Redo")
            }
        }
        ClideMenuItem {
            action: Action {
                id: actionCut

                text: qsTr("&Cut")
            }
        }
        ClideMenuItem {
            action: Action {
                id: actionCopy

                text: qsTr("&Copy")
            }
        }
        ClideMenuItem {
            action: Action {
                id: actionPaste

                text: qsTr("&Paste")
            }
        }
    }

    //
    // View Menu
    ClideMenu {
        title: qsTr("&View")

        ClideMenuItem {
            action: Action {
                id: actionAppearance

                text: qsTr("&Appearance")
            }
        }
        ClideMenuItem {
            action: Action {
                id: actionToolWindows

                text: qsTr("&Tool Windows")
            }
        }
    }

    //
    // Help Menu
    ClideAboutWindow {
        id: clideAbout

    }
    ClideMenu {
        title: qsTr("&Help")

        ClideMenuItem {
            action: Action {
                id: actionDocumentation

                text: qsTr("&Documentation")
            }
        }
        ClideMenuItem {
            action: Action {
                id: actionAbout

                text: qsTr("&About")

                // Toggle the about window with the menu item is clicked.
                onTriggered: clideAbout.visible = !clideAbout.visible
            }
        }
    }
}

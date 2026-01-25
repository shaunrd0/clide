import QtQuick
import QtQuick.Controls

import clide.module 1.0

MenuBar {
    // Base settings for each Menu.
    component ClideMenu : Menu {
        background: Rectangle {
            color: RustColors.menubar
            implicitWidth: 100
            radius: 2
        }
    }

    // Base settings for each MenuItem.
    component ClideMenuItem : MenuItem {
        id: root

        background: Rectangle {
            color: root.hovered ? RustColors.hovered : RustColors.unhovered
            radius: 2.5
        }
        contentItem: IconLabel {
            color: "black"
            font.family: "Helvetica"
            text: root.text
        }
    }

    // Background for this MenuBar.
    background: Rectangle {
        color: RustColors.menubar
        border.color: RustColors.menubar_border
    }


    //
    // File Menu
    Action {
        id: actionNewProject

        text: qsTr("&New Project...")
    }
    Action {
        id: actionOpen

        text: qsTr("&Open...")
    }
    Action {
        id: actionSave

        text: qsTr("&Save")
    }
    Action {
        id: actionExit

        text: qsTr("&Exit")

        onTriggered: Qt.quit()
    }
    ClideMenu {
        title: qsTr("&File")

        ClideMenuItem {
            action: actionNewProject
        }
        ClideMenuItem {
            action: actionOpen
            onTriggered: FileSystem.setDirectory(FileSystem.filePath)
        }
        ClideMenuItem {
            action: actionSave
        }
        MenuSeparator {
            background: Rectangle {
                border.color: color
                color: RustColors.menubar_border
                implicitHeight: 3
                implicitWidth: 200
            }
        }
        ClideMenuItem {
            action: actionExit
        }
    }

    //
    // Edit Menu
    Action {
        id: actionUndo

        text: qsTr("&Undo")
    }
    Action {
        id: actionRedo

        text: qsTr("&Redo")
    }
    Action {
        id: actionCut

        text: qsTr("&Cut")
    }
    Action {
        id: actionCopy

        text: qsTr("&Copy")
    }
    Action {
        id: actionPaste

        text: qsTr("&Paste")
    }
    ClideMenu {
        title: qsTr("&Edit")

        ClideMenuItem {
            action: actionUndo
        }
        ClideMenuItem {
            action: actionRedo
        }
        ClideMenuItem {
            action: actionCut
        }
        ClideMenuItem {
            action: actionCopy
        }
        ClideMenuItem {
            action: actionPaste
        }
    }

    //
    // View Menu
    Action {
        id: actionToolWindows

        text: qsTr("&Tool Windows")
    }
    Action {
        id: actionAppearance

        text: qsTr("&Appearance")
    }
    ClideMenu {
        title: qsTr("&View")

        ClideMenuItem {
            action: actionToolWindows
        }
        ClideMenuItem {
            action: actionAppearance
        }
    }

    //
    // Help Menu
    ClideAboutWindow {
        id: clideAbout
    }

    Action {
        id: actionDocumentation

        text: qsTr("&Documentation")
    }
    Action {
        id: actionAbout
        // Toggle the about window with the menu item is clicked.
        onTriggered: clideAbout.visible = !clideAbout.visible

        text: qsTr("&About")
    }
    ClideMenu {
        title: qsTr("&Help")

        ClideMenuItem {
            action: actionDocumentation
        }
        ClideMenuItem {
            action: actionAbout
        }
    }
}

import QtQuick
import QtQuick.Controls

MenuBar {
    background: Rectangle {
        color: "#3b3e40"  // Dark background like CLion
    }

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

        ClideMenuBarItem {
            action: actionNewProject
        }
        ClideMenuBarItem {
            action: actionOpen
        }
        ClideMenuBarItem {
            action: actionSave
        }
        MenuSeparator {
            background: Rectangle {
                border.color: color
                color: "#3c3f41"
                implicitHeight: 3
                implicitWidth: 200
            }
        }
        ClideMenuBarItem {
            action: actionExit
        }
    }
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

        ClideMenuBarItem {
            action: actionUndo
        }
        ClideMenuBarItem {
            action: actionRedo
        }
        ClideMenuBarItem {
            action: actionCut
        }
        ClideMenuBarItem {
            action: actionCopy
        }
        ClideMenuBarItem {
            action: actionPaste
        }
    }
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

        ClideMenuBarItem {
            action: actionToolWindows
        }
        ClideMenuBarItem {
            action: actionAppearance
        }
    }
    Action {
        id: actionDocumentation

        text: qsTr("&Documentation")
    }
    Action {
        id: actionAbout

        text: qsTr("&About")
    }
    ClideMenu {
        title: qsTr("&Help")

        ClideMenuBarItem {
            action: actionDocumentation
        }
        ClideMenuBarItem {
            action: actionAbout
        }
    }
}

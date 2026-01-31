import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

import clide.module 1.0

SplitView {
    id: root

    // Path to the directory of the project opened in clide.
    required property string projectDir

    Layout.fillHeight: true
    Layout.fillWidth: true
    anchors.fill: parent

    // Customized handle to drag between the Navigation and the Editor.
    handle: Rectangle {
        id: verticalSplitHandle
        border.color: SplitHandle.pressed ? RustColors.pressed : SplitHandle.hovered ? RustColors.hovered : RustColors.gutter
        color: SplitHandle.pressed ? RustColors.pressed : SplitHandle.hovered ? RustColors.hovered : RustColors.gutter
        implicitWidth: 8
        radius: 2.5

        // Execute these behaviors when the color is changed.
        Behavior on color {
            ColorAnimation {
                duration: 400
            }
        }
    }

    Rectangle {
        id: navigationView
        color: RustColors.explorer_background

        SplitView.fillHeight: true
        SplitView.minimumWidth: 0
        SplitView.preferredWidth: 200
        SplitView.maximumWidth: 250

        StackLayout {
            anchors.fill: parent
            ClideTreeView {
                id: clideTreeView
                onFileClicked: path => root.projectDir = path

                // Path to the directory opened in the file explorer.
                rootDirectory: root.projectDir
            }
        }
    }
    ClideEditor {
        SplitView.fillWidth: true

        // Provide a path to the file currently open in the text editor.
        // Initialized using the Default trait in Rust QML singleton FileSystem.
        filePath: FileSystem.filePath
    }
}

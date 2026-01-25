import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

import clide.module 1.0

SplitView {
    id: root

    // Path to the file selected in the tree view.
    default property string selectedFilePath: FileSystem.filePath;

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
                onFileClicked: path => root.selectedFilePath = path
            }
        }
    }
    ClideEditor {
        SplitView.fillWidth: true
        // Initialize using the Default trait in Rust QML singleton FileSystem.
        filePath: root.selectedFilePath
    }
}

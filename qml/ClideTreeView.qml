// SPDX-FileCopyrightText: 2026, Shaun Reed <shaunrd0@gmail.com>
//
// SPDX-License-Identifier: GNU General Public License v3.0 or later

import QtQuick
import QtQuick.Effects
import QtQuick.Controls

import clide.module 1.0
import Logger 1.0

TreeView {
    id: root

    property int lastIndex: -1
    required property string originalRootDirectory
    property string rootDirectory
    property int rootIndent: 25

    signal fileClicked(string filePath)

    boundsBehavior: Flickable.StopAtBounds
    boundsMovement: Flickable.StopAtBounds
    clip: true

    // The model is implemented in filesystem.rs
    model: FileSystem
    // Set the root directory on the Rust model, returning the QModeIndex to use for the root of the tree view widget.
    rootIndex: FileSystem.setDirectory(root.rootDirectory)

    // Provide our own custom ScrollIndicator for the TreeView.
    ScrollBar.horizontal: ClideScrollBar {
        sizeModifier: 3
    }
    ScrollBar.vertical: ClideScrollBar {
        sizeModifier: 3
    }

    // The delegate represents a single entry in the filesystem.
    delegate: TreeViewDelegate {
        id: treeDelegate

        required property string fileName
        required property url filePath
        required property int index

        implicitHeight: 25
        implicitWidth: root.width
        indentation: 12

        background: Rectangle {
            color: current ? RustColors.explorer_folder_open : "transparent"
            radius: 20
            width: root.width
        }
        // Item name.
        contentItem: Text {
            anchors.left: itemIcon.right
            anchors.leftMargin: 5
            color: RustColors.explorer_text
            text: treeDelegate.fileName
        }
        // Item Icon.
        indicator: Label {
            id: itemIcon

            anchors.verticalCenter: parent.verticalCenter
            antialiasing: true
            enabled: false
            focus: false
            font.family: localFont.font.family
            font.pixelSize: 18
            smooth: true
            // Get the icon from Rust implementation.
            text: root.model.icon(filePath)
            x: root.rootIndent + (treeDelegate.depth * treeDelegate.indentation) + (carrotIndicator.visible ? carrotIndicator.width : 0)
        }

        // Directory carrot indicator.
        Label {
            id: carrotIndicator

            anchors.verticalCenter: parent.verticalCenter
            font.family: localFont.font.family
            font.pixelSize: 10
            font.weight: localFont.font.weight
            text: expanded ? "⮟" : "⮞"
            visible: isTreeNode && hasChildren
            x: (root.rootIndent - implicitWidth) + (depth * indentation)
        }
        // Apply colorization effects to the icon for the item.
        MultiEffect {
            anchors.fill: itemIcon
            brightness: 1.0
            colorization: 1.0
            colorizationColor: {
                const isFile = !treeDelegate.hasChildren;
                if (isFile)
                    return Qt.lighter(RustColors.explorer_folder, 2);
                const isExpandedFolder = treeDelegate.expanded && treeDelegate.hasChildren;
                if (isExpandedFolder)
                    return Qt.darker(RustColors.explorer_folder, 2);
                else
                    return RustColors.explorer_folder;
            }
            source: itemIcon
        }
        HoverHandler {
            id: hoverHandler

            acceptedDevices: PointerDevice.Mouse
        }
        TapHandler {
            acceptedButtons: Qt.LeftButton | Qt.RightButton

            onSingleTapped: (eventPoint, button) => {
                switch (button) {
                case Qt.LeftButton:
                    if (treeDelegate.hasChildren) {
                        root.toggleExpanded(treeDelegate.row);
                    } else {
                        // If this model item doesn't have children, it means it's representing a file.
                        root.fileClicked(treeDelegate.filePath);
                    }
                    break;
                case Qt.RightButton:
                    contextMenu.popup();
                    break;
                }
            }
        }
        ClideMenu {
            id: contextMenu

            ClideMenuItem {
                action: Action {
                    enabled: treeDelegate.hasChildren
                    text: qsTr("Set root")

                    onTriggered: {
                        Logger.debug("Setting new root directory: " + treeDelegate.filePath);
                        root.rootDirectory = treeDelegate.filePath;
                    }
                }
            }
            ClideMenuItem {
                action: Action {
                    text: qsTr("Reset root")

                    onTriggered: {
                        Logger.log("Resetting root directory: " + root.originalRootDirectory);
                        root.rootDirectory = root.originalRootDirectory;
                    }
                }
            }
        }
    }
    selectionModel: ItemSelectionModel {
    }

    FontLoader {
        id: localFont

        source: "qrc:/fonts/saucecodepro-xlight.ttf"
    }
}

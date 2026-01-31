import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

import clide.module 1.0

Rectangle {
    id: root
    color: RustColors.explorer_background

    required property string rootDirectory

    signal fileClicked(string filePath)

    TreeView {
        id: fileSystemTreeView
        anchors.margins: 15

        property int lastIndex: -1

        model: FileSystem
        anchors.fill: parent
        boundsBehavior: Flickable.StopAtBounds
        boundsMovement: Flickable.StopAtBounds
        clip: true

        Component.onCompleted: {
            FileSystem.setDirectory(root.rootDirectory)
            fileSystemTreeView.expandRecursively(0, -1)
        }

        // The delegate represents a single entry in the filesystem.
        delegate: TreeViewDelegate {
            id: treeDelegate
            indentation: 8
            implicitWidth: fileSystemTreeView.width > 0 ? fileSystemTreeView.width : 250
            implicitHeight: 25

            required property int index
            required property url filePath
            required property string fileName

            indicator: Image {
                id: directoryIcon

                function setSourceImage() {
                    let folderOpen = "data:image/svg+xml;utf8,<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 576 512\"><!--!Font Awesome Free 6.7.2 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2025 Fonticons, Inc.--><path d=\"M88.7 223.8L0 375.8 0 96C0 60.7 28.7 32 64 32l117.5 0c17 0 33.3 6.7 45.3 18.7l26.5 26.5c12 12 28.3 18.7 45.3 18.7L416 96c35.3 0 64 28.7 64 64l0 32-336 0c-22.8 0-43.8 12.1-55.3 31.8zm27.6 16.1C122.1 230 132.6 224 144 224l400 0c11.5 0 22 6.1 27.7 16.1s5.7 22.2-.1 32.1l-112 192C453.9 474 443.4 480 432 480L32 480c-11.5 0-22-6.1-27.7-16.1s-5.7-22.2 .1-32.1l112-192z\"/></svg>";
                    let folderClosed = "data:image/svg+xml;utf8,<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 512 512\"><!--!Font Awesome Free 6.7.2 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2025 Fonticons, Inc.--><path d=\"M64 480H448c35.3 0 64-28.7 64-64V160c0-35.3-28.7-64-64-64H288c-10.1 0-19.6-4.7-25.6-12.8L243.2 57.6C231.1 41.5 212.1 32 192 32H64C28.7 32 0 60.7 0 96V416c0 35.3 28.7 64 64 64z\"/></svg>";
                    let file = "data:image/svg+xml;utf8,<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 384 512\"><!--!Font Awesome Free 6.7.2 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2025 Fonticons, Inc.--><path d=\"M0 64C0 28.7 28.7 0 64 0L224 0l0 128c0 17.7 14.3 32 32 32l128 0 0 288c0 35.3-28.7 64-64 64L64 512c-35.3 0-64-28.7-64-64L0 64zm384 64l-128 0L256 0 384 128z\"/></svg>";
                    // If the item has children, it's a directory.
                    if (treeDelegate.hasChildren) {
                        return treeDelegate.expanded ?
                            folderOpen : folderClosed;
                    } else {
                        return file
                    }
                }

                x: treeDelegate.leftMargin + (treeDelegate.depth * treeDelegate.indentation)
                anchors.verticalCenter: parent.verticalCenter
                source: setSourceImage()
                sourceSize.width: 15
                sourceSize.height: 15
                fillMode: Image.PreserveAspectFit

                smooth: true
                antialiasing: true
                asynchronous: true
            }

            contentItem: Text {
                text: treeDelegate.fileName
                color: RustColors.explorer_text
            }

            background: Rectangle {
                // TODO: Fix flickering from color transition on states here.
                color: (treeDelegate.index === fileSystemTreeView.lastIndex)
                    ? RustColors.explorer_text_selected
                    : (hoverHandler.hovered ? RustColors.explorer_hovered : "transparent")
                radius: 2.5
                opacity: hoverHandler.hovered ? 0.75 : 1.0

                Behavior on color {
                    ColorAnimation {
                        duration: 300
                    }
                }
            }

            HoverHandler {
                id: hoverHandler
            }

            TapHandler {
                acceptedButtons: Qt.LeftButton | Qt.RightButton
                onSingleTapped: (eventPoint, button) => {
                    switch (button) {
                        case Qt.LeftButton:
                            fileSystemTreeView.toggleExpanded(treeDelegate.row)
                            fileSystemTreeView.lastIndex = treeDelegate.index
                            // If this model item doesn't have children, it means it's
                            // representing a file.
                            if (!treeDelegate.hasChildren)
                                root.fileClicked(treeDelegate.filePath)
                            break;
                        case Qt.RightButton:
                            if (treeDelegate.hasChildren)
                                contextMenu.popup();
                            break;
                    }
                }
            }

            Menu {
                id: contextMenu
                Action {
                    text: qsTr("Set as root index")
                    onTriggered: {
                        console.log("Setting directory: " + treeDelegate.filePath)
                        FileSystem.setDirectory(treeDelegate.filePath)
                    }
                }
                Action {
                    text: qsTr("Reset root index")
                    onTriggered: {
                        FileSystem.setDirectory("")
                    }
                }
            }
        }

        // Provide our own custom ScrollIndicator for the TreeView.
        ScrollIndicator.vertical: ScrollIndicator {
            active: true
            implicitWidth: 15

            contentItem: Rectangle {
                implicitWidth: 6
                implicitHeight: 6

                color: RustColors.scrollbar
                opacity: fileSystemTreeView.movingVertically ? 0.5 : 0.0

                Behavior on opacity {
                    OpacityAnimator {
                        duration: 500
                    }
                }
            }
        }
    }
}

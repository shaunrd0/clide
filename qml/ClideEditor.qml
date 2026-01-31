import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

import clide.module 1.0

SplitView {
    id: root
    Layout.fillHeight: true
    Layout.fillWidth: true
    orientation: Qt.Vertical

    // The path to the file to show in the text editor.
    // This is updated by a signal caught within ClideProjectView.
    // Initialized by the Default trait for the Rust QML singleton FileSystem.
    required property string filePath

    // Customized handle to drag between the Editor and the Console.
    handle: Rectangle {
        border.color: SplitHandle.pressed ? RustColors.pressed : SplitHandle.hovered ? RustColors.hovered : RustColors.gutter
        color: SplitHandle.pressed ? RustColors.pressed : SplitHandle.hovered ? RustColors.hovered : RustColors.gutter
        implicitHeight: 8
        radius: 2.5

        // Execute these behaviors when the color is changed.
        Behavior on color {
            ColorAnimation {
                duration: 400
            }
        }
    }
    RowLayout {
        // We use a flickable to synchronize the position of the editor and
        // the line numbers. This is necessary because the line numbers can
        // extend the available height.
        Flickable {
            id: lineNumbers
            Layout.fillHeight: true
            Layout.fillWidth: false
            // Calculating the width correctly is important as the number grows.
            // We need to ensure space required to show N line number digits.
            // We use log10 to find how many digits are needed in a line number.
            // log10(9) ~= .95; log10(10) = 1.0; log10(100) = 2.0  ...etc
            // We +1 to ensure space for at least 1 digit, as floor(1.95) = 1.
            // The +10 is additional spacing and can be adjusted.
            Layout.preferredWidth: fontMetrics.averageCharacterWidth * (Math.floor(Math.log10(textArea.lineCount)) + 1) + 10
            contentY: editorFlickable.contentY
            interactive: false

            Column {
                anchors.fill: parent
                topPadding: textArea.topPadding

                Repeater {
                    id: repeatedLineNumbers
                    // TODO: Bug where text wrapping shows as new line number.
                    model: textArea.lineCount

                    // This Item is used for each line number in the gutter.
                    delegate: Item {
                        // Calculates the height of each line in the text area.
                        height: textArea.contentHeight / textArea.lineCount
                        width: parent.width

                        required property int index

                        // Show the line number.
                        Label {
                            id: numbers
                            color: RustColors.linenumber
                            font: textArea.font
                            height: parent.height
                            horizontalAlignment: Text.AlignLeft
                            text: parent.index + 1
                            verticalAlignment: Text.AlignVCenter
                            width: parent.width - indicator.width
                        }
                        // Draw edge along the right side of the line number.
                        Rectangle {
                            id: indicator
                            anchors.left: numbers.right
                            color: RustColors.linenumber
                            height: parent.height
                            width: 1
                        }
                    }
                }
            }
        }
        Flickable {
            id: editorFlickable
            Layout.fillHeight: true
            Layout.fillWidth: true
            boundsBehavior: Flickable.StopAtBounds
            height: 650

            ScrollBar.horizontal: MyScrollBar {
            }
            ScrollBar.vertical: MyScrollBar {
            }

            TextArea.flickable: TextArea {
                id: textArea
                focus: true
                persistentSelection: true
                antialiasing: true
                selectByMouse: true
                selectionColor: RustColors.editor_highlight
                selectedTextColor: RustColors.editor_highlighted_text
                textFormat: Qt.AutoText
                wrapMode: TextArea.Wrap
                text: FileSystem.readFile(root.filePath)

                onLinkActivated: function (link) {
                    Qt.openUrlExternally(link);
                }

                // TODO: Handle saving
                // Component.onCompleted: {
                //     if (Qt.application.arguments.length === 2)
                //         textDocument.source = "file:" + Qt.application.arguments[1]
                //     else
                //         textDocument.source = "qrc:/texteditor.html"
                // }
                // textDocument.onStatusChanged: {
                //     // a message lookup table using computed properties:
                //     // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Object_initializer
                //     const statusMessages = {
                //         [ TextDocument.ReadError ]: qsTr("Failed to load “%1”"),
                //         [ TextDocument.WriteError ]: qsTr("Failed to save “%1”"),
                //         [ TextDocument.NonLocalFileError ]: qsTr("Not a local file: “%1”"),
                //     }
                //     const err = statusMessages[textDocument.status]
                //     if (err) {
                //         errorDialog.text = err.arg(textDocument.source)
                //         errorDialog.open()
                //     }
                // }
            }

            FontMetrics {
                id: fontMetrics

                font: textArea.font
            }
        }
    }
    TextArea {
        id: areaConsole

        height: 100
        placeholderText: qsTr("Placeholder for bash terminal.")
        placeholderTextColor: "white"
        readOnly: true
        wrapMode: TextArea.Wrap
        background: Rectangle {
            color: RustColors.editor_background
            implicitHeight: 100
            // border.color: control.enabled ? RustColors.active : RustColors.inactive
        }
    }

    // We use an inline component to customize the horizontal and vertical
    // scroll-bars. This is convenient when the component is only used in one file.
    component MyScrollBar: ScrollBar {
        id: scrollBar

        // Scroll bar gutter
        background: Rectangle {
            implicitHeight: scrollBar.interactive ? 8 : 4
            implicitWidth: scrollBar.interactive ? 8 : 4
            color: RustColors.scrollbar_gutter

            // Fade the scrollbar gutter when inactive.
            opacity: scrollBar.active && scrollBar.size < 1.0 ? 1.0 : 0.2
            Behavior on opacity {
                OpacityAnimator {
                    duration: 500
                }
            }
        }

        // Scroll bar
        contentItem: Rectangle {
            implicitHeight: scrollBar.interactive ? 8 : 4
            implicitWidth: scrollBar.interactive ? 8 : 4

            // If we don't need a scrollbar, fallback to the gutter color.
            // If the scrollbar is required change it's color based on activity.
            color: scrollBar.size < 1.0 ? scrollBar.active ? RustColors.scrollbar_active : RustColors.scrollbar : RustColors.scrollbar_gutter
            // Smooth transition between color changes based on the state above.
            Behavior on color {
                ColorAnimation {
                    duration: 1000
                }
            }
            // Fade the scrollbar when inactive.
            opacity: scrollBar.active && scrollBar.size < 1.0 ? 1.0 : 0.35
            Behavior on opacity {
                OpacityAnimator {
                    duration: 500
                }
            }
        }
    }
}

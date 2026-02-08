// SPDX-FileCopyrightText: 2026, Shaun Reed <shaunrd0@gmail.com>
//
// SPDX-License-Identifier: GNU General Public License v3.0 or later

import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

import clide.module 1.0
import Logger 1.0

Rectangle {
    color: RustColors.editor_background

    RowLayout {
        anchors.fill: parent

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
                        required property int index

                        // Calculates the height of each line in the text area.
                        height: textArea.contentHeight / textArea.lineCount
                        width: parent.width

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
                        // Draw an edge along the right side of the line number.
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

            ScrollBar.horizontal: ClideScrollBar {
            }
            ScrollBar.vertical: ClideScrollBar {
            }
            TextArea.flickable: TextArea {
                id: textArea

                antialiasing: true
                focus: true
                persistentSelection: true
                selectByMouse: true
                selectedTextColor: RustColors.editor_highlighted_text
                selectionColor: RustColors.editor_highlight
                text: FileSystem.readFile(root.filePath)
                textFormat: Qt.AutoText
                wrapMode: TextArea.Wrap

                onLinkActivated: function (link) {
                    Qt.openUrlExternally(link);
                }
            }

            FontMetrics {
                id: fontMetrics

                font: textArea.font
            }
        }
    }
}

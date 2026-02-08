import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

import clide.module 1.0
import Logger 1.0

Rectangle {
    id: root

    property var fullPaths: []
    required property string path
    property var segments: []

    signal crumbClicked(string path)
    signal resetRoot

    function rebuildSegments(): string {
        let cleaned = path;
        if (cleaned.endsWith("/"))
            cleaned = cleaned.slice(0, -1);
        Logger.trace("Building segments for path: " + cleaned);
        segments = ["/"];
        fullPaths = ["/"];
        let parts = cleaned.split("/");
        let current = "";
        // We already pushed the root `/` path during initialization, so skip index 0.
        for (let i = 1; i < parts.length; ++i) {
            current += "/" + parts[i];
            Logger.trace("Pushing path: " + parts[i] + " Current: " + current);
            segments.push(parts[i]);
            fullPaths.push(current);
        }
        // Update the model used in the Repeater to show the new segments.
        repeater.model = segments;
    }

    color: "transparent"
    implicitHeight: breadcrumbRow.implicitHeight
    width: parent.width

    Component.onCompleted: rebuildSegments()
    onPathChanged: rebuildSegments()

    Flow {
        id: breadcrumbRow

        anchors.fill: parent
        spacing: 2
        width: parent.width

        Repeater {
            id: repeater

            model: root.segments

            delegate: Text {
                required property int index
                required property string modelData

                function getText(): string {
                    if (modelData === "/") {
                        return modelData;
                    }
                    return modelData + "/";
                }

                // Show blue underlined hyperlink text if the mouse is hovering a segment.
                color: mouseArea.containsMouse ? "#2a7fff" : RustColors.explorer_text
                font.underline: mouseArea.containsMouse
                text: getText()

                // Click events for each path segment call signal so the parent can set the file explorer root path.
                MouseArea {
                    id: mouseArea

                    anchors.fill: parent
                    hoverEnabled: true

                    onClicked: {
                        Logger.info(index + "] Breadcrumb clicked:" + root.fullPaths[index]);
                        crumbClicked(root.fullPaths[index]);
                    }
                }
            }
        }
    }
    TapHandler {
        acceptedButtons: Qt.RightButton

        onSingleTapped: contextMenu.popup()
    }
    ClideMenu {
        id: contextMenu

        ClideMenuItem {
            action: Action {
                text: qsTr("Reset root")

                onTriggered: {
                    Logger.info("Resetting root directory from ClideBreadCrumbs");
                    resetRoot();
                }
            }
        }
    }
}

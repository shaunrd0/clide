import QtQuick
import QtQuick.Controls

MenuItem {
    id: root

    background: Rectangle {
        color: root.hovered ? "#4b4f51" : "#3c3f41"  // Hover effect
        radius: 2.5
    }
    contentItem: IconLabel {
        color: "white"
        font.family: "Helvetica"
        text: root.text
    }
}

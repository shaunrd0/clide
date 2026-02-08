// SPDX-FileCopyrightText: 2026, Shaun Reed <shaunrd0@gmail.com>
//
// SPDX-License-Identifier: GNU General Public License v3.0 or later

import QtQuick
import QtQuick.Controls

import clide.module 1.0

Rectangle {
    id: root

    readonly property color currentColor: {
        if (pressed) {
            return RustColors.pressed;
        } else if (hovered) {
            return RustColors.hovered;
        } else {
            return "transparent";
        }
    }
    required property bool hovered
    required property bool pressed

    border.color: currentColor
    color: currentColor
    implicitHeight: 8
    implicitWidth: 8
    radius: 2.5
    opacity: root.hovered ? 1.0 : 0.0

    // Execute these behaviors when the color is changed.
    Behavior on color {
        ColorAnimation {
            duration: 500
        }
    }

    Behavior on opacity {
        OpacityAnimator {
            duration: 500
        }
    }
}

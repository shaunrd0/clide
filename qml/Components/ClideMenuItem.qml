// SPDX-FileCopyrightText: 2026, Shaun Reed <shaunrd0@gmail.com>
//
// SPDX-License-Identifier: GNU General Public License v3.0 or later

import QtQuick
import QtQuick.Controls.Basic

import clide.module 1.0

MenuItem {
    id: root

    background: Rectangle {
        color: root.hovered ? RustColors.hovered : RustColors.unhovered
        radius: 1.0
    }
    contentItem: IconLabel {
        color: "black"
        font.family: "Helvetica"
        text: root.text
    }
}

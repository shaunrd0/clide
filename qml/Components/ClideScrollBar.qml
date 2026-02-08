// SPDX-FileCopyrightText: 2026, Shaun Reed <shaunrd0@gmail.com>
//
// SPDX-License-Identifier: GNU General Public License v3.0 or later

import QtQuick
import QtQuick.Controls

import clide.module 1.0

ScrollBar {
    id: scrollBar

    // Height, opacitiy, width
    property int h: scrollBar.interactive ? sizeModifier * 2 : sizeModifier
    property int o: scrollBar.active && scrollBar.size < 1.0 ? 1.0 : 0.0
    property int sizeModifier: 4
    property int w: scrollBar.interactive ? sizeModifier * 2 : sizeModifier

    // Scroll bar gutter
    background: Rectangle {
        id: gutter

        color: RustColors.scrollbar_gutter
        implicitHeight: scrollBar.h
        implicitWidth: scrollBar.w

        // Fade the scrollbar gutter when inactive.
        opacity: scrollBar.o
        radius: 20

        Behavior on opacity {
            OpacityAnimator {
                duration: 500
            }
        }
    }

    // Scroll bar
    contentItem: Rectangle {
        readonly property color barColor: {
            if (scrollBar.size < 1.0) {
                // If the scrollbar is required change it's color based on activity.
                if (scrollBar.active) {
                    return RustColors.scrollbar_active;
                } else {
                    return RustColors.scrollbar;
                }
            } else {
                // If we don't need a scrollbar, fallback to the gutter color.
                return gutter.color;
            }
        }

        color: barColor
        implicitHeight: scrollBar.h
        implicitWidth: scrollBar.w

        // Fade the scrollbar when inactive.
        opacity: scrollBar.o
        radius: 20

        // Smooth transition between color changes based on the state above.
        Behavior on color {
            ColorAnimation {
                duration: 1000
            }
        }
        Behavior on opacity {
            OpacityAnimator {
                duration: 1000
            }
        }
    }
}

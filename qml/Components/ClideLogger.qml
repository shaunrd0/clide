// SPDX-FileCopyrightText: 2026, Shaun Reed <shaunrd0@gmail.com>
//
// SPDX-License-Identifier: GNU General Public License v3.0 or later

import QtQuick
import QtQuick.Controls

import clide.module 1.0
import Logger 1.0

Rectangle {
    color: RustColors.terminal_background
    radius: 10

    ListModel {
        id: model

    }
    ListView {
        id: listView

        function getLogColor(level) {
            switch (level) {
            case "INFO":
                return RustColors.info_log;
            case "DEBUG":
                return RustColors.debug_log;
            case "WARN":
                return RustColors.warn_log;
            case "ERROR":
                return RustColors.error_log;
            case "TRACE":
                return RustColors.trace_log;
            default:
                return RustColors.info_log;
            }
        }

        anchors.fill: parent
        model: model

        delegate: Text {
            color: listView.getLogColor(level)
            font.family: "monospace"
            text: `[${level}] ${message}`
        }

        onCountChanged: Qt.callLater(positionViewAtEnd)
    }
    Connections {
        function onLogged(level, message) {
            model.append({
                level,
                message
            });
        }

        target: Logger
    }
}

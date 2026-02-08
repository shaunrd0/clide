// SPDX-FileCopyrightText: 2026, Shaun Reed <shaunrd0@gmail.com>
//
// SPDX-License-Identifier: GNU General Public License v3.0 or later

pragma Singleton
import QtQuick

QtObject {
    signal logged(string level, string message)

    function debug(msg) {
        console.log(msg);
        logged("DEBUG", msg);
    }
    function error(msg) {
        console.error(msg);
        logged("ERROR", msg);
    }
    function info(msg) {
        console.log(msg);
        logged("INFO", msg);
    }
    function log(msg) {
        console.log(msg);
        logged("INFO", msg);
    }
    function trace(msg) {
        console.log(msg);
        logged("TRACE", msg);
    }
    function warn(msg) {
        console.warn(msg);
        logged("WARN", msg);
    }
}

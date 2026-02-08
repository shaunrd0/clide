// SPDX-FileCopyrightText: 2026, Shaun Reed <shaunrd0@gmail.com>
//
// SPDX-License-Identifier: GNU General Public License v3.0 or later

#[cxx_qt::bridge]

pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qcolor.h");
        type QColor = cxx_qt_lib::QColor;
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qml_singleton]
        #[qproperty(QColor, hovered)]
        #[qproperty(QColor, unhovered)]
        #[qproperty(QColor, pressed)]
        #[qproperty(QColor, menubar)]
        #[qproperty(QColor, menubar_border)]
        #[qproperty(QColor, scrollbar)]
        #[qproperty(QColor, scrollbar_active)]
        #[qproperty(QColor, scrollbar_gutter)]
        #[qproperty(QColor, linenumber)]
        #[qproperty(QColor, active)]
        #[qproperty(QColor, inactive)]
        #[qproperty(QColor, editor_background)]
        #[qproperty(QColor, editor_text)]
        #[qproperty(QColor, editor_highlighted_text)]
        #[qproperty(QColor, editor_highlight)]
        #[qproperty(QColor, gutter)]
        #[qproperty(QColor, explorer_hovered)]
        #[qproperty(QColor, explorer_text)]
        #[qproperty(QColor, explorer_text_selected)]
        #[qproperty(QColor, explorer_background)]
        #[qproperty(QColor, explorer_folder)]
        #[qproperty(QColor, explorer_folder_open)]
        #[qproperty(QColor, terminal_background)]
        #[qproperty(QColor, info_log)]
        #[qproperty(QColor, debug_log)]
        #[qproperty(QColor, warn_log)]
        #[qproperty(QColor, error_log)]
        #[qproperty(QColor, trace_log)]
        type RustColors = super::RustColorsImpl;
    }
}

use cxx_qt_lib::QColor;

pub struct RustColorsImpl {
    hovered: QColor,
    unhovered: QColor,
    pressed: QColor,
    menubar: QColor,
    menubar_border: QColor,
    scrollbar: QColor,
    scrollbar_active: QColor,
    scrollbar_gutter: QColor,
    linenumber: QColor,
    active: QColor,
    inactive: QColor,
    editor_background: QColor,
    editor_text: QColor,
    editor_highlighted_text: QColor,
    editor_highlight: QColor,
    gutter: QColor,
    explorer_hovered: QColor,
    explorer_text: QColor,
    explorer_text_selected: QColor,
    explorer_background: QColor,
    explorer_folder: QColor,
    explorer_folder_open: QColor,
    terminal_background: QColor,
    info_log: QColor,
    debug_log: QColor,
    warn_log: QColor,
    error_log: QColor,
    trace_log: QColor,
}

impl Default for RustColorsImpl {
    fn default() -> Self {
        Self {
            hovered: QColor::try_from("#303234").unwrap(),
            unhovered: QColor::try_from("#3c3f41").unwrap(),
            pressed: QColor::try_from("#4b4f51").unwrap(),
            menubar: QColor::try_from("#262626").unwrap(),
            menubar_border: QColor::try_from("#575757").unwrap(),
            scrollbar: QColor::try_from("#4b4f51").unwrap(),
            scrollbar_active: QColor::try_from("#4b4f51").unwrap(),
            scrollbar_gutter: QColor::try_from("#3b3b3b").unwrap(),
            linenumber: QColor::try_from("#94989b").unwrap(),
            active: QColor::try_from("#a9acb0").unwrap(),
            inactive: QColor::try_from("#FFF").unwrap(),
            editor_background: QColor::try_from("#1E1F22").unwrap(),
            editor_text: QColor::try_from("#acaea3").unwrap(),
            editor_highlighted_text: QColor::try_from("#ccced3").unwrap(),
            editor_highlight: QColor::try_from("#ccced3").unwrap(),
            gutter: QColor::try_from("#1e1f22").unwrap(),
            explorer_hovered: QColor::try_from("#4c5053").unwrap(),
            explorer_text: QColor::try_from("#FFF").unwrap(),
            explorer_text_selected: QColor::try_from("#262626").unwrap(),
            explorer_background: QColor::try_from("#1E1F22").unwrap(),
            explorer_folder: QColor::try_from("#54585b").unwrap(),
            explorer_folder_open: QColor::try_from("#393B40").unwrap(),
            terminal_background: QColor::try_from("#111111").unwrap(),
            info_log: QColor::try_from("#C4FFFF").unwrap(),
            debug_log: QColor::try_from("#9148AF").unwrap(),
            warn_log: QColor::try_from("#C4A958").unwrap(),
            error_log: QColor::try_from("#ff5555").unwrap(),
            trace_log: QColor::try_from("#ffaa00").unwrap(),
        }
    }
}

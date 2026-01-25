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
}

impl Default for RustColorsImpl {
    fn default() -> Self {
        Self {
            hovered: QColor::try_from("#303234").unwrap(),
            unhovered: QColor::try_from("#3c3f41").unwrap(),
            pressed: QColor::try_from("#4b4f51").unwrap(),
            menubar: QColor::try_from("#3c3f41").unwrap(),
            menubar_border: QColor::try_from("#575757").unwrap(),
            scrollbar: QColor::try_from("#4b4f51").unwrap(),
            scrollbar_active: QColor::try_from("#4b4f51").unwrap(),
            scrollbar_gutter: QColor::try_from("#3b3b3b").unwrap(),
            linenumber: QColor::try_from("#94989b").unwrap(),
            active: QColor::try_from("#a9acb0").unwrap(),
            inactive: QColor::try_from("#FFF").unwrap(),
            editor_background: QColor::try_from("#2b2b2b").unwrap(),
            editor_text: QColor::try_from("#acaea3").unwrap(),
            editor_highlighted_text: QColor::try_from("#ccced3").unwrap(),
            editor_highlight: QColor::try_from("#ccced3").unwrap(),
            gutter: QColor::try_from("#1e1f22").unwrap(),
            explorer_hovered: QColor::try_from("#4c5053").unwrap(),
            explorer_text: QColor::try_from("#3b3b3b").unwrap(),
            explorer_text_selected: QColor::try_from("#8b8b8b").unwrap(),
            explorer_background: QColor::try_from("#676c70").unwrap(),
            explorer_folder: QColor::try_from("#54585b").unwrap(),
            explorer_folder_open: QColor::try_from("#FFF").unwrap(),
        }
    }
}

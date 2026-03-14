// SPDX-FileCopyrightText: 2026, Shaun Reed <shaunrd0@gmail.com>
//
// SPDX-License-Identifier: GNU General Public License v3.0 or later

/// Colors shared between the TUI and GUI for the current theme.
pub struct Colors {}
impl Colors {
    pub const HOVERED: &str = "#303234";
    pub const UNHOVERED: &str = "#3c3f41";
    pub const PRESSED: &str = "#4b4f51";
    pub const MENUBAR: &str = "#262626";
    pub const MENUBAR_BORDER: &str = "#575757";
    pub const SCROLLBAR: &str = "#4b4f51";
    pub const SCROLLBAR_ACTIVE: &str = "#4b4f51";
    pub const SCROLLBAR_GUTTER: &str = "#3b3b3b";
    pub const LINENUMBER: &str = "#94989b";
    pub const ACTIVE: &str = "#d1d33f";
    pub const INACTIVE: &str = "#FFF";
    pub const EDITOR_BACKGROUND: &str = "#1E1F22";
    pub const EDITOR_TEXT: &str = "#acaea3";
    pub const EDITOR_HIGHLIGHTED_TEXT: &str = "#ccced3";
    pub const EDITOR_HIGHLIGHT: &str = "#ccced3";
    pub const GUTTER: &str = "#1e1f22";
    pub const EXPLORER_HOVERED: &str = "#4c5053";
    pub const EXPLORER_TEXT: &str = "#FFF";
    pub const EXPLORER_TEXT_SELECTED: &str = "#262626";
    pub const EXPLORER_BACKGROUND: &str = "#1E1F22";
    pub const EXPLORER_FOLDER: &str = "#54585b";
    pub const EXPLORER_FOLDER_OPEN: &str = "#393B40";
    pub const TERMINAL_BACKGROUND: &str = "#111111";
    pub const INFO_LOG: &str = "#C4FFFF";
    pub const DEBUG_LOG: &str = "#9148AF";
    pub const WARN_LOG: &str = "#C4A958";
    pub const ERROR_LOG: &str = "#ff5555";
    pub const TRACE_LOG: &str = "#ffaa00";

    /// Converts a CSS hex color string (e.g., "#RRGGBB" or "#RGB") to u32 in 0x00RRGGBB format.
    pub fn css_to_u32(css: &str) -> u32 {
        let hex = css.trim_start_matches('#');
        // Expand shorthand #RGB to #RRGGBB
        let hex_full = match hex.len() {
            3 => hex
                .chars()
                .map(|c| std::iter::repeat_n(c, 2).collect::<String>())
                .collect::<String>(),
            6 => hex.to_string(),
            _ => panic!("Invalid hex color length: {hex:?}"),
        };
        // Parse the hex string as u32, masking to ensure the top (alpha) byte is 0x00.
        u32::from_str_radix(&hex_full, 16)
            .map(|rgb| rgb & 0x00FF_FFFF)
            .unwrap_or_else(|e| panic!("Failed to parse hex: {e:?}"))
    }
}

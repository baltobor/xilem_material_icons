//! Material Symbols icons for xilem/masonry applications.
//!
//! This crate bundles the Material Symbols Outlined font from Google and provides
//! icon codepoint constants for use in xilem applications.
//!
//! # Font License
//!
//! The Material Symbols font is copyright Google LLC and licensed under the
//! Apache License 2.0.
//!
//! - Repository: <https://github.com/google/material-design-icons>
//! - License: <https://github.com/google/material-design-icons/blob/master/LICENSE>
//! - Font source: <https://github.com/google/material-design-icons/raw/master/variablefont/MaterialSymbolsOutlined%5BFILL%2CGRAD%2Copsz%2Cwght%5D.ttf>
//!
//! # Usage
//!
//! ```rust,ignore
//! use xilem_material_icons::{FONT_DATA, FONT_FAMILY, icons};
//!
//! // Register the font when creating your xilem app
//! let app = Xilem::new(state, logic)
//!     .with_font(FONT_DATA);
//!
//! // Use icons in labels
//! label(icons::FOLDER)
//!     .font(FONT_FAMILY)
//!     .text_size(24.0)
//! ```

/// The Material Symbols Outlined font data (TTF format).
///
/// Pass this to `Xilem::with_font()` to register the font.
pub const FONT_DATA: &[u8] = include_bytes!("../assets/MaterialSymbolsOutlined.ttf");

/// The font family name to use with labels.
pub const FONT_FAMILY: &str = "Material Symbols Outlined";

/// Standard icon sizes.
pub const ICON_SIZE_SM: f32 = 16.0;
pub const ICON_SIZE_MD: f32 = 20.0;
pub const ICON_SIZE_LG: f32 = 24.0;
pub const ICON_SIZE_XL: f32 = 32.0;

/// Material Symbols icon codepoints.
///
/// Each constant is a Unicode codepoint string that renders as an icon
/// when displayed with the Material Symbols font.
pub mod icons {
    // Navigation
    pub const CHEVRON_RIGHT: &str = "\u{e5cc}";
    pub const CHEVRON_LEFT: &str = "\u{e5cb}";
    pub const EXPAND_MORE: &str = "\u{e5cf}";
    pub const EXPAND_LESS: &str = "\u{e5ce}";
    pub const ARROW_DROP_DOWN: &str = "\u{e5c5}";
    pub const ARROW_DROP_UP: &str = "\u{e5c7}";
    pub const ARROW_UPWARD: &str = "\u{e5d8}";
    pub const ARROW_DOWNWARD: &str = "\u{e5db}";
    pub const ARROW_FORWARD: &str = "\u{e5c8}";
    pub const ARROW_BACK: &str = "\u{e5c4}";
    pub const MENU: &str = "\u{e5d2}";
    pub const MORE_VERT: &str = "\u{e5d4}";
    pub const MORE_HORIZ: &str = "\u{e5d3}";

    // Files and folders
    pub const FOLDER: &str = "\u{e2c7}";
    pub const FOLDER_OPEN: &str = "\u{e2c8}";
    pub const DESCRIPTION: &str = "\u{e873}";
    pub const INSERT_DRIVE_FILE: &str = "\u{e24d}";
    pub const FILE_COPY: &str = "\u{e173}";
    pub const CREATE_NEW_FOLDER: &str = "\u{e2cc}";
    pub const UPLOAD_FILE: &str = "\u{e9fc}";
    pub const DOWNLOAD: &str = "\u{f090}";
    pub const ATTACH_FILE: &str = "\u{e226}";

    // Actions
    pub const CHECK: &str = "\u{e5ca}";
    pub const CLOSE: &str = "\u{e5cd}";
    pub const ADD: &str = "\u{e145}";
    pub const REMOVE: &str = "\u{e15b}";
    pub const DELETE: &str = "\u{e872}";
    pub const EDIT: &str = "\u{e3c9}";
    pub const SAVE: &str = "\u{e161}";
    pub const SEARCH: &str = "\u{e8b6}";
    pub const REFRESH: &str = "\u{e5d5}";
    pub const SETTINGS: &str = "\u{e8b8}";
    pub const UNDO: &str = "\u{e166}";
    pub const REDO: &str = "\u{e15a}";
    pub const COPY: &str = "\u{e14d}";
    pub const PASTE: &str = "\u{e14f}";
    pub const CUT: &str = "\u{e14e}";

    // Status and alerts
    pub const ERROR: &str = "\u{e000}";
    pub const WARNING: &str = "\u{e002}";
    pub const INFO: &str = "\u{e88e}";
    pub const CHECK_CIRCLE: &str = "\u{e86c}";
    pub const HELP: &str = "\u{e887}";
    pub const CANCEL: &str = "\u{e5c9}";

    // Sort and filter
    pub const SORT: &str = "\u{e164}";
    pub const UNFOLD_MORE: &str = "\u{e5d7}";
    pub const FILTER_LIST: &str = "\u{e152}";

    // Visibility
    pub const VISIBILITY: &str = "\u{e8f4}";
    pub const VISIBILITY_OFF: &str = "\u{e8f5}";

    // Selection
    pub const CHECK_BOX: &str = "\u{e834}";
    pub const CHECK_BOX_OUTLINE_BLANK: &str = "\u{e835}";
    pub const RADIO_BUTTON_CHECKED: &str = "\u{e837}";
    pub const RADIO_BUTTON_UNCHECKED: &str = "\u{e836}";
    pub const INDETERMINATE_CHECK_BOX: &str = "\u{e909}";

    // Media
    pub const PLAY_ARROW: &str = "\u{e037}";
    pub const PAUSE: &str = "\u{e034}";
    pub const STOP: &str = "\u{e047}";
    pub const SKIP_NEXT: &str = "\u{e044}";
    pub const SKIP_PREVIOUS: &str = "\u{e045}";
    pub const VOLUME_UP: &str = "\u{e050}";
    pub const VOLUME_DOWN: &str = "\u{e04d}";
    pub const VOLUME_MUTE: &str = "\u{e04e}";
    pub const VOLUME_OFF: &str = "\u{e04f}";

    // Communication
    pub const EMAIL: &str = "\u{e0be}";
    pub const CHAT: &str = "\u{e0b7}";
    pub const NOTIFICATIONS: &str = "\u{e7f4}";
    pub const PERSON: &str = "\u{e7fd}";
    pub const GROUP: &str = "\u{e7ef}";

    // Content
    pub const CONTENT_COPY: &str = "\u{e14d}";
    pub const CONTENT_PASTE: &str = "\u{e14f}";
    pub const CONTENT_CUT: &str = "\u{e14e}";
    pub const LINK: &str = "\u{e157}";
    pub const LINK_OFF: &str = "\u{e16f}";

    // Device
    pub const SMARTPHONE: &str = "\u{e32c}";
    pub const COMPUTER: &str = "\u{e30a}";
    pub const KEYBOARD: &str = "\u{e312}";
    pub const MOUSE: &str = "\u{e323}";

    // Image
    pub const IMAGE: &str = "\u{e3f4}";
    pub const PHOTO: &str = "\u{e410}";
    pub const CAMERA: &str = "\u{e3af}";
    pub const PALETTE: &str = "\u{e40a}";

    // Places
    pub const HOME: &str = "\u{e88a}";
    pub const WORK: &str = "\u{e8f9}";
    pub const PLACE: &str = "\u{e55f}";
    pub const MAP: &str = "\u{e55b}";

    // Social
    pub const FAVORITE: &str = "\u{e87d}";
    pub const FAVORITE_BORDER: &str = "\u{e87e}";
    pub const STAR: &str = "\u{e838}";
    pub const STAR_BORDER: &str = "\u{e83a}";
    pub const THUMB_UP: &str = "\u{e8dc}";
    pub const THUMB_DOWN: &str = "\u{e8db}";
    pub const SHARE: &str = "\u{e80d}";

    // Toggle
    pub const TOGGLE_ON: &str = "\u{e9f6}";
    pub const TOGGLE_OFF: &str = "\u{e9f5}";

    // Time
    pub const SCHEDULE: &str = "\u{e8b5}";
    pub const TIMER: &str = "\u{e425}";
    pub const ALARM: &str = "\u{e855}";
    pub const CALENDAR_TODAY: &str = "\u{e935}";
    pub const EVENT: &str = "\u{e878}";

    // Editor
    pub const FORMAT_BOLD: &str = "\u{e238}";
    pub const FORMAT_ITALIC: &str = "\u{e23f}";
    pub const FORMAT_UNDERLINE: &str = "\u{e249}";
    pub const FORMAT_LIST_BULLETED: &str = "\u{e241}";
    pub const FORMAT_LIST_NUMBERED: &str = "\u{e242}";
    pub const FORMAT_ALIGN_LEFT: &str = "\u{e236}";
    pub const FORMAT_ALIGN_CENTER: &str = "\u{e234}";
    pub const FORMAT_ALIGN_RIGHT: &str = "\u{e237}";

    // Active mobility (from xilem_extras)
    pub const PEDAL_BIKE: &str = "\u{eb29}";
    pub const DIRECTIONS_WALK: &str = "\u{e536}";
    pub const DIRECTIONS_RUN: &str = "\u{e566}";
    pub const PARK: &str = "\u{ea63}";
    pub const NATURE: &str = "\u{e406}";
    pub const SENTIMENT_SATISFIED: &str = "\u{e813}";

    // Code and development
    pub const CODE: &str = "\u{e86f}";
    pub const BUG_REPORT: &str = "\u{e868}";
    pub const BUILD: &str = "\u{e869}";
    pub const TERMINAL: &str = "\u{eb8e}";

    // Window controls
    pub const FULLSCREEN: &str = "\u{e5d0}";
    pub const FULLSCREEN_EXIT: &str = "\u{e5d1}";
    pub const OPEN_IN_NEW: &str = "\u{e89e}";
    pub const LAUNCH: &str = "\u{e895}";
    pub const MINIMIZE: &str = "\u{e931}";
    pub const MAXIMIZE: &str = "\u{e930}";

    // Lock
    pub const LOCK: &str = "\u{e897}";
    pub const LOCK_OPEN: &str = "\u{e898}";

    // Sync
    pub const SYNC: &str = "\u{e627}";
    pub const SYNC_DISABLED: &str = "\u{e628}";
    pub const SYNC_PROBLEM: &str = "\u{e629}";
    pub const CLOUD: &str = "\u{e2bd}";
    pub const CLOUD_UPLOAD: &str = "\u{e2c3}";
    pub const CLOUD_DOWNLOAD: &str = "\u{e2c0}";
    pub const CLOUD_OFF: &str = "\u{e2c1}";

    // Zoom
    pub const ZOOM_IN: &str = "\u{e8ff}";
    pub const ZOOM_OUT: &str = "\u{e900}";
    pub const FIT_SCREEN: &str = "\u{ea10}";

    // Misc
    pub const DRAG_HANDLE: &str = "\u{e25d}";
    pub const DRAG_INDICATOR: &str = "\u{e945}";
    pub const REORDER: &str = "\u{e8fe}";
    pub const PRINT: &str = "\u{e8ad}";
    pub const LANGUAGE: &str = "\u{e894}";
    pub const TRANSLATE: &str = "\u{e8e2}";
    pub const DARK_MODE: &str = "\u{e51c}";
    pub const LIGHT_MODE: &str = "\u{e518}";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn font_data_is_not_empty() {
        assert!(!FONT_DATA.is_empty());
    }

    #[test]
    fn font_family_is_correct() {
        assert_eq!(FONT_FAMILY, "Material Symbols Outlined");
    }

    #[test]
    fn icon_codepoints_are_valid_unicode() {
        // Test a few icons to ensure they are valid strings
        assert!(!icons::FOLDER.is_empty());
        assert!(!icons::CHECK.is_empty());
        assert!(!icons::CLOSE.is_empty());
    }
}

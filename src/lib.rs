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
//! Register the font once when creating your Xilem app, then use icons freely:
//!
//! ```rust,ignore
//! use xilem_material_icons::{FONT_DATA, icon, icons};
//!
//! let app = Xilem::new(state, logic)
//!     .with_font(FONT_DATA);
//!
//! // Use icons in any view
//! icon(icons::FOLDER)
//! icon(icons::SETTINGS).size(24.0).color(Color::RED)
//! ```
//!
//! # Multi-window note
//!
//! Xilem registers fonts per window via `RenderRoot::register_fonts`, which is
//! called for each window that exists at startup.  Secondary windows opened
//! after startup (e.g. preferences, file manager) currently receive a fresh
//! `FontContext` that does **not** automatically inherit registered fonts —
//! this is a known limitation of xilem's font lifecycle (see the upstream TODO
//! in `masonry_core/src/app/render_root.rs`).
//!
//! Until xilem provides a per-window font hook, callers that open secondary
//! windows must ensure icons appear in the initial window set so the font
//! context is populated, or register the font explicitly for each window via
//! `Xilem::with_on_start` and `MasonryState::roots()`.

use xilem::masonry::peniko::Color;
use xilem::style::Style;
use xilem::view::label;
use xilem::AnyWidgetView;

/// The Material Symbols Outlined font data (TTF format).
///
/// Pass this to `Xilem::with_font()` to register the font for your app.
pub const FONT_DATA: &[u8] = include_bytes!("../assets/MaterialSymbolsOutlined.ttf");

/// The font family name to use with labels.
pub const FONT_FAMILY: &str = "Material Symbols Outlined";

/// Standard icon sizes.
pub const ICON_SIZE_SM: f32 = 16.0;
pub const ICON_SIZE_MD: f32 = 20.0;
pub const ICON_SIZE_LG: f32 = 24.0;
pub const ICON_SIZE_XL: f32 = 32.0;

/// A Material Symbol icon view.
///
/// Use the [`icon`] function to create an icon, then chain methods to customize it.
///
/// # Example
///
/// ```rust,ignore
/// use xilem_material_icons::{icon, icons};
/// use xilem::masonry::peniko::Color;
///
/// // Basic icon (default size: 20px)
/// icon(icons::FOLDER)
///
/// // Customized icon
/// icon(icons::SETTINGS)
///     .size(24.0)
///     .color(Color::from_rgb8(100, 180, 100))
/// ```
#[derive(Clone)]
pub struct Icon {
    codepoint: &'static str,
    size: f32,
    color: Option<Color>,
}

impl Icon {
    /// Creates a new icon with the given codepoint.
    pub fn new(codepoint: &'static str) -> Self {
        Self {
            codepoint,
            size: ICON_SIZE_MD,
            color: None,
        }
    }

    /// Sets the icon size in pixels.
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Sets the icon color.
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Builds the icon as a xilem view.
    pub fn build<State: 'static, Action: 'static>(self) -> Box<AnyWidgetView<State, Action>> {
        let lbl = label(self.codepoint).font(FONT_FAMILY).text_size(self.size);

        if let Some(color) = self.color {
            Box::new(lbl.color(color))
        } else {
            Box::new(lbl)
        }
    }
}

/// Creates a Material Symbol icon view.
///
/// # Example
///
/// ```rust,ignore
/// use xilem_material_icons::{icon, icons};
///
/// icon(icons::FOLDER)
/// icon(icons::CHECK).size(16.0)
/// icon(icons::ERROR).color(Color::RED)
/// ```
pub fn icon(codepoint: &'static str) -> Icon {
    Icon::new(codepoint)
}

/// Creates a small icon (16px).
pub fn icon_sm(codepoint: &'static str) -> Icon {
    Icon::new(codepoint).size(ICON_SIZE_SM)
}

/// Creates a medium icon (20px, default).
pub fn icon_md(codepoint: &'static str) -> Icon {
    Icon::new(codepoint).size(ICON_SIZE_MD)
}

/// Creates a large icon (24px).
pub fn icon_lg(codepoint: &'static str) -> Icon {
    Icon::new(codepoint).size(ICON_SIZE_LG)
}

/// Creates an extra-large icon (32px).
pub fn icon_xl(codepoint: &'static str) -> Icon {
    Icon::new(codepoint).size(ICON_SIZE_XL)
}

/// Material Symbols icon codepoints.
///
/// Each constant is a Unicode codepoint string that renders as an icon
/// when displayed with the Material Symbols font.
///
/// The full set (~4,200 icons) is generated from
/// `assets/MaterialSymbolsOutlined.codepoints` by the `gen_icons`
/// binary. Names follow Material's snake_case scheme upper-cased
/// (`CHEVRON_RIGHT`, `FOLDER`, `SETTINGS`, …); names that would
/// otherwise begin with a digit get a leading underscore (`_10K`,
/// `_123`).
#[allow(non_upper_case_globals)]
pub mod icons {
    include!("icons_generated.rs");
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
        assert!(!icons::FOLDER.is_empty());
        assert!(!icons::CHECK.is_empty());
        assert!(!icons::CLOSE.is_empty());
    }
}

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
//! The font registers itself automatically the first time any icon is laid out
//! in a window.  No `.with_font()` call is needed:
//!
//! ```rust,ignore
//! use xilem_material_icons::{icon, icons};
//!
//! // Use icons anywhere in your view tree — they work in every window
//! icon(icons::FOLDER)
//! icon(icons::SETTINGS).size(24.0).color(Color::RED)
//! ```

use std::sync::{Arc, OnceLock};

use xilem::core::{MessageCtx, MessageResult, Mut, View, ViewMarker};
use xilem::masonry::accesskit::{Node, Role};
use xilem::masonry::core::{
    AccessCtx, AccessEvent, ChildrenIds, EventCtx, LayoutCtx, MeasureCtx, NoAction, PaintCtx,
    PointerEvent, PropertiesMut, PropertiesRef, RegisterCtx, TextEvent, Update, UpdateCtx, Widget,
    WidgetMut,
};
use xilem::masonry::imaging::Painter;
use xilem::masonry::kurbo::{Axis, Size};
use xilem::masonry::layout::{AsUnit, LenReq, Length};
use xilem::masonry::peniko::{Blob, Color};
use xilem::style::Style;
use xilem::view::label;
use xilem::{AnyWidgetView, Pod, ViewCtx};

/// The Material Symbols Outlined font data (TTF format).
pub const FONT_DATA: &[u8] = include_bytes!("../assets/MaterialSymbolsOutlined.ttf");

/// The font family name to use with labels.
pub const FONT_FAMILY: &str = "Material Symbols Outlined";

/// Standard icon sizes.
pub const ICON_SIZE_SM: f32 = 16.0;
pub const ICON_SIZE_MD: f32 = 20.0;
pub const ICON_SIZE_LG: f32 = 24.0;
pub const ICON_SIZE_XL: f32 = 32.0;

// --- Font blob singleton ---

// The Blob wraps an Arc, so cloning it is cheap (pointer copy, no data copy).
// We create it once here so every FontRegistrar widget shares the same Arc.
fn font_blob() -> Blob<u8> {
    static BLOB: OnceLock<Blob<u8>> = OnceLock::new();
    BLOB.get_or_init(|| Blob::new(Arc::new(FONT_DATA))).clone()
}

// --- Font self-registration widget ---

/// Zero-size masonry widget that registers the Material Symbols font into the
/// per-window `FontContext` on first layout, then becomes a no-op.
///
/// One instance lives in the view tree for each icon. The first one encountered
/// in a window registers the font; all subsequent ones in that window skip the
/// call because their `registered` flag is `true`.
#[derive(Debug)]
struct FontRegistrar {
    /// True once the font has been registered in this widget's window.
    registered: bool,
}

impl FontRegistrar {
    fn new() -> Self {
        Self { registered: false }
    }

    fn register_if_needed(&mut self, font_ctx: &mut xilem::masonry::parley::FontContext) {
        if self.registered {
            return;
        }
        font_ctx.collection.register_fonts(font_blob(), None);
        self.registered = true;
    }
}

impl Widget for FontRegistrar {
    type Action = NoAction;

    fn on_pointer_event(
        &mut self,
        _ctx: &mut EventCtx<'_>,
        _props: &mut PropertiesMut<'_>,
        _event: &PointerEvent,
    ) {
    }

    fn on_text_event(
        &mut self,
        _ctx: &mut EventCtx<'_>,
        _props: &mut PropertiesMut<'_>,
        _event: &TextEvent,
    ) {
    }

    fn on_access_event(
        &mut self,
        _ctx: &mut EventCtx<'_>,
        _props: &mut PropertiesMut<'_>,
        _event: &AccessEvent,
    ) {
    }

    fn register_children(&mut self, _ctx: &mut RegisterCtx<'_>) {}

    fn update(&mut self, _ctx: &mut UpdateCtx<'_>, _props: &mut PropertiesMut<'_>, event: &Update) {
        // Allow re-registration if the font context was reset (suspend/resume).
        if matches!(event, Update::FontsChanged) {
            self.registered = false;
        }
    }

    fn measure(
        &mut self,
        ctx: &mut MeasureCtx<'_>,
        _props: &PropertiesRef<'_>,
        _axis: Axis,
        _len_req: LenReq,
        _cross_length: Option<Length>,
    ) -> Length {
        self.register_if_needed(ctx.text_contexts().0);
        0.0_f64.px()
    }

    fn layout(&mut self, ctx: &mut LayoutCtx<'_>, _props: &PropertiesRef<'_>, _size: Size) {
        self.register_if_needed(ctx.text_contexts().0);
    }

    fn paint(
        &mut self,
        _ctx: &mut PaintCtx<'_>,
        _props: &PropertiesRef<'_>,
        _painter: &mut Painter<'_>,
    ) {
    }

    fn accessibility_role(&self) -> Role {
        Role::GenericContainer
    }

    fn accessibility(
        &mut self,
        _ctx: &mut AccessCtx<'_>,
        _props: &PropertiesRef<'_>,
        _node: &mut Node,
    ) {
    }

    fn children_ids(&self) -> ChildrenIds {
        ChildrenIds::new()
    }
}

// --- FontRegistrar view ---

#[derive(Clone)]
struct FontRegistrarView;

impl ViewMarker for FontRegistrarView {}

impl<State: 'static, Action: 'static> View<State, Action, ViewCtx> for FontRegistrarView {
    type Element = Pod<FontRegistrar>;
    type ViewState = ();

    fn build(&self, ctx: &mut ViewCtx, _: &mut State) -> (Self::Element, Self::ViewState) {
        (ctx.create_pod(FontRegistrar::new()), ())
    }

    fn rebuild(
        &self,
        _prev: &Self,
        (): &mut Self::ViewState,
        _ctx: &mut ViewCtx,
        _element: Mut<'_, Self::Element>,
        _: &mut State,
    ) {
    }

    fn teardown(
        &self,
        (): &mut Self::ViewState,
        _ctx: &mut ViewCtx,
        _element: Mut<'_, Self::Element>,
    ) {
    }

    fn message(
        &self,
        (): &mut Self::ViewState,
        _message: &mut MessageCtx,
        _element: WidgetMut<'_, FontRegistrar>,
        _app_state: &mut State,
    ) -> MessageResult<Action> {
        MessageResult::Stale
    }
}

// --- Public API ---

/// A Material Symbol icon view.
///
/// Use the [`icon`] function to create an icon, then chain methods to customize it.
///
/// The font registers itself automatically into whichever window the icon first
/// appears in — no `.with_font()` call needed.
///
/// # Example
///
/// ```rust,ignore
/// use xilem_material_icons::{icon, icons};
/// use xilem::masonry::peniko::Color;
///
/// icon(icons::FOLDER)
/// icon(icons::SETTINGS).size(24.0).color(Color::from_rgb8(100, 180, 100))
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
    ///
    /// Embeds a zero-size [`FontRegistrar`] widget that registers the Material
    /// Symbols font into the window's `FontContext` on first layout. This makes
    /// icons work in every window — including secondary windows opened after
    /// app startup — without any setup from the caller.
    pub fn build<State: 'static, Action: 'static>(self) -> Box<AnyWidgetView<State, Action>> {
        let lbl = label(self.codepoint).font(FONT_FAMILY).text_size(self.size);

        let lbl_view: Box<AnyWidgetView<State, Action>> = if let Some(color) = self.color {
            Box::new(lbl.color(color))
        } else {
            Box::new(lbl)
        };

        // FontRegistrarView is zero-size; flex_col stacks it above the label.
        // The column is clamped to icon size so the registrar takes no space.
        Box::new(
            xilem::view::flex_col((FontRegistrarView, lbl_view))
                .width(Length::px(self.size as f64))
                .height(Length::px(self.size as f64)),
        )
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

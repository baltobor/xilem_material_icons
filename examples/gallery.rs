//! Icon Gallery - demonstrates all available Material Symbols icons.

use xilem::view::{button, flex_col, flex_row, label, portal, CrossAxisAlignment};
use xilem::style::Style;
use xilem::masonry::vello::peniko::Color;
use xilem::masonry::layout::AsUnit;
use xilem::{EventLoop, WidgetView, WindowOptions, Xilem};

use xilem_material_icons::{FONT_DATA, FONT_FAMILY, ICON_SIZE_LG, icons};

const TEXT_COLOR: Color = Color::from_rgb8(220, 218, 214);
const TEXT_SECONDARY: Color = Color::from_rgb8(160, 156, 150);
const BG_DARK: Color = Color::from_rgb8(30, 28, 26);
const BG_SECTION: Color = Color::from_rgb8(40, 38, 36);
const ACCENT: Color = Color::from_rgb8(100, 180, 100);

#[derive(Default)]
struct AppState {
    selected_icon: Option<&'static str>,
}

fn icon_button(
    icon: &'static str,
    name: &'static str,
    selected: Option<&'static str>,
) -> impl WidgetView<AppState> {
    let is_selected = selected == Some(icon);
    let bg = if is_selected { ACCENT } else { Color::TRANSPARENT };

    button(
        flex_col((
            label(icon)
                .font(FONT_FAMILY)
                .text_size(ICON_SIZE_LG)
                .color(if is_selected { BG_DARK } else { TEXT_COLOR }),
            label(name)
                .text_size(9.0)
                .color(if is_selected { BG_DARK } else { TEXT_SECONDARY }),
        ))
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .gap(4.px())
        .padding(8.0),
        move |state: &mut AppState| {
            state.selected_icon = Some(icon);
        },
    )
    .background_color(bg)
}

fn section<'a>(
    title: &'a str,
    icons: Vec<(&'static str, &'static str)>,
    selected: Option<&'static str>,
) -> impl WidgetView<AppState> + use<'a> {
    let icon_buttons: Vec<_> = icons
        .into_iter()
        .map(|(icon, name)| icon_button(icon, name, selected).boxed())
        .collect();

    flex_col((
        label(title.to_string())
            .text_size(14.0)
            .weight(xilem::FontWeight::BOLD)
            .color(ACCENT),
        flex_row(icon_buttons)
            .gap(4.px()),
    ))
    .cross_axis_alignment(CrossAxisAlignment::Start)
    .gap(8.px())
    .padding(12.0)
    .background_color(BG_SECTION)
}

fn app_logic(state: &mut AppState) -> impl WidgetView<AppState> {
    let selected = state.selected_icon;

    flex_col((
        // Header (fixed)
        flex_row((
            label(icons::PALETTE)
                .font(FONT_FAMILY)
                .text_size(ICON_SIZE_LG)
                .color(ACCENT),
            label("Material Icons Gallery")
                .text_size(20.0)
                .weight(xilem::FontWeight::BOLD)
                .color(TEXT_COLOR),
        ))
        .gap(12.px())
        .padding(16.0),

        // Scrollable content (takes remaining space)
        portal(
            flex_col((
                // Navigation
                section("Navigation", vec![
                    (icons::CHEVRON_RIGHT, "chevron_right"),
                    (icons::CHEVRON_LEFT, "chevron_left"),
                    (icons::EXPAND_MORE, "expand_more"),
                    (icons::EXPAND_LESS, "expand_less"),
                    (icons::ARROW_BACK, "arrow_back"),
                    (icons::ARROW_FORWARD, "arrow_forward"),
                    (icons::MENU, "menu"),
                    (icons::MORE_VERT, "more_vert"),
                ], selected),

                // Files
                section("Files", vec![
                    (icons::FOLDER, "folder"),
                    (icons::FOLDER_OPEN, "folder_open"),
                    (icons::DESCRIPTION, "description"),
                    (icons::INSERT_DRIVE_FILE, "file"),
                    (icons::DOWNLOAD, "download"),
                    (icons::UPLOAD_FILE, "upload"),
                    (icons::ATTACH_FILE, "attach"),
                ], selected),

                // Actions
                section("Actions", vec![
                    (icons::CHECK, "check"),
                    (icons::CLOSE, "close"),
                    (icons::ADD, "add"),
                    (icons::REMOVE, "remove"),
                    (icons::DELETE, "delete"),
                    (icons::EDIT, "edit"),
                    (icons::SAVE, "save"),
                    (icons::SEARCH, "search"),
                    (icons::REFRESH, "refresh"),
                    (icons::SETTINGS, "settings"),
                ], selected),

                // Status
                section("Status", vec![
                    (icons::ERROR, "error"),
                    (icons::WARNING, "warning"),
                    (icons::INFO, "info"),
                    (icons::CHECK_CIRCLE, "check_circle"),
                    (icons::HELP, "help"),
                    (icons::CANCEL, "cancel"),
                ], selected),

                // Selection
                section("Selection", vec![
                    (icons::CHECK_BOX, "checkbox"),
                    (icons::CHECK_BOX_OUTLINE_BLANK, "checkbox_blank"),
                    (icons::RADIO_BUTTON_CHECKED, "radio_on"),
                    (icons::RADIO_BUTTON_UNCHECKED, "radio_off"),
                ], selected),

                // Media
                section("Media", vec![
                    (icons::PLAY_ARROW, "play"),
                    (icons::PAUSE, "pause"),
                    (icons::STOP, "stop"),
                    (icons::SKIP_NEXT, "next"),
                    (icons::SKIP_PREVIOUS, "prev"),
                    (icons::VOLUME_UP, "vol_up"),
                    (icons::VOLUME_DOWN, "vol_down"),
                    (icons::VOLUME_MUTE, "mute"),
                ], selected),

                // Social
                section("Social", vec![
                    (icons::FAVORITE, "favorite"),
                    (icons::FAVORITE_BORDER, "fav_border"),
                    (icons::STAR, "star"),
                    (icons::STAR_BORDER, "star_border"),
                    (icons::THUMB_UP, "thumb_up"),
                    (icons::THUMB_DOWN, "thumb_down"),
                    (icons::SHARE, "share"),
                ], selected),

                // Misc
                section("Misc", vec![
                    (icons::HOME, "home"),
                    (icons::PERSON, "person"),
                    (icons::GROUP, "group"),
                    (icons::LOCK, "lock"),
                    (icons::LOCK_OPEN, "unlock"),
                    (icons::DARK_MODE, "dark"),
                    (icons::LIGHT_MODE, "light"),
                    (icons::CODE, "code"),
                    (icons::TERMINAL, "terminal"),
                    (icons::PEDAL_BIKE, "bike"),
                ], selected),
            ))
            .gap(8.px())
            .padding(16.0)
        )
        .must_fill(true),

        // Footer (fixed) - shows selected icon
        flex_row((
            label("Selected:")
                .text_size(14.0)
                .color(TEXT_SECONDARY),
            if let Some(icon) = selected {
                flex_row((
                    label(icon)
                        .font(FONT_FAMILY)
                        .text_size(32.0)
                        .color(ACCENT),
                    label(format!("Unicode: {}", icon.chars().next().map(|c| format!("U+{:04X}", c as u32)).unwrap_or_default()))
                        .text_size(12.0)
                        .color(TEXT_SECONDARY),
                ))
                .cross_axis_alignment(CrossAxisAlignment::Center)
                .gap(16.px())
                .boxed()
            } else {
                label("Click an icon to select")
                    .text_size(12.0)
                    .color(TEXT_SECONDARY)
                    .boxed()
            },
        ))
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .gap(16.px())
        .padding(16.0)
        .background_color(BG_SECTION),
    ))
    .gap(0.px())
    .background_color(BG_DARK)
}

fn main() {
    let app = Xilem::new_simple(
        AppState::default(),
        app_logic,
        WindowOptions::new("Material Icons Gallery")
            .with_initial_inner_size(xilem::winit::dpi::LogicalSize::new(860.0, 700.0)),
    )
    .with_font(FONT_DATA.to_vec());

    app.run_in(EventLoop::with_user_event()).unwrap();
}

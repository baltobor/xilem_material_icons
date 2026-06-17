//! Icon Gallery — every Material Symbols icon, grouped by category.
//!
//! Loads two bundled assets at startup:
//!
//!  * `assets/MaterialSymbolsOutlined.codepoints` — the canonical
//!    `name<space>hex` list also used by the `gen_icons` binary.
//!  * `assets/MaterialSymbolsOutlined.categories` — `category\tname`
//!    pairs distilled from Google's `icons.json` metadata by
//!    `scripts/fetch_categories.pl`.
//!
//! The UI is a tab strip across the top (one tab per category, plus a
//! synthetic "All" tab listing every icon in the codepoints file), a
//! search box that filters within the active tab, and a scrolling
//! grid of clickable cells. Click a cell → footer shows the icon's
//! name and Unicode codepoint.

use std::collections::BTreeMap;
use std::sync::OnceLock;

use xilem::masonry::layout::{AsUnit, Length};
use xilem::masonry::peniko::Color;
use xilem::style::Style;
use xilem::view::{
    button, flex_col, flex_row, label, portal, text_input, CrossAxisAlignment, FlexExt,
};
use xilem::{EventLoop, WidgetView, WindowOptions, Xilem};

use xilem_material_icons::{icons, FONT_FAMILY, ICON_SIZE_LG};

const TEXT_COLOR: Color = Color::from_rgb8(220, 218, 214);
const TEXT_SECONDARY: Color = Color::from_rgb8(160, 156, 150);
const BG_DARK: Color = Color::from_rgb8(30, 28, 26);
const BG_SECTION: Color = Color::from_rgb8(40, 38, 36);
const BG_TAB_INACTIVE: Color = Color::from_rgb8(50, 48, 46);
const ACCENT: Color = Color::from_rgb8(100, 180, 100);

/// Number of cells per visible row in the grid.
const COLUMNS: usize = 12;

/// One icon: lowercase Material name + glyph as a `&'static str`
/// (single-codepoint, ready to drop into `label(...)`).
#[derive(Clone, Copy)]
struct IconEntry {
    name: &'static str,
    glyph: &'static str,
}

/// All icons keyed by name, plus the inferred category buckets and a
/// synthetic "All" bucket. Built once at startup.
struct Catalogue {
    /// Icon name → entry, alphabetical.
    by_name: BTreeMap<&'static str, IconEntry>,
    /// Category name (display order = alphabetical, "All" first) →
    /// the icon names that belong to it.
    categories: Vec<(String, Vec<&'static str>)>,
}

fn catalogue() -> &'static Catalogue {
    static CACHE: OnceLock<Catalogue> = OnceLock::new();
    CACHE.get_or_init(|| {
        // ---- Parse codepoints (every icon the font ships) ----
        let raw_cp = include_str!("../assets/MaterialSymbolsOutlined.codepoints");
        let mut by_name: BTreeMap<&'static str, IconEntry> = BTreeMap::new();
        for line in raw_cp.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let mut parts = line.split_whitespace();
            let (Some(name), Some(hex)) = (parts.next(), parts.next()) else {
                continue;
            };
            let cp = match u32::from_str_radix(hex, 16) {
                Ok(v) => v,
                Err(_) => continue,
            };
            let glyph_str: String = char::from_u32(cp).unwrap_or('?').to_string();
            let glyph: &'static str = Box::leak(glyph_str.into_boxed_str());
            let name_static: &'static str = Box::leak(name.to_string().into_boxed_str());
            by_name.insert(
                name_static,
                IconEntry {
                    name: name_static,
                    glyph,
                },
            );
        }

        // ---- Parse categories (subset; some newest icons are not
        // listed yet). Every icon also lives in the synthetic "All"
        // bucket so the user can browse the full set. ----
        let raw_cat = include_str!("../assets/MaterialSymbolsOutlined.categories");
        let mut buckets: BTreeMap<String, Vec<&'static str>> = BTreeMap::new();
        for line in raw_cat.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let mut parts = line.splitn(2, '\t');
            let (Some(cat), Some(name)) = (parts.next(), parts.next()) else {
                continue;
            };
            // Only include icons we actually have a glyph for.
            if let Some(entry) = by_name.get(name).copied() {
                buckets.entry(cat.to_string()).or_default().push(entry.name);
            }
        }
        let mut categories: Vec<(String, Vec<&'static str>)> = buckets.into_iter().collect();
        // Each bucket is name-sorted to keep tab content stable.
        for (_, names) in categories.iter_mut() {
            names.sort_unstable();
        }

        // Prepend an "All" bucket containing every icon by name.
        let all: Vec<&'static str> = by_name.keys().copied().collect();
        categories.insert(0, ("All".to_string(), all));

        Catalogue {
            by_name,
            categories,
        }
    })
}

#[derive(Default)]
struct AppState {
    /// Index into `catalogue().categories`. 0 = "All".
    active_tab: usize,
    /// Lowercase substring filter applied within the active tab.
    filter: String,
    selected_glyph: Option<&'static str>,
    selected_name: Option<&'static str>,
}

fn icon_button(entry: IconEntry, selected: Option<&'static str>) -> impl WidgetView<AppState> {
    let is_selected = selected == Some(entry.glyph);
    let bg = if is_selected {
        ACCENT
    } else {
        Color::TRANSPARENT
    };
    let glyph = entry.glyph;
    let name = entry.name;

    button(
        flex_col((
            label(glyph)
                .font(FONT_FAMILY)
                .text_size(ICON_SIZE_LG)
                .color(if is_selected { BG_DARK } else { TEXT_COLOR }),
            label(name)
                .text_size(9.0)
                .color(if is_selected { BG_DARK } else { TEXT_SECONDARY }),
        ))
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .gap(4.px())
        .padding(Length::px(6.0)),
        move |state: &mut AppState| {
            state.selected_glyph = Some(glyph);
            state.selected_name = Some(name);
        },
    )
    .background_color(bg)
}

fn tab_button(
    idx: usize,
    label_text: &str,
    count: usize,
    active: bool,
) -> impl WidgetView<AppState> + use<'_> {
    let bg = if active { ACCENT } else { BG_TAB_INACTIVE };
    let fg = if active { BG_DARK } else { TEXT_COLOR };
    button(
        flex_row((
            label(label_text.to_string())
                .text_size(12.0)
                .weight(xilem::FontWeight::BOLD)
                .color(fg),
            label(format!("{}", count))
                .text_size(11.0)
                .color(if active { BG_DARK } else { TEXT_SECONDARY }),
        ))
        .gap(6.px())
        .padding(Length::px(8.0)),
        move |state: &mut AppState| {
            state.active_tab = idx;
            state.filter.clear();
        },
    )
    .background_color(bg)
}

/// Group `entries` into rows of `COLUMNS` and render each as a
/// `flex_row`. Returned as boxed views so the outer `flex_col` accepts
/// the dynamic count.
fn icon_rows(
    entries: Vec<IconEntry>,
    selected: Option<&'static str>,
) -> Vec<Box<xilem::AnyWidgetView<AppState>>> {
    let mut rows: Vec<Box<xilem::AnyWidgetView<AppState>>> = Vec::new();
    for chunk in entries.chunks(COLUMNS) {
        let cells: Vec<_> = chunk
            .iter()
            .map(|e| icon_button(*e, selected).boxed())
            .collect();
        rows.push(
            flex_row(cells)
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .gap(4.px())
                .boxed(),
        );
    }
    rows
}

fn app_logic(state: &mut AppState) -> impl WidgetView<AppState> {
    let cat = catalogue();
    let active_tab = state.active_tab.min(cat.categories.len().saturating_sub(1));
    let (cat_name, cat_names) = &cat.categories[active_tab];

    let filter_lc = state.filter.to_ascii_lowercase();
    let entries: Vec<IconEntry> = cat_names
        .iter()
        .filter_map(|n| cat.by_name.get(n).copied())
        .filter(|e| filter_lc.is_empty() || e.name.contains(&filter_lc))
        .collect();
    let shown = entries.len();
    let total_in_tab = cat_names.len();
    let selected = state.selected_glyph;

    let header = flex_row((
        label(icons::PALETTE)
            .font(FONT_FAMILY)
            .text_size(ICON_SIZE_LG)
            .color(ACCENT),
        label("Material Icons Gallery")
            .text_size(20.0)
            .weight(xilem::FontWeight::BOLD)
            .color(TEXT_COLOR),
        label(format!("{cat_name}: {shown} / {total_in_tab}"))
            .text_size(12.0)
            .color(TEXT_SECONDARY),
    ))
    .cross_axis_alignment(CrossAxisAlignment::Center)
    .gap(12.px())
    .padding(Length::px(16.0));

    // Tab strip. Wraps via a scrolling portal so it doesn't blow the
    // window width when there are many categories.
    let tab_views: Vec<Box<xilem::AnyWidgetView<AppState>>> = cat
        .categories
        .iter()
        .enumerate()
        .map(|(i, (name, names))| tab_button(i, name, names.len(), i == active_tab).boxed())
        .collect();
    let tab_strip = portal(
        flex_row(tab_views)
            .gap(4.px())
            .padding(Length::px(8.0))
            .background_color(BG_SECTION),
    );

    let search = text_input(state.filter.clone(), |state: &mut AppState, new: String| {
        state.filter = new;
    })
    .placeholder("Search within this category…")
    .text_size(14.0);

    let rows = icon_rows(entries, selected);

    let footer = flex_row((
        label("Selected:").text_size(14.0).color(TEXT_SECONDARY),
        match (state.selected_glyph, state.selected_name) {
            (Some(glyph), Some(name)) => flex_row((
                label(glyph).font(FONT_FAMILY).text_size(32.0).color(ACCENT),
                flex_col((
                    label(name).text_size(13.0).color(TEXT_COLOR),
                    label(format!(
                        "Unicode: {}",
                        glyph
                            .chars()
                            .next()
                            .map(|c| format!("U+{:04X}", c as u32))
                            .unwrap_or_default()
                    ))
                    .text_size(11.0)
                    .color(TEXT_SECONDARY),
                ))
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .gap(2.px()),
            ))
            .cross_axis_alignment(CrossAxisAlignment::Center)
            .gap(16.px())
            .boxed(),
            _ => label("Click an icon to select")
                .text_size(12.0)
                .color(TEXT_SECONDARY)
                .boxed(),
        },
    ))
    .cross_axis_alignment(CrossAxisAlignment::Center)
    .gap(16.px())
    .padding(Length::px(16.0))
    .background_color(BG_SECTION);

    flex_col((
        header,
        tab_strip,
        flex_row((search.flex(1.0),)).padding(Length::px(12.0)),
        portal(
            flex_col(rows)
                .gap(4.px())
                .padding(Length::px(12.0))
                .background_color(BG_SECTION),
        )
        .flex(1.0),
        footer,
    ))
    .gap(0.px())
    .background_color(BG_DARK)
}

fn main() {
    let app = Xilem::new_simple(
        AppState::default(),
        app_logic,
        WindowOptions::new("Material Icons Gallery")
            .with_initial_inner_size(xilem::winit::dpi::LogicalSize::new(1100.0, 760.0)),
    );

    app.run_in(EventLoop::with_user_event()).unwrap();
}

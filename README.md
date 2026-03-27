# Xilem Material Icons

Material Symbols icons for [Xilem](https://github.com/linebender/xilem) applications.

Bundles the [Material Symbols Outlined](https://fonts.google.com/icons) variable font from Google with 150+ icon codepoints ready to use.

Similar to [egui_material_icons](https://crates.io/crates/egui_material_icons) but for the Xilem/Masonry ecosystem.

## Quick Start

```toml
[dependencies]
xilem_material_icons = "0.1"
```

```rust
use xilem_material_icons::{FONT_DATA, icon, icons};
use xilem::masonry::vello::peniko::Color;

// Register font when creating your app
let app = Xilem::new(state, app_logic)
    .with_font(FONT_DATA);

// Use the icon view
icon(icons::FOLDER).build()
icon(icons::SETTINGS).size(24.0).build()
icon(icons::ERROR).color(Color::RED).build()

// Convenience functions for common sizes
icon_sm(icons::CHECK).build()  // 16px
icon_md(icons::CHECK).build()  // 20px (default)
icon_lg(icons::CHECK).build()  // 24px
icon_xl(icons::CHECK).build()  // 32px
```

Run the gallery example:
```bash
cargo run --example gallery
```

<img width="972" height="844" alt="GalleryIconsScreenshot" src="https://github.com/user-attachments/assets/cbf1ce7e-5257-41bf-bc74-56dc0d307521" />


## Available Icons

**Navigation**: `CHEVRON_RIGHT`, `CHEVRON_LEFT`, `EXPAND_MORE`, `EXPAND_LESS`, `ARROW_BACK`, `ARROW_FORWARD`, `MENU`, `MORE_VERT`

**Files**: `FOLDER`, `FOLDER_OPEN`, `DESCRIPTION`, `INSERT_DRIVE_FILE`, `DOWNLOAD`, `UPLOAD_FILE`, `ATTACH_FILE`

**Actions**: `CHECK`, `CLOSE`, `ADD`, `REMOVE`, `DELETE`, `EDIT`, `SAVE`, `SEARCH`, `REFRESH`, `SETTINGS`, `UNDO`, `REDO`, `COPY`, `PASTE`, `CUT`

**Status**: `ERROR`, `WARNING`, `INFO`, `CHECK_CIRCLE`, `HELP`, `CANCEL`

**Selection**: `CHECK_BOX`, `CHECK_BOX_OUTLINE_BLANK`, `RADIO_BUTTON_CHECKED`, `RADIO_BUTTON_UNCHECKED`

**Media**: `PLAY_ARROW`, `PAUSE`, `STOP`, `SKIP_NEXT`, `SKIP_PREVIOUS`, `VOLUME_UP`, `VOLUME_DOWN`, `VOLUME_MUTE`

**Social**: `FAVORITE`, `STAR`, `THUMB_UP`, `THUMB_DOWN`, `SHARE`

**And many more...** See `src/lib.rs` for the complete list.

## Icon Sizes

```rust
use xilem_material_icons::{ICON_SIZE_SM, ICON_SIZE_MD, ICON_SIZE_LG, ICON_SIZE_XL};

// SM: 16px, MD: 20px, LG: 24px, XL: 32px
```

## Platform Support

- macOS Tahoe 26.3
- Linux (tested on Arch with Cosmic)
- Windows (untested but should work)

## License

This crate is licensed under the Apache License 2.0.

### Font License

The Material Symbols Outlined font is copyright Google LLC and licensed under the Apache License 2.0.

- Repository: [google/material-design-icons](https://github.com/google/material-design-icons)
- License: [Apache 2.0](https://github.com/google/material-design-icons/blob/master/LICENSE)
- Font source: [MaterialSymbolsOutlined.ttf](https://github.com/google/material-design-icons/raw/master/variablefont/MaterialSymbolsOutlined%5BFILL%2CGRAD%2Copsz%2Cwght%5D.ttf)
- Google Fonts: [Material Symbols](https://fonts.google.com/icons)

## Author

Jacek Wisniowski

---

*This software is provided as-is, without warranty. Use at your own risk.*

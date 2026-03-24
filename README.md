# Xilem Material Icons

Material Symbols icons for [Xilem](https://github.com/linebender/xilem) 0.4.0 applications.

Bundles the [Material Symbols Outlined](https://fonts.google.com/icons) variable font from Google with 150+ icon codepoints ready to use.

Similar to [egui_material_icons](https://crates.io/crates/egui_material_icons) but for the Xilem/Masonry ecosystem.

## Quick Start

```toml
[dependencies]
xilem_material_icons = "0.1"
```

```rust
use xilem_material_icons::{FONT_DATA, FONT_FAMILY, icons};

// Register font when creating your app
let app = Xilem::new(state, app_logic)
    .with_font(FONT_DATA);

// Use icons in views
label(icons::FOLDER)
    .font(FONT_FAMILY)
    .text_size(24.0)
```

Run the gallery example:
```bash
cargo run --example gallery
```

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

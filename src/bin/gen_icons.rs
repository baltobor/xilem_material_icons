//! Generator for `src/icons_generated.rs`.
//!
//! Reads the official Material Symbols codepoints file (one
//! `name hex` per line) and emits a Rust source file with one
//! `pub const NAME: &str = "\u{HHHH}";` per icon.
//!
//! Run: `cargo run --bin gen_icons` (manifest dir is the lib root).
//! Output is written to `src/icons_generated.rs` and committed —
//! consumers do not run the generator at build time.
//!
//! Names starting with a digit (`10k`, `123`, `12mp`, …) get a
//! leading underscore so they're valid Rust identifiers
//! (`_10K`, `_123`, `_12MP`).

use std::fs;
use std::path::PathBuf;

const HEADER: &str = "// AUTO-GENERATED — do not edit by hand.\n\
// Regenerate with: `cargo run --bin gen_icons` from this crate's root.\n\
// Source: assets/MaterialSymbolsOutlined.codepoints\n\
// (mirrored from https://github.com/google/material-design-icons)\n\n";

fn main() {
    let manifest_dir: PathBuf = env!("CARGO_MANIFEST_DIR").into();
    let codepoints_path = manifest_dir.join("assets/MaterialSymbolsOutlined.codepoints");
    let out_path = manifest_dir.join("src/icons_generated.rs");

    let raw = fs::read_to_string(&codepoints_path)
        .unwrap_or_else(|e| panic!("read {}: {e}", codepoints_path.display()));

    let mut out = String::with_capacity(raw.len() * 4);
    out.push_str(HEADER);

    let mut count = 0usize;
    let mut emitted_idents: std::collections::HashSet<String> = Default::default();

    for line in raw.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut parts = line.split_whitespace();
        let (Some(name), Some(hex)) = (parts.next(), parts.next()) else { continue };

        // Validate hex: 4–6 hex chars, parseable.
        let cp = match u32::from_str_radix(hex, 16) {
            Ok(v) => v,
            Err(_) => continue,
        };

        let ident = name_to_ident(name);
        // Defensive: skip duplicate identifiers (shouldn't happen with
        // Google's data, but if two names normalise to the same ident
        // we keep the first to stay deterministic).
        if !emitted_idents.insert(ident.clone()) {
            continue;
        }

        out.push_str(&format!(
            "/// `{name}` — U+{hex}\npub const {ident}: &str = \"\\u{{{cp:X}}}\";\n",
        ));
        count += 1;
    }

    fs::write(&out_path, &out)
        .unwrap_or_else(|e| panic!("write {}: {e}", out_path.display()));

    println!(
        "Wrote {} icon constants to {}",
        count,
        out_path.display()
    );
}

/// Map Google's `snake_case_with_optional_leading_digits` to a valid
/// Rust UPPER_SNAKE identifier. Underscore-prefix the cases that would
/// otherwise start with a digit.
fn name_to_ident(name: &str) -> String {
    let upper = name.to_ascii_uppercase();
    if upper.chars().next().map_or(false, |c| c.is_ascii_digit()) {
        format!("_{upper}")
    } else {
        upper
    }
}

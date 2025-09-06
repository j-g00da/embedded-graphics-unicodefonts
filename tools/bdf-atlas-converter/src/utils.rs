use bdf_parser::{Encoding, Font};
use color_eyre::Result;
use std::path::Path;

/// Extract all valid glyph code points from a font
pub fn extract_glyphs_from_font(font: &Font) -> Vec<u32> {
    font.glyphs
        .iter()
        .filter_map(|g| match g.encoding {
            Encoding::Standard(v) | Encoding::NonStandard(v) => Some(v),
            Encoding::Unspecified => None,
        })
        .collect()
}

/// Load and parse a BDF font file
pub fn load_font(path: &Path) -> Result<Font> {
    let font_content = std::fs::read_to_string(path)?;
    Ok(Font::parse(&font_content)?)
}

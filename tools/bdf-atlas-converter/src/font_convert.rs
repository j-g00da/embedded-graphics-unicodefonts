use crate::args::Args;
use crate::font_info::print_glyph_summary;
use crate::layout::{layout_glyphs, GlyphLayout};
use crate::utils::{extract_glyphs_from_font, load_font};
use bdf_parser::Font;
use color_eyre::{eyre::eyre, Result};
use eg_font_converter::{FontConverter, MonoFontOutput};
use embedded_graphics::geometry::Size;
use embedded_graphics::mono_font::{DecorationDimensions, MonoFont};
use embedded_graphics_unicodefonts::atlas::NamedUnicodeBlock;
use std::ffi::OsStr;
use std::ops::RangeInclusive;
use std::path::{Path, PathBuf};

const DEFAULT_BLOCKS: &[NamedUnicodeBlock] = &[
    NamedUnicodeBlock::Ascii,
    NamedUnicodeBlock::Latin1,
    NamedUnicodeBlock::BoxDrawing,
    NamedUnicodeBlock::BlockElements,
    NamedUnicodeBlock::Miscellaneous,
    NamedUnicodeBlock::BraillePatterns,
    NamedUnicodeBlock::CurrencySymbols,
    NamedUnicodeBlock::SymbolsAndArrows,
];

pub fn cmd_convert_font(args: Args) -> Result<()> {
    let output_path = determine_output_path(&args)?;
    let font = load_font(&args.input)?;
    let filtered_glyphs = build_filtered_glyphs(&font, &args.ranges);
    let total_filtered_glyphs = filtered_glyphs.len();

    let glyph_layout = layout_glyphs(filtered_glyphs, args.gap_threshold, args.min_range_length);
    let basename = create_font_basename(&args.input, args.suffix.as_deref())?;

    let (font_output, atlas_src) = generate_font_files(&basename, &args.input, &glyph_layout)?;

    save_font(
        basename,
        &output_path,
        atlas_src,
        font_output,
        args.save_png,
    )?;

    // Print summary of what was generated
    println!("Font Generation Summary:");
    print_glyph_summary(
        &args.input,
        Some(&output_path),
        total_filtered_glyphs,
        &glyph_layout,
    );

    Ok(())
}

fn determine_output_path(args: &Args) -> Result<PathBuf> {
    if let Some(path) = &args.output {
        Ok(path.clone())
    } else {
        args.input
            .file_stem()
            .and_then(OsStr::to_str)
            .map(normalize_font_name)
            .map(PathBuf::from)
            .ok_or_else(|| eyre!("Invalid input filename"))
    }
}

fn build_filtered_glyphs(font: &Font, additional_ranges: &[RangeInclusive<char>]) -> Vec<u32> {
    let available_glyphs: std::collections::HashSet<u32> =
        extract_glyphs_from_font(font).into_iter().collect();

    // Get requested ranges (default blocks + command line ranges)
    let requested_ranges: Vec<RangeInclusive<char>> = DEFAULT_BLOCKS
        .iter()
        .map(NamedUnicodeBlock::range)
        .chain(additional_ranges.iter().cloned())
        .collect();

    // Filter requested ranges to only include available glyphs
    let mut filtered_glyphs: Vec<u32> = requested_ranges
        .into_iter()
        .flat_map(|range| range.filter(|c| !c.is_ascii_control()).map(u32::from))
        .filter(|code_point| available_glyphs.contains(code_point))
        .collect();

    filtered_glyphs.sort_unstable();
    filtered_glyphs.dedup();
    filtered_glyphs
}

fn create_font_basename(input: &Path, suffix: Option<&str>) -> Result<String> {
    let mut basename = input
        .file_stem()
        .and_then(OsStr::to_str)
        .map(normalize_font_name)
        .ok_or_else(|| eyre!("Invalid input filename"))?;

    if let Some(suffix) = suffix {
        basename.push_str(suffix);
    }

    Ok(basename)
}

fn generate_font_files(
    basename: &str,
    input: &Path,
    blocks: &[GlyphLayout],
) -> Result<(MonoFontOutput, String)> {
    let min_range_length = blocks
        .iter()
        .filter_map(|b| match b {
            GlyphLayout::Range { span, .. } => Some(range_len(span)),
            GlyphLayout::Single(_) => None,
        })
        .min()
        .unwrap_or(0);

    let mapping_string = generate_ranges_string(blocks);
    let font_output = convert_bdf(basename, input, blocks)?;
    let atlas_src = rust_font_atlas(basename, &font_output, &mapping_string, min_range_length)?;

    Ok((font_output, atlas_src))
}

fn save_font(
    name: String,
    output_path: &PathBuf,
    atlas_src: String,
    font_output: MonoFontOutput,
    save_png: bool,
) -> Result<()> {
    if !output_path.exists() {
        std::fs::create_dir_all(output_path)?;
    }

    // save the .data file + standard .rs file
    font_output.save(output_path)?;

    // save the atlas .rs file
    let atlas_file_path = output_path.join(format!("{name}_atlas.rs"));
    std::fs::write(&atlas_file_path, atlas_src)?;

    // save PNG if requested
    if save_png {
        let png_path = output_path.join(format!("{name}.png"));
        font_output
            .save_png(&png_path)
            .map_err(|e| eyre!("Failed to save PNG: {}", e))?;
    }

    Ok(())
}

fn convert_bdf(basename: &str, input: &Path, blocks: &[GlyphLayout]) -> Result<MonoFontOutput> {
    let name = basename.to_ascii_uppercase();
    let mut converter = FontConverter::with_file(input, &name);

    for block in blocks {
        let glyphs = match block {
            GlyphLayout::Range { span, .. } => span.clone(),
            GlyphLayout::Single('\u{0000}') => continue,
            GlyphLayout::Single(c) => *c..=*c,
        };

        converter = converter.glyphs(glyphs);
    }

    converter
        .missing_glyph_substitute(' ')
        .convert_mono_font()
        .map_err(|e| eyre!("{}", e))
}

fn generate_ranges_string(blocks: &[GlyphLayout]) -> String {
    let mut result = String::new();

    for range in blocks {
        match range {
            GlyphLayout::Range { span, .. } => {
                result.push('\0'); // Range start marker
                result.push(*span.start()); // Range start character
                result.push(*span.end()); // Range end character
            }
            GlyphLayout::Single(c) => result.push(*c),
        }
    }

    result
}

fn rust_font_atlas(
    font_basename: &str,
    font_output: &MonoFontOutput,
    mapping_string: &str,
    min_range_length: usize,
) -> Result<String> {
    let MonoFont {
        character_size:
            Size {
                width: character_width,
                height: character_height,
            },
        character_spacing,
        baseline,
        underline:
            DecorationDimensions {
                offset: underline_offset,
                height: underline_height,
            },
        strikethrough:
            DecorationDimensions {
                offset: strikethrough_offset,
                height: strikethrough_height,
            },
        ..
    } = font_output.as_font();

    let content = format!(
        r#"use crate::atlas::FontAtlas;

/// **Danger**: leaking [`FontAtlas<'static>`] for the lifetime of the program
pub fn {font_basename}_atlas() -> ::embedded_graphics::mono_font::MonoFont<'static> {{
    let atlas = FontAtlas::from_mapping_str({min_range_length}, {mapping_string:?})
        .leak();

    ::embedded_graphics::mono_font::MonoFont {{
        image: ::embedded_graphics::image::ImageRaw::new(
            include_bytes!("{font_basename}.data"),
            {texture_width}u32,
        ),
        glyph_mapping: atlas,
        character_size: ::embedded_graphics::geometry::Size::new({character_width}u32, {character_height}u32),
        character_spacing: {character_spacing}u32,
        baseline: {baseline}u32,
        underline: ::embedded_graphics::mono_font::DecorationDimensions::new({underline_offset}u32, {underline_height}u32),
        strikethrough: ::embedded_graphics::mono_font::DecorationDimensions::new({strikethrough_offset}u32, {strikethrough_height}u32),
    }}
}}"#,
        texture_width = character_width * 16
    );

    Ok(content)
}

fn normalize_font_name(filename: &str) -> String {
    let mut name = filename.to_string();

    // Handle bold/italic suffixes first
    if name.ends_with('B') {
        name.pop();
        name.push_str("_bold");
    } else if name.ends_with('O') {
        name.pop();
        name.push_str("_italic");
    }

    // Add underscore before known locale suffixes: ja, ko
    // "12x13ja" -> "12x13_ja", "18x18ko" -> "18x18_ko"
    // Only replace if not already preceded by underscore
    if name.contains("ja") && !name.contains("_ja") {
        name = name.replace("ja", "_ja");
    }
    if name.contains("ko") && !name.contains("_ko") {
        name = name.replace("ko", "_ko");
    }

    format!("mono_{}", name)
}

/// Calculates the length of a character range.
pub fn range_len(range: &RangeInclusive<char>) -> usize {
    *range.end() as usize - *range.start() as usize + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_font_name() {
        // Basic fonts
        assert_eq!(normalize_font_name("6x13"), "mono_6x13");
        assert_eq!(normalize_font_name("8x16"), "mono_8x16");

        // Bold/italic variants
        assert_eq!(normalize_font_name("6x13B"), "mono_6x13_bold");
        assert_eq!(normalize_font_name("7x14O"), "mono_7x14_italic");

        // Localized fonts
        assert_eq!(normalize_font_name("12x13ja"), "mono_12x13_ja");
        assert_eq!(normalize_font_name("18x18ko"), "mono_18x18_ko");

        // Already normalized (shouldn't double-underscore)
        assert_eq!(normalize_font_name("12x13_ja"), "mono_12x13_ja");
    }
}

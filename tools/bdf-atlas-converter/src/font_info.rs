use crate::args::Args;
use crate::layout::{layout_glyphs, GlyphLayout};
use crate::utils::{extract_glyphs_from_font, load_font};
use color_eyre::Result;

pub fn cmd_font_info(args: Args) -> Result<()> {
    let font = load_font(&args.input)?;

    let mut glyphs = extract_glyphs_from_font(&font);

    // Check if glyphs are in sorted order BEFORE sorting
    let mut is_sorted = true;
    let mut last_ordered_code = None;
    let mut first_out_of_order_code = None;

    for window in glyphs.windows(2) {
        if window[0] > window[1] {
            is_sorted = false;
            last_ordered_code = Some(window[0]);
            first_out_of_order_code = Some(window[1]);
            break;
        }
    }

    println!("Original glyphs are sorted: {}", is_sorted);
    if let (Some(last), Some(first)) = (last_ordered_code, first_out_of_order_code) {
        println!(
            "Last ordered code point: U+{:04X}, First out-of-order: U+{:04X}",
            last, first
        );
    }

    glyphs.sort_unstable();
    let total_glyphs = glyphs.len();

    // join glyphs into ranges
    let glyph_layout: Vec<GlyphLayout> =
        layout_glyphs(glyphs, args.gap_threshold, args.min_range_length);

    println!("Font Information: {}", args.input.display());
    println!("Font Properties:");
    println!("  Name: {}", font.metadata.name);
    println!(
        "  Bounding box size: {}x{}",
        font.metadata.bounding_box.size.x, font.metadata.bounding_box.size.y
    );
    println!(
        "  Bounding box offset: ({}, {})",
        font.metadata.bounding_box.offset.x, font.metadata.bounding_box.offset.y
    );
    println!("  Point size: {}", font.metadata.point_size);
    println!(
        "  Resolution: {}x{} dpi",
        font.metadata.resolution.x, font.metadata.resolution.y
    );
    println!();

    print_glyph_summary(&args.input, None, total_glyphs, &glyph_layout);
    println!();

    // extract ranges from the glyph layout
    let ranges: Vec<_> = glyph_layout
        .iter()
        .filter_map(|g| match g {
            GlyphLayout::Range { span, .. } => Some(span),
            _ => None,
        })
        .collect();

    if !ranges.is_empty() {
        let total_range_chars: u32 = glyph_layout
            .iter()
            .filter_map(|g| match g {
                GlyphLayout::Range { .. } => Some(g.glyph_count()),
                _ => None,
            })
            .sum();
        println!("Unicode Ranges ({} chars total):", total_range_chars);

        // First range has no gap info
        if let Some(first_range) = ranges.first() {
            let start_char = *first_range.start();
            let end_char = *first_range.end();
            println!(
                "  U+{:04X}..U+{:04X} ({} chars) '{}' to '{}'",
                start_char as u32,
                end_char as u32,
                end_char as u32 - start_char as u32 + 1,
                start_char,
                end_char
            );
        }

        // Subsequent ranges show gap from previous range
        for window in ranges.windows(2) {
            let prev_range = &window[0];
            let curr_range = &window[1];
            let gap = *curr_range.start() as u32 - *prev_range.end() as u32 - 1;

            let start_char = *curr_range.start();
            let end_char = *curr_range.end();

            println!(
                "  U+{:04X}..U+{:04X} ({} chars) '{}' to '{}' (gap: {} chars)",
                start_char as u32,
                end_char as u32,
                end_char as u32 - start_char as u32 + 1,
                start_char,
                end_char,
                gap
            );
        }
        println!();
    }

    // Extract singles from the glyph layout
    let singles: Vec<_> = glyph_layout
        .iter()
        .filter_map(|g| match g {
            GlyphLayout::Single(c) => Some(*c),
            _ => None,
        })
        .collect();

    if !singles.is_empty() {
        println!("Individual Characters:");
        for (i, ch) in singles.iter().enumerate() {
            if i > 0 && i % 8 == 0 {
                println!();
            }
            print!("  U+{:04X}('{}') ", *ch as u32, ch);
        }
        println!();
    }

    Ok(())
}

pub fn print_glyph_summary(
    input_path: &std::path::Path,
    output_path: Option<&std::path::Path>,
    total_glyphs: usize,
    glyph_layout: &[GlyphLayout],
) {
    let skipped_chars_count = glyph_layout.iter().fold(0, |acc, g| match g {
        GlyphLayout::Range { skipped, .. } => acc + *skipped,
        _ => acc,
    });

    let ranges: Vec<_> = glyph_layout
        .iter()
        .filter_map(|g| match g {
            GlyphLayout::Range { span, .. } => Some(span),
            _ => None,
        })
        .collect();

    let singles_count = glyph_layout
        .iter()
        .filter(|g| matches!(g, GlyphLayout::Single(_)))
        .count();

    let total_range_chars: u32 = glyph_layout
        .iter()
        .filter_map(|g| match g {
            GlyphLayout::Range { .. } => Some(g.glyph_count()),
            _ => None,
        })
        .sum();

    println!("Character Coverage:");
    println!("  Total glyphs: {}", total_glyphs);
    println!(
        "  Ranges: {} ({} chars total)",
        ranges.len(),
        total_range_chars
    );
    println!("  Gaps (wasted): {}", skipped_chars_count);
    println!("  Single characters: {}", singles_count);

    if let Some(output) = output_path {
        println!("  Input: {}", input_path.display());
        println!("  Output: {}", output.display());
    }
}

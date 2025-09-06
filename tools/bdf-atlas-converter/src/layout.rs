use crate::font_convert::range_len;
use std::ops::RangeInclusive;

#[derive(Debug)]
pub enum GlyphLayout {
    Range {
        span: RangeInclusive<char>,
        skipped: u32,
    },
    Single(char),
}

impl GlyphLayout {
    pub fn glyph_count(&self) -> u32 {
        match self {
            GlyphLayout::Range { span, .. } => range_len(span) as _,
            GlyphLayout::Single(_) => 1,
        }
    }
}

pub fn layout_glyphs(
    glyphs: Vec<u32>,
    max_gap_threshold: u32,
    min_range_length: usize,
) -> Vec<GlyphLayout> {
    let mut layout = Vec::new();

    if glyphs.is_empty() {
        return layout;
    }

    let mut skipped_chars_count = 0;
    let mut range_start = glyphs[0];
    let mut range_end = glyphs[0];

    // end current range and decide if it's a range or singles
    let mut flush_range = |start: u32, end: u32, skipped: u32| {
        let range_length = (end - start + 1) as usize;
        if range_length >= min_range_length {
            layout.push(GlyphLayout::Range {
                span: into_char(start)..=into_char(end),
                skipped,
            });
        } else {
            for code in start..=end {
                layout.push(GlyphLayout::Single(into_char(code)));
            }
        }
    };

    for &glyph in &glyphs[1..] {
        if glyph <= range_end + 1 + max_gap_threshold {
            // Count characters we're skipping in the gap
            if glyph > range_end + 1 {
                skipped_chars_count += glyph - range_end - 1;
            }
            // Extend current range
            range_end = glyph;
        } else {
            flush_range(range_start, range_end, skipped_chars_count);

            // Start new range
            range_start = glyph;
            range_end = glyph;
            skipped_chars_count = 0;
        }
    }

    // Handle the last range
    flush_range(range_start, range_end, skipped_chars_count);

    layout
}

fn into_char(c: u32) -> char {
    // we know c is a valid unicode code point as it was derived from u32::from(char)
    char::from_u32(c).unwrap_or('\u{FFFD}')
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper functions to extract data from the new layout format for testing
    fn extract_ranges(layout: &[GlyphLayout]) -> Vec<std::ops::RangeInclusive<u32>> {
        layout
            .iter()
            .filter_map(|g| match g {
                GlyphLayout::Range { span, .. } => Some(*span.start() as u32..=*span.end() as u32),
                _ => None,
            })
            .collect()
    }

    fn extract_singles(layout: &[GlyphLayout]) -> Vec<u32> {
        layout
            .iter()
            .filter_map(|g| match g {
                GlyphLayout::Single(c) => Some(*c as u32),
                _ => None,
            })
            .collect()
    }

    fn extract_skipped_chars_count(layout: &[GlyphLayout]) -> u32 {
        layout.iter().fold(0, |acc, g| match g {
            GlyphLayout::Range { skipped, .. } => acc + *skipped,
            _ => acc,
        })
    }

    #[test]
    fn test_layout_glyphs_no_gaps() {
        let glyphs = vec![10, 11, 12, 15, 16, 17, 18, 19, 20, 21, 22];
        let layout = layout_glyphs(glyphs, 0, 4);

        // Range 10-12 is only 3 chars, below min_range_length=4, so should be singles
        // Range 15-22 is 8 chars, meets min_range_length=4, so should be a range
        let ranges = extract_ranges(&layout);
        let singles = extract_singles(&layout);
        let skipped_count = extract_skipped_chars_count(&layout);

        assert_eq!(ranges.len(), 1);
        assert_eq!(ranges[0], 15..=22);
        assert_eq!(singles.len(), 3);
        assert!(singles.contains(&10));
        assert!(singles.contains(&11));
        assert!(singles.contains(&12));
        assert_eq!(skipped_count, 0);
    }

    #[test]
    fn test_layout_glyphs_with_gaps() {
        let glyphs = vec![10, 12, 14, 16, 18, 20, 22];
        let layout = layout_glyphs(glyphs, 1, 4);

        // With gap_threshold=1, should bridge single gaps: 10,12,14,16,18,20,22
        let ranges = extract_ranges(&layout);
        let singles = extract_singles(&layout);
        let skipped_count = extract_skipped_chars_count(&layout);

        assert_eq!(ranges.len(), 1);
        assert_eq!(ranges[0], 10..=22);
        assert_eq!(singles.len(), 0);
        // Should skip: 11, 13, 15, 17, 19, 21 = 6 chars
        assert_eq!(skipped_count, 6);
    }

    #[test]
    fn test_layout_glyphs_min_range_length() {
        let glyphs = vec![10, 11, 12, 20, 21, 30];
        let layout = layout_glyphs(glyphs, 0, 4);

        // Ranges 10-12 and 20-21 are too short (< 4), should be singles
        let ranges = extract_ranges(&layout);
        let singles = extract_singles(&layout);

        assert_eq!(ranges.len(), 0);
        assert_eq!(singles.len(), 6);
        assert!(singles.contains(&10));
        assert!(singles.contains(&11));
        assert!(singles.contains(&12));
        assert!(singles.contains(&20));
        assert!(singles.contains(&21));
        assert!(singles.contains(&30));
    }

    #[test]
    fn test_layout_glyphs_mixed_ranges_singles() {
        let glyphs = vec![10, 11, 12, 13, 14, 15, 16, 17, 25, 26, 35];
        let layout = layout_glyphs(glyphs, 0, 4);

        // Should have one range 10-17 and singles 25, 26, 35
        let ranges = extract_ranges(&layout);
        let singles = extract_singles(&layout);

        assert_eq!(ranges.len(), 1);
        assert_eq!(ranges[0], 10..=17);
        assert_eq!(singles.len(), 3);
        assert!(singles.contains(&25));
        assert!(singles.contains(&26));
        assert!(singles.contains(&35));
    }

    #[test]
    fn test_layout_glyphs_large_gaps() {
        let glyphs = vec![10, 11, 12, 20, 21, 22];
        let layout = layout_glyphs(glyphs, 5, 3);

        // Gap between 12 and 20 is 7, but threshold is 5, so no bridge
        let ranges = extract_ranges(&layout);
        let skipped_count = extract_skipped_chars_count(&layout);

        assert_eq!(ranges.len(), 2);
        assert_eq!(ranges[0], 10..=12);
        assert_eq!(ranges[1], 20..=22);
        assert_eq!(skipped_count, 0);
    }

    #[test]
    fn test_layout_glyphs_bridge_large_gaps() {
        let glyphs = vec![10, 11, 12, 20, 21, 22];
        let layout = layout_glyphs(glyphs, 10, 3);

        // Gap between 12 and 20 is 7, threshold is 10, so should bridge
        let ranges = extract_ranges(&layout);
        let singles = extract_singles(&layout);
        let skipped_count = extract_skipped_chars_count(&layout);

        assert_eq!(ranges.len(), 1);
        assert_eq!(ranges[0], 10..=22);
        assert_eq!(singles.len(), 0);
        // Should skip chars: 13, 14, 15, 16, 17, 18, 19 = 7 chars
        assert_eq!(skipped_count, 7);
    }

    #[test]
    fn test_layout_glyphs_empty() {
        let glyphs = vec![];
        let layout = layout_glyphs(glyphs, 1, 8);

        let ranges = extract_ranges(&layout);
        let singles = extract_singles(&layout);
        let skipped_count = extract_skipped_chars_count(&layout);

        assert_eq!(ranges.len(), 0);
        assert_eq!(singles.len(), 0);
        assert_eq!(skipped_count, 0);
    }

    #[test]
    fn test_layout_glyphs_single_char() {
        let glyphs = vec![42];
        let layout = layout_glyphs(glyphs, 1, 8);

        let ranges = extract_ranges(&layout);
        let singles = extract_singles(&layout);
        let skipped_count = extract_skipped_chars_count(&layout);

        assert_eq!(ranges.len(), 0);
        assert_eq!(singles.len(), 1);
        assert_eq!(singles[0], 42);
        assert_eq!(skipped_count, 0);
    }

    #[test]
    fn test_layout_glyphs_deduplication() {
        let glyphs = vec![10, 10, 11, 11, 12, 12, 13, 13];
        let layout = layout_glyphs(glyphs, 0, 4);

        // Should deduplicate and create one range
        let ranges = extract_ranges(&layout);
        let singles = extract_singles(&layout);

        assert_eq!(ranges.len(), 1);
        assert_eq!(ranges[0], 10..=13);
        assert_eq!(singles.len(), 0);
    }
}

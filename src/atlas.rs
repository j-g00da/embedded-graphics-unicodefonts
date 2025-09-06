use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::cmp::Ordering;
use core::ops::RangeInclusive;
use embedded_graphics::mono_font::mapping::{GlyphMapping, StrGlyphMapping};

/// ASCII space character offset for fast ASCII lookups
const ASCII_OFFSET: usize = *NamedUnicodeBlock::Ascii.range().start() as usize;

impl GlyphMapping for FontAtlas {
    /// Returns the glyph index for a character, falling back to index 0 (space) if not found
    fn index(&self, c: char) -> usize {
        self.find(c).unwrap_or(0)
    }
}

/// Font atlas containing Unicode blocks and additional symbols for efficient glyph lookup.
///
/// The atlas stores characters in two categories:
/// - Contiguous Unicode blocks (e.g., ASCII, Latin-1) for efficient range-based lookup
/// - Individual symbols stored separately and accessed via binary search
///
/// # Performance
/// - ASCII characters use a fast path with direct offset calculation
/// - Non-ASCII characters are looked up via binary search within blocks
/// - Memory usage is optimized for embedded systems
pub struct FontAtlas {
    blocks: Vec<UnicodeBlock>,
    other_symbols: BTreeMap<char, usize>,
}

impl FontAtlas {
    /// Returns the total number of glyphs in the atlas.
    pub fn glyph_count(&self) -> usize {
        self.glyph_count_in_blocks() + self.other_symbols.len()
    }

    /// Returns an iterator over all characters in the atlas.
    ///
    /// Unicode blocks are yielded first, followed by individual symbols.
    pub fn iter(&self) -> impl Iterator<Item = char> + '_ {
        self.blocks
            .iter()
            .flat_map(|b| b.iter())
            .chain(self.other_symbols.keys().copied())
    }

    /// Returns `true` if the atlas contains the given character.
    pub fn contains(&self, symbol: char) -> bool {
        self.blocks.iter().any(|b| b.contains(symbol)) || self.other_symbols.contains_key(&symbol)
    }

    /// Finds the glyph index for a character.
    ///
    /// # Returns
    /// The glyph index if the character is in the atlas, `None` otherwise.
    pub fn find(&self, symbol: char) -> Option<usize> {
        // ca 70% faster with ASCII fast path in benchmarks
        if symbol.is_ascii() {
            return Some(symbol as usize - ASCII_OFFSET);
        }

        // Binary search through non-ASCII blocks for better performance
        if self.blocks.len() > 1 {
            if let Ok(index) =
                self.blocks[1..].binary_search_by(|block| compare_symbol_to_block(symbol, block))
            {
                // Found the block containing the symbol
                if let Some(glyph_index) = self.blocks[index + 1].try_index(symbol) {
                    return Some(glyph_index);
                }
            }
        }

        // Fallback to individual symbols
        self.other_symbols.get(&symbol).copied()
    }

    /// Leaks the font atlas to obtain a 'static reference.
    ///
    /// # Safety
    /// This permanently leaks memory and should only be used when the atlas needs
    /// to live for the entire program duration.
    #[must_use]
    pub fn leak(self) -> &'static Self {
        // danger: leaking memory for the lifetime of the program
        Box::leak(Box::new(self))
    }

    /// Creates a font atlas by partitioning character ranges based on size threshold.
    ///
    /// Ranges meeting the minimum threshold become Unicode blocks for efficient lookup,
    /// while smaller ranges are stored as individual symbols in a `BTreeMap`.
    ///
    /// Panics when debug assertions are enabled if ranges are not sorted.
    pub fn from_mapping_str(min_range_threshold: usize, mapping: &str) -> Self {
        // the atlas uses the same mapping as StrGlyphMapping,
        // so we can reuse its parsing logic
        let mapping = StrGlyphMapping::new(mapping, 0);

        let (blocks, singles): (Vec<_>, Vec<_>) = mapping
            .ranges()
            .partition(|(_, range)| range_len(range) >= min_range_threshold);

        let blocks: Vec<_> = blocks.into_iter().map(UnicodeBlock::from).collect();

        let singles = singles
            .into_iter()
            .flat_map(|(offset, range)| range.into_iter().map(move |c| (c, offset)))
            .collect::<BTreeMap<_, _>>();

        debug_assert!(
            blocks.windows(2).all(|w| w[0].start() < w[1].start()),
            "Invalid FontAtlas: ranges must be sorted"
        );

        Self {
            blocks,
            other_symbols: singles,
        }
    }

    fn glyph_count_in_blocks(&self) -> usize {
        self.blocks
            .last()
            .map(|b| b.len() + b.base_offset)
            .unwrap_or(0)
    }
}

/// Internal representation of a Unicode block with base offset for glyph indexing.
#[derive(Debug, Clone)]
struct UnicodeBlock {
    base_offset: usize,
    range: RangeInclusive<char>,
}

impl UnicodeBlock {
    fn contains(&self, symbol: char) -> bool {
        symbol <= *self.range.end() && symbol >= *self.range.start()
    }

    /// Returns the glyph index for the symbol if it's in this block
    fn try_index(&self, symbol: char) -> Option<usize> {
        if self.contains(symbol) {
            let offset = symbol as usize - *self.range.start() as usize;
            Some(self.base_offset + offset)
        } else {
            None
        }
    }

    fn len(&self) -> usize {
        range_len(&self.range)
    }

    /// Iterates over all characters in the atlas
    fn iter(&self) -> impl Iterator<Item = char> + '_ {
        self.range.clone()
    }

    fn start(&self) -> char {
        *self.range.start()
    }
}

/// Named Unicode blocks with predefined ranges for font generation
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum NamedUnicodeBlock {
    Ascii,            // 0x0020..=0x007F
    Latin1,           // 0x00A0..=0x00FF
    BlockElements,    // 0x2580..=0x259F
    BoxDrawing,       // 0x2500..=0x257F
    Miscellaneous,    // 0x2600..=0x26FF
    BraillePatterns,  // 0x2800..=0x28FF
    CurrencySymbols,  // 0x20A0..=0x20CF
    Dingbats,         // 0x2700..=0x27BF
    SymbolsAndArrows, // 0x2B00..=0x2BFF
    TransportAndMap,  // 0x1F680..=0x1F6FF
    Pictographs,      // 0x1F300..=0x1F5FF
}

impl NamedUnicodeBlock {
    /// Returns the Unicode range for this block
    #[rustfmt::skip]
    pub const fn range(&self) -> RangeInclusive<char> {
        match self {
            Self::Ascii            => '\u{0020}'..='\u{007E}',
            Self::Latin1           => '\u{00A0}'..='\u{00FF}',
            Self::BoxDrawing       => '\u{2500}'..='\u{257F}',
            Self::BlockElements    => '\u{2580}'..='\u{259F}',
            Self::Miscellaneous    => '\u{2600}'..='\u{26FF}',
            Self::BraillePatterns  => '\u{2800}'..='\u{28FF}',
            Self::CurrencySymbols  => '\u{20A0}'..='\u{20CF}',
            Self::Dingbats         => '\u{2700}'..='\u{27BF}',
            Self::SymbolsAndArrows => '\u{2B00}'..='\u{2BFF}',
            Self::Pictographs      => '\u{1F300}'..='\u{1F5FF}',
            Self::TransportAndMap  => '\u{1F680}'..='\u{1F6FF}',
        }
    }
}

/// Calculates the length of a character range.
fn range_len(range: &RangeInclusive<char>) -> usize {
    *range.end() as usize - *range.start() as usize + 1
}

impl From<(usize, RangeInclusive<char>)> for UnicodeBlock {
    fn from((base_offset, range): (usize, RangeInclusive<char>)) -> Self {
        Self { base_offset, range }
    }
}

/// Helper function for binary search comparison of symbol against Unicode block
fn compare_symbol_to_block(symbol: char, block: &UnicodeBlock) -> Ordering {
    if symbol < *block.range.start() {
        Ordering::Greater
    } else if symbol > *block.range.end() {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

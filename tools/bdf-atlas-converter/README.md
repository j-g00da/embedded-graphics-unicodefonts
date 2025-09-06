# BDF Atlas Converter

A command-line tool that converts BDF fonts into Rust font modules with
extended Unicode character support.

```
Convert BDF fonts using to atlas-backed embedded fonts

Usage: bdf-atlas-converter [OPTIONS] <INPUT>

Arguments:
  <INPUT>  Input BDF file

Options:
  -i, --info
          Display information about the BDF file without converting
  -o, --output <OUTPUT>
          Output file
  -r, --range <RANGES>
          Additional Unicode ranges in hex format (e.g., 0x20..0x7f)
      --gap-threshold <GAP_THRESHOLD>
          Maximum gap size to bridge when creating ranges (default: 1) [default: 1]
      --min-range-length <MIN_RANGE_LENGTH>
          Minimum consecutive characters needed to form a range (default: 8) [default: 8]
      --save-png
          Save a PNG image of the generated font atlas
      --suffix <SUFFIX>
          Optional suffix to append to font names (e.g. "_optimized")
  -h, --help
          Print help
```

## Overview

The `bdf-atlas-converter` generates two types of font modules from BDF font files:
- **Standard font modules**: `MonoFont` with the standard `StrGlyphMapping` for lookups
- **Atlas font modules**: `MonoFont` backed by `FontAtlas` for efficient glyph lookups

All font files in the main `embedded-graphics-unicodefonts` crate are generated using this tool.

## Basic Usage

```bash
# Build the tool
cargo build --release

# Display information about a BDF font without converting
cargo run -- input.bdf --info

# Convert a single BDF font
cargo run -- input.bdf --output ../../src

# Convert with custom suffix and range optimization settings
cargo run -- input.bdf --suffix "_optimized" --gap-threshold 2 --min-range-length 4

# Generate PNG visualization of the font atlas
cargo run -- input.bdf --save-png

# Generate all fonts using the automated script
./generate-fonts.sh
```

## Font Generation Process

The `generate-fonts.sh` generates all fonts in batch mode with these steps:

1. **Build Check**: Ensures the release binary exists, building it if necessary
2. **Font Repository**: Clones/updates the X.Org `misc-misc` font repository from GitLab
3. **Batch Processing**: Converts all compatible BDF fonts (excludes localized variants: `ja`, `ko`, `nil2`, `k14`)
4. **Output**: Places generated modules in the main crate's `src/` directory

## Unicode Block Support

Atlas fonts include these Unicode blocks by default:

- **ASCII** (`0x20-0x7F`): Basic Latin characters
- **Latin-1 Supplement** (`0x80-0xFF`): Extended Latin characters  
- **Currency Symbols** (`0x20A0-0x20CF`): Currency symbols
- **Box Drawing** (`0x2500-0x257F`): Line drawing characters
- **Block Elements** (`0x2580-0x259F`): Block/shading characters
- **Miscellaneous Symbols**: (`0x2600-0x26FF`) Various symbols
- **Braille Patterns** (`0x2800-0x28FF`): Braille dot patterns
- **Symbols and Arrows**: (`0x2B00-0x2BFF`) Arrows and mathematical symbols

Additional ranges can be specified using the `--range` option:

```bash
cargo run -- font.bdf --range 0x1F680..0x1F6FF --range 0x1F300..0x1F5FF
```

## Advanced Options

### Range Optimization

- `--gap-threshold <N>`: Bridges gaps between characters when forming ranges. Default: 1
- `--min-range-length <N>`: Minimum consecutive characters needed to form a range. Default: 8

### Font Name Suffixes

Use `--suffix` to create font variants:

```bash
# Creates mono_6x13_optimized.rs instead of mono_6x13.rs
cargo run -- 6x13.bdf --suffix "_optimized"
```

### PNG Generation

The `--save-png` flag generates a visual representation of the font atlas:

```bash
cargo run -- input.bdf --save-png --output ../../assets
```

## FontAtlas integration with embedded-graphics-unicodefonts

The tool generates code that that the creates a `MonoFont` using the `FontAtlas`
type for glyph mapping. Example generated code:

```rust
// Generated atlas module example
use crate::atlas::FontAtlas;

pub fn mono_6x13_atlas() -> ::embedded_graphics::mono_font::MonoFont<'static> {
    let atlas = FontAtlas::from("\0 \x7f\0\x80\xff\0\x2500\x257f...")
        .leak();
    
    ::embedded_graphics::mono_font::MonoFont {
        image: ::embedded_graphics::image::ImageRaw::new(
            include_bytes!("mono_6x13.data"),
            0u32,
        ),
        glyph_mapping: atlas,
        // ... font metrics
    }
}
```

The font atlas provides optimized glyph lookups compared to `embedded-graphics`'s default linear search.`


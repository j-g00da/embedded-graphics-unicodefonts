pub const MONO_6X12_OPTIMIZED: ::embedded_graphics::mono_font::MonoFont = ::embedded_graphics::mono_font::MonoFont {
    image: ::embedded_graphics::image::ImageRaw::new(
        include_bytes!("raw/mono_6x12_optimized.data"),
        96u32,
    ),
    glyph_mapping: &::embedded_graphics::mono_font::mapping::StrGlyphMapping::new(
        "\0 ~\0\u{a0}ÿ\0₠₵\0─▟\0☀⚜\0⚠⚼\0⛀⛃\0⠀⣿\0⬀⭌",
        31usize,
    ),
    character_size: ::embedded_graphics::geometry::Size::new(6u32, 12u32),
    character_spacing: 0u32,
    baseline: 9u32,
    underline: ::embedded_graphics::mono_font::DecorationDimensions::new(11u32, 1u32),
    strikethrough: ::embedded_graphics::mono_font::DecorationDimensions::new(6u32, 1u32),
};

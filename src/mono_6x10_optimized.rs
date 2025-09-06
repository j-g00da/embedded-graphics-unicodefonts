pub const MONO_6X10_OPTIMIZED: ::embedded_graphics::mono_font::MonoFont = ::embedded_graphics::mono_font::MonoFont {
    image: ::embedded_graphics::image::ImageRaw::new(
        include_bytes!("raw/mono_6x10_optimized.data"),
        96u32,
    ),
    glyph_mapping: &::embedded_graphics::mono_font::mapping::StrGlyphMapping::new(
        "\0 ~\0\u{a0}ÿ\0₣₧\0₫₯\0─▟☀\0☺♂\0♠♦\0♪♯\0⠀⣿",
        31usize,
    ),
    character_size: ::embedded_graphics::geometry::Size::new(6u32, 10u32),
    character_spacing: 0u32,
    baseline: 7u32,
    underline: ::embedded_graphics::mono_font::DecorationDimensions::new(9u32, 1u32),
    strikethrough: ::embedded_graphics::mono_font::DecorationDimensions::new(5u32, 1u32),
};

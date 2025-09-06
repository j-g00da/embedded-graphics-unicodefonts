use crate::atlas::FontAtlas;

/// **Danger**: leaking [`FontAtlas<'static>`] for the lifetime of the program
pub fn mono_6x12_optimized_atlas() -> ::embedded_graphics::mono_font::MonoFont<'static> {
    let atlas = FontAtlas::from_mapping_str(22, "\0 ~\0\u{a0}ÿ\0₠₵\0─▟\0☀⚜\0⚠⚼⛀⛁⛂⛃\0⠀⣿\0⬀⭌")
        .leak();

    ::embedded_graphics::mono_font::MonoFont {
        image: ::embedded_graphics::image::ImageRaw::new(
            include_bytes!("raw/mono_6x12_optimized.data"),
            96u32,
        ),
        glyph_mapping: atlas,
        character_size: ::embedded_graphics::geometry::Size::new(6u32, 12u32),
        character_spacing: 0u32,
        baseline: 9u32,
        underline: ::embedded_graphics::mono_font::DecorationDimensions::new(11u32, 1u32),
        strikethrough: ::embedded_graphics::mono_font::DecorationDimensions::new(6u32, 1u32),
    }
}
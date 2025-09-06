use crate::atlas::FontAtlas;

/// **Danger**: leaking [`FontAtlas<'static>`] for the lifetime of the program
pub fn mono_8x13_italic_atlas() -> ::embedded_graphics::mono_font::MonoFont<'static> {
    let atlas = FontAtlas::from_mapping_str(8, "\0 ~\0\u{a0}ȟ\0Ȣȳ\0ɐʭ\0ʰˮʹ͵ͺ;΄΅Ά·ΈΉΊΌ\0ΎΡ\0Σώ\0ϐϗϚϛϜϝϰϱϲϳϴϵ\0ЀѿҐґ\0ก\u{e3a}\0฿๛\0ḀẛỲỳ\0\u{2000}\u{200a}\0‐‧\0\u{202f}⁆⁈⁉⁊⁋⁌⁍₠\0₧₯ℂ℃℄℉ℍℎℏ\0ℓℚℝ™Ω℧\0⅓Ↄ←↑→↓↔↕−≠≤≥␉␊␋␌␍␎␏␛␤♪ﬀﬁﬂﬃﬄﬅﬆ�")
        .leak();

    ::embedded_graphics::mono_font::MonoFont {
        image: ::embedded_graphics::image::ImageRaw::new(
            include_bytes!("raw/mono_8x13_italic.data"),
            128u32,
        ),
        glyph_mapping: atlas,
        character_size: ::embedded_graphics::geometry::Size::new(8u32, 13u32),
        character_spacing: 0u32,
        baseline: 10u32,
        underline: ::embedded_graphics::mono_font::DecorationDimensions::new(12u32, 1u32),
        strikethrough: ::embedded_graphics::mono_font::DecorationDimensions::new(6u32, 1u32),
    }
}
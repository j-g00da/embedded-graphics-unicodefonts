use crate::atlas::FontAtlas;

/// **Danger**: leaking [`FontAtlas<'static>`] for the lifetime of the program
pub fn mono_6x13_italic_atlas() -> ::embedded_graphics::mono_font::MonoFont<'static> {
    let atlas = FontAtlas::from_mapping_str(11, "\0 ~\0\u{a0}ſƏƑƒ\0Ǻțə˄˅ˆˇˉ˘˙˚˛˜˝\0\u{300}\u{319}ʹ͵ͺ;΄΅Ά·ΈΉΊΌ\0ΎΡ\0Σώϑϕϖϱϲϳϴϵ\0ЀѿҐґ\0ḀḏḞḟṀṁṂṃṖṗṘṙṠṡṪṫ\0ẀẛỲỳ\0\u{2000}\u{200a}\0‐‧\0\u{202f}⁆⁈⁉⁊⁋⁌⁍€№™Ω℧⅛⅜⅝⅞←↑→↓−≠≤≥␉␊␋␌␍␤♪ﬀﬁﬂﬃﬄﬅﬆ\u{fe20}\u{fe21}\u{fe22}\u{fe23}�")
        .leak();

    ::embedded_graphics::mono_font::MonoFont {
        image: ::embedded_graphics::image::ImageRaw::new(
            include_bytes!("raw/mono_6x13_italic.data"),
            96u32,
        ),
        glyph_mapping: atlas,
        character_size: ::embedded_graphics::geometry::Size::new(6u32, 13u32),
        character_spacing: 0u32,
        baseline: 10u32,
        underline: ::embedded_graphics::mono_font::DecorationDimensions::new(12u32, 1u32),
        strikethrough: ::embedded_graphics::mono_font::DecorationDimensions::new(6u32, 1u32),
    }
}
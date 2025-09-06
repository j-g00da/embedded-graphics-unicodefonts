use crate::atlas::FontAtlas;

/// **Danger**: leaking [`FontAtlas<'static>`] for the lifetime of the program
pub fn mono_7x13_italic_atlas() -> ::embedded_graphics::mono_font::MonoFont<'static> {
    let atlas = FontAtlas::from_mapping_str(8, "\0 ~\0\u{a0}ſƏƐƑƒǺǻǼǽǾǿȘșȚțə\0˄ˋ˘˙˚˛˜˝\0\u{300}\u{311}\u{323}\u{324}\u{340}\u{341}ʹ͵ͺ;΄΅Ά·ΈΉΊΌ\0ΎΡ\0Σώ\0ϐϗϚϛϜϝϞϟϰϱϲϳϴϵ\0Ѐѥ\0ѮѿҐґ\0ก\u{e3a}\0฿๛ḂḃḊḋḌḍḎḏḞḟḢḣḤḥ\0ṀṇṖṗṘṙṠṡṢṣṪṫẀẁẂẃẄẅỲỳ\0\u{2000}\u{200a}\0‐‧\0\u{202f}›‼‽‾‿⁀⁂⁃⁄⁅⁆⁈⁉⁊⁋⁌⁍₨€ℂ℃℄℉ℍℎℏℓ№ℙℚℝ™Ω℧⅛⅜⅝⅞←↑→↓↔↕−≠≤≥␈␉␊␋␌␍␤♩♪ﬀﬁﬂﬃﬄﬅﬆ\u{fe20}\u{fe21}\u{fe22}\u{fe23}�")
        .leak();

    ::embedded_graphics::mono_font::MonoFont {
        image: ::embedded_graphics::image::ImageRaw::new(
            include_bytes!("raw/mono_7x13_italic.data"),
            112u32,
        ),
        glyph_mapping: atlas,
        character_size: ::embedded_graphics::geometry::Size::new(7u32, 13u32),
        character_spacing: 0u32,
        baseline: 10u32,
        underline: ::embedded_graphics::mono_font::DecorationDimensions::new(12u32, 1u32),
        strikethrough: ::embedded_graphics::mono_font::DecorationDimensions::new(6u32, 1u32),
    }
}
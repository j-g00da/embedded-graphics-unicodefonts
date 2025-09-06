use crate::atlas::FontAtlas;

/// **Danger**: leaking [`FontAtlas<'static>`] for the lifetime of the program
pub fn mono_4x6_atlas() -> ::embedded_graphics::mono_font::MonoFont<'static> {
    let atlas = FontAtlas::from_mapping_str(12, "\0 ~\0\u{a0}ſƏƒȘșȚțəˆˇˉ˘˙˚˛˜˝ʹ͵ͺ;΄΅Ά·ΈΉΊΌ\0ΎΡ\0Σώ\0ЁЌ\0Ўя\0ёќўџҐґҒғ\0אתḂḃḊḋḞḟṀṁṖṗṠṡṪṫẀẁẂẃẄẅỲỳ\0‐‧‰‹›‾ⁿ₧€№™Ω℧⅛⅜⅝⅞←↑→↓↔↕\0∀≳\0≶⊋⌐⌠⌡⎺⎻⎼⎽␉␊␋␌␍␣␤\0─▕■□◆♠♣♥♦♩♪♫♬♭♮♯�")
        .leak();

    ::embedded_graphics::mono_font::MonoFont {
        image: ::embedded_graphics::image::ImageRaw::new(
            include_bytes!("raw/mono_4x6.data"),
            64u32,
        ),
        glyph_mapping: atlas,
        character_size: ::embedded_graphics::geometry::Size::new(4u32, 6u32),
        character_spacing: 0u32,
        baseline: 4u32,
        underline: ::embedded_graphics::mono_font::DecorationDimensions::new(6u32, 1u32),
        strikethrough: ::embedded_graphics::mono_font::DecorationDimensions::new(3u32, 1u32),
    }
}
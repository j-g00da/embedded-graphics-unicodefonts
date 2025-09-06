use crate::atlas::FontAtlas;

/// **Danger**: leaking [`FontAtlas<'static>`] for the lifetime of the program
pub fn mono_9x18_bold_atlas() -> ::embedded_graphics::mono_font::MonoFont<'static> {
    let atlas = FontAtlas::from_mapping_str(18, "\0 ~\0\u{a0}ſƏƒƠơƯưȘșȚțəʼʽˆˇ˘˙˚˛˜˝\0\u{300}\u{311}\u{323}\u{324}\u{325}\u{327}\u{328}\u{340}\u{341}ʹ͵ͺ;΄΅Ά·ΈΉΊΌ\0ΎΡ\0Σώ\0ЀџҐґ\0את\0ก\u{e3a}\0฿๛ḂḃḊḋḞḟṀṁṖṗṠṡṪṫẀẁẂẃẄẅỲỳ\0‐‧‰′″‴‵‶‷‹›‼‾⁄₫€₯№™Ω⅛⅜⅝⅞←↑→↓∀∃−≠≤≥⎺⎻⎼⎽␉␊␋␌␍␤─│┌┐└┘├┤┬┴┼╭╮╯╰▒▮◆♪�")
        .leak();

    ::embedded_graphics::mono_font::MonoFont {
        image: ::embedded_graphics::image::ImageRaw::new(
            include_bytes!("raw/mono_9x18_bold.data"),
            144u32,
        ),
        glyph_mapping: atlas,
        character_size: ::embedded_graphics::geometry::Size::new(9u32, 18u32),
        character_spacing: 0u32,
        baseline: 13u32,
        underline: ::embedded_graphics::mono_font::DecorationDimensions::new(15u32, 1u32),
        strikethrough: ::embedded_graphics::mono_font::DecorationDimensions::new(9u32, 1u32),
    }
}
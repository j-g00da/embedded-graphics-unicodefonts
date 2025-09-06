use crate::atlas::FontAtlas;

/// **Danger**: leaking [`FontAtlas<'static>`] for the lifetime of the program
pub fn mono_7x14_bold_atlas() -> ::embedded_graphics::mono_font::MonoFont<'static> {
    let atlas = FontAtlas::from_mapping_str(8, "\0 ~\0\u{a0}ſƏƒƠơƯưȘșȚțəʼʽˆˇ˘˙˚˛˜˝\0\u{300}\u{30f}\u{311}\u{323}\u{324}\u{340}\u{341}ʹ͵ͺ;΄΅Ά·ΈΉΊΌ\0ΎΡ\0Σώ\0ϐϗ\0Ϛϡϰϱϲϳϴϵ\0ЁЌ\0Ўя\0ёќўџҐґ־׀׃\0אתװױײ׳״\0ก\u{e3a}\0฿๛ḂḃḊḋḞḟṀṁṄṅṖṗṠṡṪṫẀẁẂẃẄẅỲỳ\0ἀἕἘἙἚἛἜἝ\0ἠὅὈὉὊὋὌὍ\0ὐὗὙὛὝ\0Ὗώ\0ᾀᾴ\0ᾶῄ\0ῆΐῖῗῘῙῚΊ\0῝`ῲῳῴ\0ῶ῾\0‐•…‰‱‹›‾₫€₯№™Ω⅛⅜⅝⅞←↑→↓−≠≤≥⎺⎻⎼⎽␉␊␋␌␍␤─│┌┐└┘├┤┬┴┼╭╮╯╰╱╲╳▒▮◆♪�")
        .leak();

    ::embedded_graphics::mono_font::MonoFont {
        image: ::embedded_graphics::image::ImageRaw::new(
            include_bytes!("raw/mono_7x14_bold.data"),
            112u32,
        ),
        glyph_mapping: atlas,
        character_size: ::embedded_graphics::geometry::Size::new(7u32, 14u32),
        character_spacing: 0u32,
        baseline: 11u32,
        underline: ::embedded_graphics::mono_font::DecorationDimensions::new(13u32, 1u32),
        strikethrough: ::embedded_graphics::mono_font::DecorationDimensions::new(7u32, 1u32),
    }
}
use crate::atlas::FontAtlas;

/// **Danger**: leaking [`FontAtlas<'static>`] for the lifetime of the program
pub fn mono_6x13_bold_atlas() -> ::embedded_graphics::mono_font::MonoFont<'static> {
    let atlas = FontAtlas::from_mapping_str(8, "\0 ~\0\u{a0}ƔƠơƯưǀǁǂǃ\0ǍǔǦǧǨǩȘșȚțəʼʽˆˇ˘˙˚˛˜˝\0\u{300}\u{345}ʹ͵ͺ;΄΅Ά·ΈΉΊΌ\0ΎΡ\0Σώ\0ϐϗ\0Ϛϡϰϱϲϳϴϵ\0ЁЌ\0Ўя\0ёќ\0ўѿҐґ\0\u{591}\u{59a}\u{59c}\u{59d}\u{59e}\u{59f}\u{5a0}\u{5a1}\u{5a3}\u{5a4}\u{5a5}\u{5a6}\u{5a8}\u{5a9}\u{5aa}\u{5ab}\u{5ac}\u{5ad}\u{5af}\u{5b0}\0\u{5b2}\u{5b9}\0\u{5bb}\u{5c4}\0אתװױײ׳״\0ḀẛỲỳ\0ἀἕἘἙἚἛἜἝ\0ἠὅὈὉὊὋὌὍ\0ὐὗὙὛὝ\0Ὗώ\0ᾀᾴ\0ᾶῄ\0ῆΐῖῗῘῙῚΊ\0῝`ῲῳῴ\0ῶ῾\0‐•…‰‹›‾⁰⁴⁵⁶⁷⁸⁹\0₀₉₫€₯№™Ω⅛⅜⅝⅞←↑→↓∀∁∂∃∈∋−∓∧∨∩∪≠≤≥⊂⊃⊄⊅⋅⋮⋯⋰⋱⌈⌉⌊⌋⎺⎻⎼⎽␈␉␊␋␌␍␠␢␣␤\0⑀⑊─│┌┐└┘├┤┬┴┼\0╭╷▒▮◆♩♪♫♬♭♮♯⟨⟩�")
        .leak();

    ::embedded_graphics::mono_font::MonoFont {
        image: ::embedded_graphics::image::ImageRaw::new(
            include_bytes!("raw/mono_6x13_bold.data"),
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
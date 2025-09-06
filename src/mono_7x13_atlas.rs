use crate::atlas::FontAtlas;

/// **Danger**: leaking [`FontAtlas<'static>`] for the lifetime of the program
pub fn mono_7x13_atlas() -> ::embedded_graphics::mono_font::MonoFont<'static> {
    let atlas = FontAtlas::from_mapping_str(8, "\0 ~\0\u{a0}ȟ\0Ȣȳ\0ɐʭ\0ʰˮ\0\u{300}\u{311}\0\u{323}\u{333}\u{338}\u{340}\u{341}\u{360}\u{361}ʹ͵ͺ;΄΅Ά·ΈΉΊΌ\0ΎΡ\0Σώ\0ϐϗ\0Ϛϵ\0Ѐ\u{486}\u{488}\u{489}\0ҌӄӇӈӋӌ\0ӐӵӸӹ\0ԱՖՙ՚՛՜՝՞՟\0աև։֊־׀׃\0אתװױײ׳״\0ก\u{e3a}\0฿๛\0ႠჅ\0აჶ჻ᚠᚢᚣᚦᚩᚪᚫᚱᚳᚷᚸᚹᚻᚾᛁᛄᛇᛈᛉᛋᛏᛒᛖᛗᛚᛝᛞᛟᛠᛡᛢᛥ᛫᛬᛭\0Ḁẛ\0Ạỹ\0ἀἕἘἙἚἛἜἝ\0ἠὅὈὉὊὋὌὍ\0ὐὗὙὛὝ\0Ὗώ\0ᾀᾴ\0ᾶῄ\0ῆΐῖῗῘῙῚΊ\0῝`ῲῳῴ\0ῶ῾\0\u{2000}\u{200a}\0‐‧\0\u{202f}⁍⁗⁰ⁱ\0⁴₎\0₠₯\0\u{20d0}\u{20e3}\0℀℺\0⅓Ↄ\0←⇳\0∀⌀⌂⌈⌉⌊⌋⌐⌠⌡⌢⌣\0⎛⎽\0␀␦\0⑀⑊\0─◷\0☀☓\0☙♱⟦⟧⟨⟩⟪⟫\0⟵⣿⨀⨁⨂⨃⨄⨅⨆⨉⨝⨿〿ﬀﬁﬂﬃﬄﬅﬆײַﬠ\0﬩זּטּיּךּכּלּמּנּסּףּפּ\0צּﭏ\u{fe20}\u{fe21}\u{fe22}\u{fe23}�")
        .leak();

    ::embedded_graphics::mono_font::MonoFont {
        image: ::embedded_graphics::image::ImageRaw::new(
            include_bytes!("raw/mono_7x13.data"),
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
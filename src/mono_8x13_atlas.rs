use crate::atlas::FontAtlas;

/// **Danger**: leaking [`FontAtlas<'static>`] for the lifetime of the program
pub fn mono_8x13_atlas() -> ::embedded_graphics::mono_font::MonoFont<'static> {
    let atlas = FontAtlas::from_mapping_str(8, "\0 ~\0\u{a0}Ƞ\0Ȣȳ\0ɐʭ\0ʰˮ\0\u{300}\u{34f}\0\u{360}\u{36f}ʹ͵ͺ;΄΅Ά·ΈΉΊΌ\0ΎΡ\0Σώ\0ϐ϶\0Ѐ\u{486}\0\u{488}ӎ\0ӐӵӸӹ\0Ԁԏ\0ԱՖՙ՚՛՜՝՞՟\0աև։֊\0\u{5b0}\u{5b9}\0\u{5bb}\u{5c4}\0אתװױײ׳״\0ก\u{e3a}\0฿๛\0ႠჅ\0აჸ჻\0ᚠᛰ\0Ḁẛ\0Ạỹ\0ἀἕἘἙἚἛἜἝ\0ἠὅὈὉὊὋὌὍ\0ὐὗὙὛὝ\0Ὗώ\0ᾀᾴ\0ᾶῄ\0ῆΐῖῗῘῙῚΊ\0῝`ῲῳῴ\0ῶ῾\0\u{2000}\u{200a}\0‐‧\0\u{202f}⁒⁗\u{205f}\u{2060}\u{2061}\u{2062}\u{2063}\0\u{206a}ⁱ\0⁴₎\0₠₱\0\u{20d0}\u{20ea}\0℀℺\0ℽ⅋\0⅓Ↄ\0←⌨\0⌫⏎\0␀␦\0⑀⑊\0─☓☖☗\0☙♽\0⚀⚉⟕⟖⟗⟦⟧⟨⟩⟪⟫⟰⟱\0⟵⣿\0⨀⨉⨝⨿〿ﬀﬁﬂﬃﬄﬅﬆﬓﬔﬕﬖﬗ\0יִזּטּיּךּכּלּמּנּסּףּפּ\0צּﭏ\u{fe20}\u{fe21}\u{fe22}\u{fe23}\0｡\u{ff9f}�")
        .leak();

    ::embedded_graphics::mono_font::MonoFont {
        image: ::embedded_graphics::image::ImageRaw::new(
            include_bytes!("raw/mono_8x13.data"),
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
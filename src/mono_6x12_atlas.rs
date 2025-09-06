use crate::atlas::FontAtlas;

/// **Danger**: leaking [`FontAtlas<'static>`] for the lifetime of the program
pub fn mono_6x12_atlas() -> ::embedded_graphics::mono_font::MonoFont<'static> {
    let atlas = FontAtlas::from_mapping_str(8, "\0 ~\0\u{a0}ͷͺͻͼͽ;΄΅Ά·ΈΉΊΌ\0ΎΡ\0Σԣ\0ԱՖՙ՚՛՜՝՞՟\0աև։֊\0\u{591}\u{5c7}\0אתװױײ׳״\0\u{1680}᚜\0ᚠᛰḂḃḄḅḆḇ\0ḊḓḞḟḰḱḲḳḴḵḾḿṀṁṂṃṔṕṖṗ\0Ṡṱ\0Ẁẏ\0Ỳỹ\0ἀἕἘἙἚἛἜἝ\0ἠὅὈὉὊὋὌὍ\0ὐὗὙὛὝ\0Ὗώ\0ᾀᾴ\0ᾶῄ\0ῆΐῖῗῘῙῚΊ\0῝`ῲῳῴ\0ῶ῾\0‐‧\0‰⁞⁰ⁱ\0⁴₎ₐₑₒₓₔ\0₠₵\0\u{20d0}\u{20f0}\0℀⅏\0⅓ↈ\0←⌨\0⌫⏧\0␀␦\0⑀⑊\0①☓☖☗\0☙⚜\0⚠⚼⛀⛁⛂⛃✁✂✃✄✆✇✈✉\0✌✧\0✩❋❍❏❐❑❒❖❘❙❚❛❜❝❞\0❡➔\0➘➯\0➱➾\0⟀⟊⟌\0⟐⭌\0ⱠⱯ\0ⱱⱽ\0\u{2de0}\u{2dff}ﬀﬁﬂﬃﬄﬅﬆ\u{fe20}\u{fe21}\u{fe22}\u{fe23}�")
        .leak();

    ::embedded_graphics::mono_font::MonoFont {
        image: ::embedded_graphics::image::ImageRaw::new(
            include_bytes!("raw/mono_6x12.data"),
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
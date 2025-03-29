pub const MONO_6X12: ::embedded_graphics::mono_font::MonoFont = ::embedded_graphics::mono_font::MonoFont {
    image: ::embedded_graphics::image::ImageRaw::new(
        include_bytes!("raw/mono_6x12.data"),
        96u32,
    ),
    glyph_mapping: &::embedded_graphics::mono_font::mapping::StrGlyphMapping::new(
        "⬚\0\u{20}~\0\u{a0}ͷ\0ͺ;\0΄ΊΌ\0ΎΡ\0Σԣ\0ԱՖ\0ՙ՟\0աև։֊\0\u{591}\u{5c7}\0את\0װ״\0\u{1680}᚜\0ᚠᛰ\0Ḃḇ\0ḊḓḞḟ\0Ḱḵ\0Ḿṃ\0Ṕṗ\0Ṡṱ\0Ẁẏ\0Ỳỹ\0ἀἕ\0ἘἝ\0ἠὅ\0ὈὍ\0ὐὗὙὛὝ\0Ὗώ\0ᾀᾴ\0ᾶῄ\0ῆΐ\0ῖΊ\0῝`\0ῲῴ\0ῶ῾\0‐‧\0‰⁞⁰ⁱ\0⁴₎\0ₐₔ\0₠₵\0\u{20d0}\u{20f0}\0℀⅏\0⅓ↈ\0←⌨\0⌫⏧\0␀␦\0⑀⑊\0①☓☖☗\0☙⚜\0⚠⚼\0⛀⛃\0✁✄\0✆✉\0✌✧\0✩❋❍\0❏❒❖\0❘❞\0❡➔\0➘➯\0➱➾\0⟀⟊⟌\0⟐⭌\0ⱠⱯ\0ⱱⱽ\0\u{2de0}\u{2dff}\0ﬀﬆ\0\u{fe20}\u{fe23}�",
        4530usize,
    ),
    character_size: ::embedded_graphics::geometry::Size::new(6u32, 12u32),
    character_spacing: 0u32,
    baseline: 9u32,
    underline: ::embedded_graphics::mono_font::DecorationDimensions::new(11u32, 1u32),
    strikethrough: ::embedded_graphics::mono_font::DecorationDimensions::new(6u32, 1u32),
};

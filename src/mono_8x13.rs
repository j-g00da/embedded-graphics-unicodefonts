pub const MONO_8X13: ::embedded_graphics::mono_font::MonoFont = ::embedded_graphics::mono_font::MonoFont {
    image: ::embedded_graphics::image::ImageRaw::new(
        include_bytes!("raw/mono_8x13.data"),
        128u32,
    ),
    glyph_mapping: &::embedded_graphics::mono_font::mapping::StrGlyphMapping::new(
        "⬚\0\u{20}~\0\u{a0}Ƞ\0Ȣȳ\0ɐʭ\0ʰˮ\0\u{300}\u{34f}\0\u{360}\u{36f}ʹ͵ͺ;\0΄ΊΌ\0ΎΡ\0Σώ\0ϐ϶\0Ѐ\u{486}\0\u{488}ӎ\0ӐӵӸӹ\0Ԁԏ\0ԱՖ\0ՙ՟\0աև։֊\0\u{5b0}\u{5b9}\0\u{5bb}\u{5c4}\0את\0װ״\0ก\u{e3a}\0฿๛\0ႠჅ\0აჸ჻\0ᚠᛰ\0Ḁẛ\0Ạỹ\0ἀἕ\0ἘἝ\0ἠὅ\0ὈὍ\0ὐὗὙὛὝ\0Ὗώ\0ᾀᾴ\0ᾶῄ\0ῆΐ\0ῖΊ\0῝`\0ῲῴ\0ῶ῾\0\u{2000}\u{200a}\0‐‧\0\u{202f}⁒⁗\0\u{205f}\u{2063}\0\u{206a}ⁱ\0⁴₎\0₠₱\0\u{20d0}\u{20ea}\0℀℺\0ℽ⅋\0⅓Ↄ\0←⌨\0⌫⏎\0␀␦\0⑀⑊\0─☓☖☗\0☙♽\0⚀⚉\0⟕⟗\0⟦⟫⟰⟱\0⟵⣿\0⨀⨉⨝⨿〿\0ﬀﬆ\0ﬓﬗ\0יִזּ\0טּלּמּנּסּףּפּ\0צּﭏ\0\u{fe20}\u{fe23}\0｡\u{ff9f}�",
        3702usize,
    ),
    character_size: ::embedded_graphics::geometry::Size::new(8u32, 13u32),
    character_spacing: 0u32,
    baseline: 10u32,
    underline: ::embedded_graphics::mono_font::DecorationDimensions::new(12u32, 1u32),
    strikethrough: ::embedded_graphics::mono_font::DecorationDimensions::new(6u32, 1u32),
};

pub const MONO_7X13_ITALIC: ::embedded_graphics::mono_font::MonoFont = ::embedded_graphics::mono_font::MonoFont {
    image: ::embedded_graphics::image::ImageRaw::new(
        include_bytes!("raw/mono_7x13o.data"),
        112u32,
    ),
    glyph_mapping: &::embedded_graphics::mono_font::mapping::StrGlyphMapping::new(
        "⬚\0\u{20}~\0\u{a0}ſ\0Əƒ\0Ǻǿ\0Șțə\0˄ˋ\0˘˝\0\u{300}\u{311}\u{323}\u{324}\u{340}\u{341}ʹ͵ͺ;\0΄ΊΌ\0ΎΡ\0Σώ\0ϐϗ\0Ϛϟ\0ϰϵ\0Ѐѥ\0ѮѿҐґ\0ก\u{e3a}\0฿๛Ḃḃ\0ḊḏḞḟ\0Ḣḥ\0Ṁṇ\0Ṗṙ\0ṠṣṪṫ\0ẀẅỲỳ\0\u{2000}\u{200a}\0‐‧\0\u{202f}›\0‼⁀\0⁂⁆\0⁈⁍₨€\0ℂ℄℉\0ℍℏℓ№ℙℚℝ™Ω℧\0⅛⅞\0←↕−≠≤≥\0␈␍␤♩♪\0ﬀﬆ\0\u{fe20}\u{fe23}�",
        830usize,
    ),
    character_size: ::embedded_graphics::geometry::Size::new(7u32, 13u32),
    character_spacing: 0u32,
    baseline: 10u32,
    underline: ::embedded_graphics::mono_font::DecorationDimensions::new(12u32, 1u32),
    strikethrough: ::embedded_graphics::mono_font::DecorationDimensions::new(6u32, 1u32),
};

pub const MONO_6X13_BOLD: ::embedded_graphics::mono_font::MonoFont = ::embedded_graphics::mono_font::MonoFont {
    image: ::embedded_graphics::image::ImageRaw::new(
        include_bytes!("raw/mono_6x13B.data"),
        96u32,
    ),
    glyph_mapping: &::embedded_graphics::mono_font::mapping::StrGlyphMapping::new(
        "⬚\0\u{20}~\0\u{a0}ƔƠơƯư\0ǀǃ\0Ǎǔ\0Ǧǩ\0Șțəʼʽˆˇ\0˘˝\0\u{300}\u{345}ʹ͵ͺ;\0΄ΊΌ\0ΎΡ\0Σώ\0ϐϗ\0Ϛϡ\0ϰϵ\0ЁЌ\0Ўя\0ёќ\0ўѿҐґ\0\u{591}\u{59a}\0\u{59c}\u{5a1}\0\u{5a3}\u{5a6}\0\u{5a8}\u{5ad}\u{5af}\u{5b0}\0\u{5b2}\u{5b9}\0\u{5bb}\u{5c4}\0את\0װ״\0ḀẛỲỳ\0ἀἕ\0ἘἝ\0ἠὅ\0ὈὍ\0ὐὗὙὛὝ\0Ὗώ\0ᾀᾴ\0ᾶῄ\0ῆΐ\0ῖΊ\0῝`\0ῲῴ\0ῶ῾\0‐•…‰‹›‾⁰\0⁴⁹\0₀₉₫€₯№™Ω\0⅛⅞\0←↓\0∀∃∈∋−∓\0∧∪≠≤≥\0⊂⊅⋅\0⋮⋱\0⌈⌋\0⎺⎽\0␈␍␠\0␢␤\0⑀⑊─│┌┐└┘├┤┬┴┼\0╭╷▒▮◆\0♩♯⟨⟩�",
        1281usize,
    ),
    character_size: ::embedded_graphics::geometry::Size::new(6u32, 13u32),
    character_spacing: 0u32,
    baseline: 10u32,
    underline: ::embedded_graphics::mono_font::DecorationDimensions::new(12u32, 1u32),
    strikethrough: ::embedded_graphics::mono_font::DecorationDimensions::new(6u32, 1u32),
};

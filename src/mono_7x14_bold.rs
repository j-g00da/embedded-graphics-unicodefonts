pub const MONO_7X14_BOLD: ::embedded_graphics::mono_font::MonoFont = ::embedded_graphics::mono_font::MonoFont {
    image: ::embedded_graphics::image::ImageRaw::new(
        include_bytes!("raw/mono_7x14B.data"),
        112u32,
    ),
    glyph_mapping: &::embedded_graphics::mono_font::mapping::StrGlyphMapping::new(
        "⬚\0\u{20}~\0\u{a0}ſƏƒƠơƯư\0Șțəʼʽˆˇ\0˘˝\0\u{300}\u{30f}\u{311}\u{323}\u{324}\u{340}\u{341}ʹ͵ͺ;\0΄ΊΌ\0ΎΡ\0Σώ\0ϐϗ\0Ϛϡ\0ϰϵ\0ЁЌ\0Ўя\0ёќўџҐґ־׀׃\0את\0װ״\0ก\u{e3a}\0฿๛ḂḃḊḋḞḟṀṁṄṅṖṗṠṡṪṫ\0ẀẅỲỳ\0ἀἕ\0ἘἝ\0ἠὅ\0ὈὍ\0ὐὗὙὛὝ\0Ὗώ\0ᾀᾴ\0ᾶῄ\0ῆΐ\0ῖΊ\0῝`\0ῲῴ\0ῶ῾\0‐•…‰‱‹›‾₫€₯№™Ω\0⅛⅞\0←↓−≠≤≥\0⎺⎽\0␉␍␤─│┌┐└┘├┤┬┴┼\0╭╳▒▮◆♪�",
        1008usize,
    ),
    character_size: ::embedded_graphics::geometry::Size::new(7u32, 14u32),
    character_spacing: 0u32,
    baseline: 11u32,
    underline: ::embedded_graphics::mono_font::DecorationDimensions::new(13u32, 1u32),
    strikethrough: ::embedded_graphics::mono_font::DecorationDimensions::new(7u32, 1u32),
};

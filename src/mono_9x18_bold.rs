pub const MONO_9X18_BOLD: ::embedded_graphics::mono_font::MonoFont = ::embedded_graphics::mono_font::MonoFont {
    image: ::embedded_graphics::image::ImageRaw::new(
        include_bytes!("raw/mono_9x18B.data"),
        144u32,
    ),
    glyph_mapping: &::embedded_graphics::mono_font::mapping::StrGlyphMapping::new(
        "⬚\0\u{20}~\0\u{a0}ſƏƒƠơƯư\0Șțəʼʽˆˇ\0˘˝\0\u{300}\u{311}\0\u{323}\u{325}\u{327}\u{328}\u{340}\u{341}ʹ͵ͺ;\0΄ΊΌ\0ΎΡ\0Σώ\0ЀџҐґ\0את\0ก\u{e3a}\0฿๛ḂḃḊḋḞḟṀṁṖṗṠṡṪṫ\0ẀẅỲỳ\0‐‧‰\0′‷‹›‼‾⁄₫€₯№™Ω\0⅛⅞\0←↓∀∃−≠≤≥\0⎺⎽\0␉␍␤─│┌┐└┘├┤┬┴┼\0╭╰▒▮◆♪�",
        761usize,
    ),
    character_size: ::embedded_graphics::geometry::Size::new(9u32, 18u32),
    character_spacing: 0u32,
    baseline: 13u32,
    underline: ::embedded_graphics::mono_font::DecorationDimensions::new(15u32, 1u32),
    strikethrough: ::embedded_graphics::mono_font::DecorationDimensions::new(9u32, 1u32),
};

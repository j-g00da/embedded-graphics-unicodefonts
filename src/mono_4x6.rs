pub const MONO_4X6: ::embedded_graphics::mono_font::MonoFont = ::embedded_graphics::mono_font::MonoFont {
    image: ::embedded_graphics::image::ImageRaw::new(
        include_bytes!("raw/mono_4x6.data"),
        64u32,
    ),
    glyph_mapping: &::embedded_graphics::mono_font::mapping::StrGlyphMapping::new(
        "⬚\0\u{20}~\0\u{a0}ſƏƒ\0Șțəˆˇˉ\0˘˝ʹ͵ͺ;\0΄ΊΌ\0ΎΡ\0Σώ\0ЁЌ\0Ўя\0ёќўџ\0Ґғ\0אתḂḃḊḋḞḟṀṁṖṗṠṡṪṫ\0ẀẅỲỳ\0‐‧‰‹›‾ⁿ₧€№™Ω℧\0⅛⅞\0←↕\0∀≳\0≶⊋⌐⌠⌡\0⎺⎽\0␉␍␣␤\0─▕■□◆♠♣♥♦\0♩♯�",
        918usize,
    ),
    character_size: ::embedded_graphics::geometry::Size::new(4u32, 6u32),
    character_spacing: 0u32,
    baseline: 4u32,
    underline: ::embedded_graphics::mono_font::DecorationDimensions::new(6u32, 1u32),
    strikethrough: ::embedded_graphics::mono_font::DecorationDimensions::new(3u32, 1u32),
};

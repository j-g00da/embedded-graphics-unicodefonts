pub const MONO_6X13_ITALIC: ::embedded_graphics::mono_font::MonoFont = ::embedded_graphics::mono_font::MonoFont {
    image: ::embedded_graphics::image::ImageRaw::new(
        include_bytes!("raw/mono_6x13o.data"),
        96u32,
    ),
    glyph_mapping: &::embedded_graphics::mono_font::mapping::StrGlyphMapping::new(
        "⬚\0\u{20}~\0\u{a0}ſƏƑƒ\0Ǻțə\0˄ˇˉ\0˘˝\0\u{300}\u{319}ʹ͵ͺ;\0΄ΊΌ\0ΎΡ\0Σώϑϕϖ\0ϱϵ\0ЀѿҐґ\0ḀḏḞḟ\0Ṁṃ\0ṖṙṠṡṪṫ\0ẀẛỲỳ\0\u{2000}\u{200a}\0‐‧\0\u{202f}⁆\0⁈⁍€№™Ω℧\0⅛⅞\0←↓−≠≤≥\0␉␍␤♪\0ﬀﬆ\0\u{fe20}\u{fe23}�",
        769usize,
    ),
    character_size: ::embedded_graphics::geometry::Size::new(6u32, 13u32),
    character_spacing: 0u32,
    baseline: 10u32,
    underline: ::embedded_graphics::mono_font::DecorationDimensions::new(12u32, 1u32),
    strikethrough: ::embedded_graphics::mono_font::DecorationDimensions::new(6u32, 1u32),
};

pub const MONO_8X13_ITALIC: ::embedded_graphics::mono_font::MonoFont = ::embedded_graphics::mono_font::MonoFont {
    image: ::embedded_graphics::image::ImageRaw::new(
        include_bytes!("raw/mono_8x13O.data"),
        128u32,
    ),
    glyph_mapping: &::embedded_graphics::mono_font::mapping::StrGlyphMapping::new(
        "⬚\0\u{20}~\0\u{a0}ȟ\0Ȣȳ\0ɐʭ\0ʰˮʹ͵ͺ;\0΄ΊΌ\0ΎΡ\0Σώ\0ϐϗ\0Ϛϝ\0ϰϵ\0ЀѿҐґ\0ก\u{e3a}\0฿๛\0ḀẛỲỳ\0\u{2000}\u{200a}\0‐‧\0\u{202f}⁆\0⁈⁍₠\0₧₯\0ℂ℄℉\0ℍℏ\0ℓℚℝ™Ω℧\0⅓Ↄ\0←↕−≠≤≥\0␉␏␛␤♪\0ﬀﬆ�",
        1294usize,
    ),
    character_size: ::embedded_graphics::geometry::Size::new(8u32, 13u32),
    character_spacing: 0u32,
    baseline: 10u32,
    underline: ::embedded_graphics::mono_font::DecorationDimensions::new(12u32, 1u32),
    strikethrough: ::embedded_graphics::mono_font::DecorationDimensions::new(6u32, 1u32),
};

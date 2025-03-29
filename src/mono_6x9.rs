pub const MONO_6X9: ::embedded_graphics::mono_font::MonoFont = ::embedded_graphics::mono_font::MonoFont {
    image: ::embedded_graphics::image::ImageRaw::new(
        include_bytes!("raw/mono_6x9.data"),
        96u32,
    ),
    glyph_mapping: &::embedded_graphics::mono_font::mapping::StrGlyphMapping::new(
        "⬚\0\u{20}~\0\u{a0}ſƏƒƠơƯưƵƶǑǒǦǧ\0Ǻǿ\0Șțɘə\0ʻʽˆˇˉ\0˘˝\0\u{300}\u{314}\u{323}\u{324}ʹ͵ͺ;\0΄ΊΌ\0ΎΡ\0Σώ\0ϐϗ\0Ϛϵ\0ЀџѢѣ\0Ѱѵ\0ҐғҖҗҚқ\0ҮҳҺһӘәӢӣӨөӮӯ\0את\0װ״ḂḃḊḋḞḟṀṁṖṗṠṡṪṫ\0ẀẅỲỳ\0‐‧‰\0′‷‹›‼‾⁄⁰ⁱ\0⁴₎₣₤₧₫€₯\0\u{20d0}\u{20d7}ℂ℅ℓℕ№ℚℝ™ℤΩ℮\0⅛⅞\0←↕\0↤↨\0⇐⇕\0∀∉∋∌\0∏∓∕\0∘√\0∝∟∡\0∤∫∮\0∴∽≃≅≈≉≙\0≟≥≪≫\0⊂⊋\0⊕⊙\0⊢⊨\0⋀⋃⋅\0⋮⋱⌀⌂\0⌈⌋⌐⌠⌡\0⎺⎽\0␉␍␤\0─┃┌┐└┘├┤┬┴┼\0╌╳\0▀▣\0▪◆◊○●◘◙◦☀\0☹☼\0☿♂\0♠♦\0♩♫⟨⟩\0⠀⣿ﬁﬂ�",
        1295usize,
    ),
    character_size: ::embedded_graphics::geometry::Size::new(6u32, 9u32),
    character_spacing: 0u32,
    baseline: 6u32,
    underline: ::embedded_graphics::mono_font::DecorationDimensions::new(8u32, 1u32),
    strikethrough: ::embedded_graphics::mono_font::DecorationDimensions::new(4u32, 1u32),
};

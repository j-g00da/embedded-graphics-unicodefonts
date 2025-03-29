pub const MONO_5X8: ::embedded_graphics::mono_font::MonoFont = ::embedded_graphics::mono_font::MonoFont {
    image: ::embedded_graphics::image::ImageRaw::new(
        include_bytes!("raw/mono_5x8.data"),
        80u32,
    ),
    glyph_mapping: &::embedded_graphics::mono_font::mapping::StrGlyphMapping::new(
        "⬚\0\u{20}~\0\u{a0}ſƏƒƠơƯưƵƶǑǒǦǧ\0Ǻǿ\0Șțə\0ʻʿˆˇˉ\0˘˝\0\u{300}\u{315}\u{323}\u{324}\u{331}\u{332}\u{338}\u{340}\u{341}ʹ͵ͺ;\0΄ΊΌ\0ΎΡ\0Σώ\0ϑϖϞϟϩ\0ϰϵ\0Ёя\0ёџ\0ҐғҖҗҚқ\0ҮҳҺһӘәӢӣӨөӮӯ\0אתḂḃḊḋḞḟṀṁṖṗṠṡṪṫ\0ẀẅỲỳ\0‐•…‰\0′‶‹›\0‼‾⁄⁰ⁱ\0⁴₎₣₤₧₫€₯\0\u{20d0}\u{20d7}ℂ℅ℓℕ№ℚℝ™ℤΩ℮\0⅛⅞\0←↙\0↤↨\0↰↵\0↼⇃⇋⇌\0⇐⇙\0⇠⇩\0∀∉∋∌\0∎√\0∝∫∮\0∴∽≀≂≃≅\0≈≋\0≘≚\0≟≥≪≫\0⊂⊋\0⊕⊙\0⊞⊨\0⋀⋃⋅⌀⌂\0⌈⌋⌐⌕⌠⌡\0⎺⎽\0␉␍␤\0─▢▧▨\0▪◇\0◊◕\0◘◛\0◢◪\0☥☩\0☰☸\0☺☾\0♀♂\0♠♧\0♩♯⟨⟩\0⠀⣿\0ﬀﬂ�",
        1425usize,
    ),
    character_size: ::embedded_graphics::geometry::Size::new(5u32, 8u32),
    character_spacing: 0u32,
    baseline: 6u32,
    underline: ::embedded_graphics::mono_font::DecorationDimensions::new(8u32, 1u32),
    strikethrough: ::embedded_graphics::mono_font::DecorationDimensions::new(4u32, 1u32),
};

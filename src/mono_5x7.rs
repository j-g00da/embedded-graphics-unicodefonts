pub const MONO_5X7: ::embedded_graphics::mono_font::MonoFont = ::embedded_graphics::mono_font::MonoFont {
    image: ::embedded_graphics::image::ImageRaw::new(
        include_bytes!("raw/mono_5x7.data"),
        80u32,
    ),
    glyph_mapping: &::embedded_graphics::mono_font::mapping::StrGlyphMapping::new(
        "⬚\0\u{20}~\0\u{a0}ȟ\0ɐʨʶʸʹʼʽ\0ˆˉˌː˖\0˘˝\0\u{300}\u{311}\u{323}\u{324}\u{338}\u{340}\u{341}ʹ͵ͺ;\0΄ΊΌ\0ΎΡ\0Σώ\0ϐϗ\0Ϛϡ\0ϰϵ\0ЁЌ\0Ўя\0ёќўџ\0ҐғҖҗҚқ\0ҮҳҺһӘәӢӣӨөӮӯ\0אתᚠᚢᚣᚦ\0ᚩᚫᚱᚳ\0ᚷᚹᚻᚾᛀᛄ\0ᛇᛉᛋᛏᛒᛖᛗᛚ\0ᛝᛤ\0᛫᛭ḂḃḊḋḞḟṀṁṖṗṠṡṪṫ\0ẀẅỲỳ\0ἀἕ\0ἘἝ\0ἠὅ\0ὈὍ\0ὐὗὙὛὝ\0Ὗώ\0ᾀᾴ\0ᾶῄ\0ῆΐ\0ῖΊ\0῝`\0ῲῴ\0ῶ῾\0‐‧‰\0′‷‹›‼‾⁄⁰ⁱ\0⁴₎₣₤₧₫€₯\0\u{20d0}\u{20d7}ℂ℅ℓℕ№ℚℝ™ℤΩ℧℮\0⅛⅞\0←↕\0↤↨⇄⇆⇋⇌\0⇐⇕\0∀∰\0∴∺∼≃≅≈≉≙≚\0≟≥≪≫\0⊂⊋\0⊕⊙\0⊢⊨⋂⋃⋅\0⋮⋱⌀⌂\0⌈⌋⌐⌠⌡\0⎺⎽\0␉␍␣␤\0─□\0▪▮▲△\0▶▽\0◀◆◊○●◘◙◦\0☹☼\0♀♂♠♣♥♦\0♩♯⟨⟩\0⠀⣿ﬁﬂ�",
        1847usize,
    ),
    character_size: ::embedded_graphics::geometry::Size::new(5u32, 7u32),
    character_spacing: 0u32,
    baseline: 5u32,
    underline: ::embedded_graphics::mono_font::DecorationDimensions::new(7u32, 1u32),
    strikethrough: ::embedded_graphics::mono_font::DecorationDimensions::new(3u32, 1u32),
};

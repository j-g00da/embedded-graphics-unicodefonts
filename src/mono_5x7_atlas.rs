use crate::atlas::FontAtlas;

/// **Danger**: leaking [`FontAtlas<'static>`] for the lifetime of the program
pub fn mono_5x7_atlas() -> ::embedded_graphics::mono_font::MonoFont<'static> {
    let atlas = FontAtlas::from_mapping_str(8, "\0 ~\0\u{a0}ȟ\0ɐʨʶʸʹʼʽˆˇˈˉˌː˖˘˙˚˛˜˝\0\u{300}\u{311}\u{323}\u{324}\u{338}\u{340}\u{341}ʹ͵ͺ;΄΅Ά·ΈΉΊΌ\0ΎΡ\0Σώ\0ϐϗ\0Ϛϡϰϱϲϳϴϵ\0ЁЌ\0Ўя\0ёќўџҐґҒғҖҗҚқҮүҰұҲҳҺһӘәӢӣӨөӮӯ\0אתᚠᚢᚣᚦᚩᚪᚫᚱᚳᚷᚸᚹᚻᚾᛀᛄᛇᛈᛉᛋᛏᛒᛖᛗᛚ\0ᛝᛤ᛫᛬᛭ḂḃḊḋḞḟṀṁṖṗṠṡṪṫẀẁẂẃẄẅỲỳ\0ἀἕἘἙἚἛἜἝ\0ἠὅὈὉὊὋὌὍ\0ὐὗὙὛὝ\0Ὗώ\0ᾀᾴ\0ᾶῄ\0ῆΐῖῗῘῙῚΊ\0῝`ῲῳῴ\0ῶ῾\0‐‧‰′″‴‵‶‷‹›‼‾⁄⁰ⁱ\0⁴₎₣₤₧₫€₯\0\u{20d0}\u{20d7}ℂ℅ℓℕ№ℚℝ™ℤΩ℧℮⅛⅜⅝⅞←↑→↓↔↕↤↥↦↧↨⇄⇆⇋⇌⇐⇑⇒⇓⇔⇕\0∀∰∴∵∶∷∸∹∺∼≃≅≈≉≙≚≟≠≡≢≣≤≥≪≫\0⊂⊋⊕⊖⊗⊘⊙⊢⊣⊤⊥⊦⊧⊨⋂⋃⋅⋮⋯⋰⋱⌀⌂⌈⌉⌊⌋⌐⌠⌡⎺⎻⎼⎽␉␊␋␌␍␣␤\0─□▪▫▬▭▮▲△\0▶▽◀◁◂◃◄◅◆◊○●◘◙◦☹☺☻☼♀♁♂♠♣♥♦♩♪♫♬♭♮♯⟨⟩\0⠀⣿ﬁﬂ�")
        .leak();

    ::embedded_graphics::mono_font::MonoFont {
        image: ::embedded_graphics::image::ImageRaw::new(
            include_bytes!("raw/mono_5x7.data"),
            80u32,
        ),
        glyph_mapping: atlas,
        character_size: ::embedded_graphics::geometry::Size::new(5u32, 7u32),
        character_spacing: 0u32,
        baseline: 5u32,
        underline: ::embedded_graphics::mono_font::DecorationDimensions::new(7u32, 1u32),
        strikethrough: ::embedded_graphics::mono_font::DecorationDimensions::new(3u32, 1u32),
    }
}
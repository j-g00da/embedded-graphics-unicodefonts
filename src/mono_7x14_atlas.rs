use crate::atlas::FontAtlas;

/// **Danger**: leaking [`FontAtlas<'static>`] for the lifetime of the program
pub fn mono_7x14_atlas() -> ::embedded_graphics::mono_font::MonoFont<'static> {
    let atlas = FontAtlas::from_mapping_str(8, "\0 ~\0\u{a0}Ƞ\0Ȣȳ\0ɐʭ\0ʰˮ\0\u{300}\u{34e}\0\u{360}\u{36f}ʹ͵ͺ;΄΅Ά·ΈΉΊΌ\0ΎΡ\0Σώ\0ϐ϶\0Ѐ\u{486}\0\u{488}ӎ\0ӐӵӸӹ\0Ԁԏ\0ԱՖՙ՚՛՜՝՞՟\0աև։֊־׀׃\0אתװױײ׳״\0ก\u{e3a}\0฿๛ᚠᚢᚣᚦᚩᚪᚫᚱᚳᚷᚸᚹᚻᚾᛁᛄᛇᛈᛉᛋᛏᛒᛖᛗᛚᛝᛞᛟᛠᛡᛢᛥ᛫᛬᛭\0Ḁẛ\0Ạầ\0Ỳỹ\0ἀἕἘἙἚἛἜἝ\0ἠὅὈὉὊὋὌὍ\0ὐὗὙὛὝ\0Ὗώ\0ᾀᾴ\0ᾶῄ\0ῆΐῖῗῘῙῚΊ\0῝`ῲῳῴ\0ῶ῾\0‐…‰′″‴‵‶‷‹›‼‾⁄⁰ⁱ\0⁴₎₠₢₣₤₦₧₩₪₫€₯\0\u{20d0}\u{20d7}℀℁ℂ℃℅ℓℕ№ℚℝ™ℤΩ℧℮⅛⅜⅝⅞←↑→↓↔↕↤↥↦↧↨⇄⇆⇋⇌⇐⇑⇒⇓⇔⇕\0∀∉∋∌∏∐∑−∓∕∘∙√∝∞∟∡\0∣∫∮∼≃≅≈≉≙≟≠≡≢≣≤≥≪≫\0⊂⊋⊕⊖⊗⊘⊙⊢⊣⊤⊥⊦⊧⊨⋂⋃⋅⋮⋯⋰⋱⌀⌂⌈⌉⌊⌋⌐⌠⌡⎺⎻⎼⎽␀\0␈␏␠␡␣␤\0─□▪▫▬▭▮▯▲△\0▶▽◀◁◂◃◄◅◆◊○●◘◙◦☹☺☻☼♀♁♂♠♣♥♦♩♪♫♬♭♮⟦⟧⟨⟩⟪⟫\0⠀⣿ﬁﬂײַﬠ\0﬩זּטּיּךּכּלּמּנּסּףּפּ\0צּﭏ\0｡\u{ff9f}�")
        .leak();

    ::embedded_graphics::mono_font::MonoFont {
        image: ::embedded_graphics::image::ImageRaw::new(
            include_bytes!("raw/mono_7x14.data"),
            112u32,
        ),
        glyph_mapping: atlas,
        character_size: ::embedded_graphics::geometry::Size::new(7u32, 14u32),
        character_spacing: 0u32,
        baseline: 11u32,
        underline: ::embedded_graphics::mono_font::DecorationDimensions::new(13u32, 1u32),
        strikethrough: ::embedded_graphics::mono_font::DecorationDimensions::new(7u32, 1u32),
    }
}
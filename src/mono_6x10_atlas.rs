use crate::atlas::FontAtlas;

/// **Danger**: leaking [`FontAtlas<'static>`] for the lifetime of the program
pub fn mono_6x10_atlas() -> ::embedded_graphics::mono_font::MonoFont<'static> {
    let atlas = FontAtlas::from_mapping_str(8, "\0 ~\0\u{a0}ſƏƒƠơƯưƵƶǑǒǦǧǺǻǼǽǾǿȘșȚțəʼʽˆˇˉ˖˘˙˚˛˜˝\0\u{300}\u{311}\u{323}\u{324}\u{338}\u{340}\u{341}ʹ͵ͺ;΄΅Ά·ΈΉΊΌ\0ΎΡ\0Σώ\0ϐϗ\0Ϛϡϰϱϲϳϴϵ\0ЁЌ\0Ўя\0ёќўџѢѣ\0ѰѷҐґҒғҖҗҚқҮүҰұҲҳҺһӘәӢӣӨөӮӯ\0אתᚠᚢᚣᚦᚩᚪᚫᚬᚱᚳᚷᚸᚹᚻᚾᛁᛄᛇᛈᛉᛋᛏᛒᛖᛗᛚᛝᛞᛟᛠᛡᛢᛣᛥ᛫᛬᛭ḂḃḄḊḋḞḟṀṁṖṗṠṡṪṫẀẁẂẃẄẅỲỳ\0ἀἕἘἙἚἛἜἝ\0ἠὅὈὉὊὋὌὍ\0ὐὗὙὛὝ\0Ὗώ\0ᾀᾴ\0ᾶῄ\0ῆΐῖῗῘῙῚΊ\0῝`ῲῳῴ\0ῶ῾\0‐‧‰′″‴‵‶‷‹›‼‾⁄⁰ⁱ\0⁴₎₣₤₧₫€₯\0\u{20d0}\u{20d7}ℂ℅ℓℕ№ℚℝ™ℤΩ℧℮⅛⅜⅝⅞\0←↙↤↥↦↧↨⇄⇆⇋⇌⇐⇑⇒⇓⇔⇕\0∀∉∋∌∏∑−∓∕∘∙√∝∞∟∡\0∤∫∮∼≃≅≈≉≙≟≠≡≢≤≥≪≫\0⊂⊋⊕⊗⊤⊥⊦⊧⋀⋁⋂⋃⋅⋮⋯⋰⋱⌀⌂⌈⌉⌊⌋⌐⌠⌡⎺⎻⎼⎽␉␊␋␌␍␤\0─□▪▫▬▮▲△►▻▼▽◄◅◆◊○◌●◘◙◦☀☺☻☼☿♀♁♂♠♡♢♣♤♥♦♪♫♬♭♮♯⟨⟩\0⠀⣿ﬁﬂ�")
        .leak();

    ::embedded_graphics::mono_font::MonoFont {
        image: ::embedded_graphics::image::ImageRaw::new(
            include_bytes!("raw/mono_6x10.data"),
            96u32,
        ),
        glyph_mapping: atlas,
        character_size: ::embedded_graphics::geometry::Size::new(6u32, 10u32),
        character_spacing: 0u32,
        baseline: 7u32,
        underline: ::embedded_graphics::mono_font::DecorationDimensions::new(9u32, 1u32),
        strikethrough: ::embedded_graphics::mono_font::DecorationDimensions::new(5u32, 1u32),
    }
}
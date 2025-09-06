use crate::atlas::FontAtlas;

/// **Danger**: leaking [`FontAtlas<'static>`] for the lifetime of the program
pub fn mono_6x9_atlas() -> ::embedded_graphics::mono_font::MonoFont<'static> {
    let atlas = FontAtlas::from_mapping_str(8, "\0 ~\0\u{a0}ſƏƒƠơƯưƵƶǑǒǦǧǺǻǼǽǾǿȘșȚțɘəʻʼʽˆˇˉ˘˙˚˛˜˝\0\u{300}\u{314}\u{323}\u{324}ʹ͵ͺ;΄΅Ά·ΈΉΊΌ\0ΎΡ\0Σώ\0ϐϗ\0Ϛϵ\0ЀџѢѣѰѱѲѳѴѵҐґҒғҖҗҚқҮүҰұҲҳҺһӘәӢӣӨөӮӯ\0אתװױײ׳״ḂḃḊḋḞḟṀṁṖṗṠṡṪṫẀẁẂẃẄẅỲỳ\0‐‧‰′″‴‵‶‷‹›‼‾⁄⁰ⁱ\0⁴₎₣₤₧₫€₯\0\u{20d0}\u{20d7}ℂ℅ℓℕ№ℚℝ™ℤΩ℮⅛⅜⅝⅞←↑→↓↔↕↤↥↦↧↨⇐⇑⇒⇓⇔⇕\0∀∉∋∌∏∐∑−∓∕∘∙√∝∞∟∡\0∤∫∮\0∴∽≃≅≈≉≙≟≠≡≢≣≤≥≪≫\0⊂⊋⊕⊖⊗⊘⊙⊢⊣⊤⊥⊦⊧⊨⋀⋁⋂⋃⋅⋮⋯⋰⋱⌀⌂⌈⌉⌊⌋⌐⌠⌡⎺⎻⎼⎽␉␊␋␌␍␤─━│┃┌┐└┘├┤┬┴┼\0╌╳\0▀▣\0▪◆◊○●◘◙◦☀☹☺☻☼☿♀♁♂♠♡♢♣♤♥♦♩♪♫⟨⟩\0⠀⣿ﬁﬂ�")
        .leak();

    ::embedded_graphics::mono_font::MonoFont {
        image: ::embedded_graphics::image::ImageRaw::new(
            include_bytes!("raw/mono_6x9.data"),
            96u32,
        ),
        glyph_mapping: atlas,
        character_size: ::embedded_graphics::geometry::Size::new(6u32, 9u32),
        character_spacing: 0u32,
        baseline: 6u32,
        underline: ::embedded_graphics::mono_font::DecorationDimensions::new(8u32, 1u32),
        strikethrough: ::embedded_graphics::mono_font::DecorationDimensions::new(4u32, 1u32),
    }
}
use crate::atlas::FontAtlas;

/// **Danger**: leaking [`FontAtlas<'static>`] for the lifetime of the program
pub fn mono_5x8_atlas() -> ::embedded_graphics::mono_font::MonoFont<'static> {
    let atlas = FontAtlas::from_mapping_str(8, "\0 ~\0\u{a0}ſƏƒƠơƯưƵƶǑǒǦǧǺǻǼǽǾǿȘșȚțəʻʼʽʾʿˆˇˉ˘˙˚˛˜˝\0\u{300}\u{315}\u{323}\u{324}\u{331}\u{332}\u{338}\u{340}\u{341}ʹ͵ͺ;΄΅Ά·ΈΉΊΌ\0ΎΡ\0ΣώϑϒϓϔϕϖϞϟϩϰϱϲϳϴϵ\0Ёя\0ёџҐґҒғҖҗҚқҮүҰұҲҳҺһӘәӢӣӨөӮӯ\0אתḂḃḊḋḞḟṀṁṖṗṠṡṪṫẀẁẂẃẄẅỲỳ\0‐•…‰′″‴‵‶‹›‼‽‾⁄⁰ⁱ\0⁴₎₣₤₧₫€₯\0\u{20d0}\u{20d7}ℂ℅ℓℕ№ℚℝ™ℤΩ℮⅛⅜⅝⅞\0←↙↤↥↦↧↨↰↱↲↳↴↵\0↼⇃⇋⇌\0⇐⇙\0⇠⇩\0∀∉∋∌\0∎√\0∝∫∮\0∴∽≀≂≃≅≈≉≊≋≘≙≚≟≠≡≢≣≤≥≪≫\0⊂⊋⊕⊖⊗⊘⊙\0⊞⊨⋀⋁⋂⋃⋅⌀⌂⌈⌉⌊⌋⌐⌕⌠⌡⎺⎻⎼⎽␉␊␋␌␍␤\0─▢▧▨\0▪◇\0◊◕◘◙◚◛\0◢◪☥☦☧☨☩\0☰☸☺☻☼☽☾♀♁♂\0♠♧♩♪♫♬♭♮♯⟨⟩\0⠀⣿ﬀﬁﬂ�")
        .leak();

    ::embedded_graphics::mono_font::MonoFont {
        image: ::embedded_graphics::image::ImageRaw::new(
            include_bytes!("raw/mono_5x8.data"),
            80u32,
        ),
        glyph_mapping: atlas,
        character_size: ::embedded_graphics::geometry::Size::new(5u32, 8u32),
        character_spacing: 0u32,
        baseline: 6u32,
        underline: ::embedded_graphics::mono_font::DecorationDimensions::new(8u32, 1u32),
        strikethrough: ::embedded_graphics::mono_font::DecorationDimensions::new(4u32, 1u32),
    }
}
use crate::atlas::FontAtlas;

/// **Danger**: leaking [`FontAtlas<'static>`] for the lifetime of the program
pub fn mono_6x13_atlas() -> ::embedded_graphics::mono_font::MonoFont<'static> {
    let atlas = FontAtlas::from_mapping_str(8, "\0 ~\0\u{a0}Ƞ\0Ȣȳ\0ɐʭ\0ʰˮ\0\u{300}\u{34f}\0\u{360}\u{36f}ʹ͵ͺ;΄΅Ά·ΈΉΊΌ\0ΎΡ\0Σώ\0ϐ϶\0Ѐ\u{486}\0\u{488}ӎ\0ӐӵӸӹ\0Ԁԏ\0ԱՖՙ՚՛՜՝՞՟\0աև։֊\0\u{591}\u{59a}\u{59c}\u{59d}\u{59e}\u{59f}\u{5a0}\u{5a1}\u{5a3}\u{5a4}\u{5a5}\u{5a6}\u{5a8}\u{5a9}\u{5aa}\u{5ab}\u{5ac}\u{5ad}\0\u{5af}\u{5b9}\0\u{5bb}\u{5c4}\0אתװױײ׳״\0ก\u{e3a}\0฿๛\0აჸ჻\0ᆨᇂᇫᇹ\0\u{1680}᚜\0ᚠᛰ\0ḀἕἘἙἚἛἜἝ\0ἠὅὈὉὊὋὌὍ\0ὐὗὙὛὝ\0Ὗώ\0ᾀᾴ\0ᾶῄ\0ῆΐῖῗῘῙῚΊ\0῝`ῲῳῴ\0ῶ῾\0\u{2000}‧\0\u{202f}⁒⁗\u{205f}\u{2060}\u{2061}\u{2062}\u{2063}⁰ⁱ\0⁴₎\0₠₱\0\u{20d0}\u{20ea}\0℀℺\0ℽ⅋\0⅓Ↄ\0←⌨\0⌫⏎\0␀␦\0⑀⑊\0①⒆⓪\0─☓☖☗\0☙♽\0⚀⚉✁✂✃✄✆✈\0✓✠✢✣✤✥✦✧\0✩✸✻✼✽✾✿\0❃❋❍❏❐❑❒❖❘❙❚❛❜❝❞❡❢❣❤❥❦❧\0❶➔\0➘➡➤➥➦➧➲\0➴➾⟦⟧⟨⟩⟪⟫\0⟵⣿⤆⤇\0⦃⦘⧼⧽⨀⨁⨂⨃⨄⨅⨆⨉⨝⨿⸘⸨⸩⸮〿\0\u{e000}\u{e02b}\0\u{e030}\u{e06e}ﬀﬁﬂﬃﬄﬅﬆﬓﬔﬕﬖﬗ\0יִזּטּיּךּכּלּמּנּסּףּפּ\0צּﭏ\u{fe20}\u{fe21}\u{fe22}\u{fe23}\0｡\u{ff9f}\0ﾡﾾￂￃￄￅￆￇￊￋￌￍￎￏￒￓￔￕￖￗￚￛￜ￨￩￪￫￬￭￮￼�")
        .leak();

    ::embedded_graphics::mono_font::MonoFont {
        image: ::embedded_graphics::image::ImageRaw::new(
            include_bytes!("raw/mono_6x13.data"),
            96u32,
        ),
        glyph_mapping: atlas,
        character_size: ::embedded_graphics::geometry::Size::new(6u32, 13u32),
        character_spacing: 0u32,
        baseline: 10u32,
        underline: ::embedded_graphics::mono_font::DecorationDimensions::new(12u32, 1u32),
        strikethrough: ::embedded_graphics::mono_font::DecorationDimensions::new(6u32, 1u32),
    }
}
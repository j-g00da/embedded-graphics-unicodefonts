use crate::atlas::FontAtlas;

/// **Danger**: leaking [`FontAtlas<'static>`] for the lifetime of the program
pub fn mono_10x20_atlas() -> ::embedded_graphics::mono_font::MonoFont<'static> {
    let atlas = FontAtlas::from_mapping_str(8, "\0 ~\0\u{a0}ͷͺͻͼͽ;΄΅Ά·ΈΉΊΌ\0ΎΡ\0Σԣ\0ԱՖՙ՚՛՜՝՞՟\0աև։֊\0\u{5b0}\u{5c7}\0אתװױײ׳״\u{600}\u{601}\u{602}\u{603}\0،\u{615}؛؟\0ءغ\0ـ\u{658}\0٠ۿ\0ก\u{e3a}\0฿๛\0ႠჅ\0აჼሀሁሂሃሄህሆ\0ለቆቈቊቋቌቍቐቑቒቓቔቕቖቘቚቛቜቝ\0በኆኈኊኋኌኍ\0ነኮኰኲኳኴኵኸኹኺኻኼኽኾዀዂዃዄዅወዉዊዋዌውዎዐዑዒዓዔዕዖ\0ዘዮ\0ደጎጐጒጓጔጕጘጙጚጛጜጝጞ\0ጠፆ\0ፈፚ\0፡፼\0\u{1680}᚜\0ᚠᛰ\0ᴀᵫ\0ḀἕἘἙἚἛἜἝ\0ἠὅὈὉὊὋὌὍ\0ὐὗὙὛὝ\0Ὗώ\0ᾀᾴ\0ᾶῄ\0ῆΐῖῗῘῙῚΊ\0῝`ῲῳῴ\0ῶ῾\0\u{2000}\u{200a}\0‐‧\0\u{202f}\u{2064}⁰ⁱ\0⁴₎ₐₑₒₓₔ\0₠₵\0\u{20d0}\u{20f0}\0℀⅏\0⅓ↈ\0←⌨\0⌫⏧\0␀␦\0⑀⑊\0①⒇\0⓪⓴\0⓿⚝\0⚠⚼⛀⛁⛂⛃✁✂✃✄✆✈✉✏✑✒✓✗\0✙✧✩✪✫✱✲✳✴✵✶✻✼❍❏❐❑❒❖❘❙❚❛❜❝❞\0❶➔\0➘➯\0➱➹➾⟕⟖⟗⟦⟧⟨⟩⟪⟫\0⟵⣿\0⨀⨉⨝⨿\0⬀⬍〿\0䷀䷿ﬀﬁﬂﬃﬄﬅﬆﬓﬔﬕﬖﬗ\0יִזּטּיּךּכּלּמּנּסּףּפּ\0צּﮱ\0ﯓﯩﯼﯽﯾﯿ\0ﱝﱥﲐﳲﳳﳴﴼﴽ﴾﴿ﷲ﷼\0\u{fe00}\u{fe0f}\u{fe20}\u{fe21}\u{fe22}\u{fe23}\u{fe24}\u{fe25}\u{fe26}ﹰﹱﹲﹳﹴ\0ﹶﻼ\0｡\u{ff9f}￼�")
        .leak();

    ::embedded_graphics::mono_font::MonoFont {
        image: ::embedded_graphics::image::ImageRaw::new(
            include_bytes!("raw/mono_10x20.data"),
            160u32,
        ),
        glyph_mapping: atlas,
        character_size: ::embedded_graphics::geometry::Size::new(10u32, 20u32),
        character_spacing: 0u32,
        baseline: 15u32,
        underline: ::embedded_graphics::mono_font::DecorationDimensions::new(17u32, 1u32),
        strikethrough: ::embedded_graphics::mono_font::DecorationDimensions::new(10u32, 1u32),
    }
}
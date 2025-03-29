pub const MONO_10X20: ::embedded_graphics::mono_font::MonoFont = ::embedded_graphics::mono_font::MonoFont {
    image: ::embedded_graphics::image::ImageRaw::new(
        include_bytes!("raw/mono_10x20.data"),
        160u32,
    ),
    glyph_mapping: &::embedded_graphics::mono_font::mapping::StrGlyphMapping::new(
        "⬚\0\u{20}~\0\u{a0}ͷ\0ͺ;\0΄ΊΌ\0ΎΡ\0Σԣ\0ԱՖ\0ՙ՟\0աև։֊\0\u{5b0}\u{5c7}\0את\0װ״\0\u{600}\u{603}\0،\u{615}؛؟\0ءغ\0ـ\u{658}\0٠ۿ\0ก\u{e3a}\0฿๛\0ႠჅ\0აჼ\0ሀሆ\0ለቆቈ\0ቊቍ\0ቐቖቘ\0ቚቝ\0በኆኈ\0ኊኍ\0ነኮኰ\0ኲኵ\0ኸኾዀ\0ዂዅ\0ወዎ\0ዐዖ\0ዘዮ\0ደጎጐ\0ጒጕ\0ጘጞ\0ጠፆ\0ፈፚ\0፡፼\0\u{1680}᚜\0ᚠᛰ\0ᴀᵫ\0Ḁἕ\0ἘἝ\0ἠὅ\0ὈὍ\0ὐὗὙὛὝ\0Ὗώ\0ᾀᾴ\0ᾶῄ\0ῆΐ\0ῖΊ\0῝`\0ῲῴ\0ῶ῾\0\u{2000}\u{200a}\0‐‧\0\u{202f}\u{2064}⁰ⁱ\0⁴₎\0ₐₔ\0₠₵\0\u{20d0}\u{20f0}\0℀⅏\0⅓ↈ\0←⌨\0⌫⏧\0␀␦\0⑀⑊\0①⒇\0⓪⓴\0⓿⚝\0⚠⚼\0⛀⛃\0✁✄✆✈✉✏\0✑✓✗\0✙✧\0✩✫\0✱✶✻✼❍\0❏❒❖\0❘❞\0❶➔\0➘➯\0➱➹➾\0⟕⟗\0⟦⟫\0⟵⣿\0⨀⨉⨝⨿\0⬀⬍〿\0䷀䷿\0ﬀﬆ\0ﬓﬗ\0יִזּ\0טּלּמּנּסּףּפּ\0צּﮱ\0ﯓﯩ\0ﯼﯿ\0ﱝﱥﲐ\0ﳲﳴ\0ﴼ﴿ﷲ﷼\0\u{fe00}\u{fe0f}\0\u{fe20}\u{fe26}\0ﹰﹴ\0ﹶﻼ\0｡\u{ff9f}￼�",
        5204usize,
    ),
    character_size: ::embedded_graphics::geometry::Size::new(10u32, 20u32),
    character_spacing: 0u32,
    baseline: 15u32,
    underline: ::embedded_graphics::mono_font::DecorationDimensions::new(17u32, 1u32),
    strikethrough: ::embedded_graphics::mono_font::DecorationDimensions::new(10u32, 1u32),
};

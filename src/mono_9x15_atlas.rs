use crate::atlas::FontAtlas;

/// **Danger**: leaking [`FontAtlas<'static>`] for the lifetime of the program
pub fn mono_9x15_atlas() -> ::embedded_graphics::mono_font::MonoFont<'static> {
    let atlas = FontAtlas::from_mapping_str(8, "\0 ~\0\u{a0}Ƞ\0Ȣȳ\0ɐʭ\0ʰˮ\0\u{300}\u{34f}\0\u{360}\u{36f}ʹ͵ͺ;΄΅Ά·ΈΉΊΌ\0ΎΡ\0Σώ\0ϐ϶\0Ѐ\u{486}\0\u{488}ӎ\0ӐӵӸӹ\0Ԁԏ\0ԱՖՙ՚՛՜՝՞՟\0աև։֊\0\u{5b0}\u{5b9}\0\u{5bb}\u{5c4}\0אתװױײ׳״،؛؟\0ءغ\0ـ\u{652}\u{654}\u{655}\0٠٪پچڎڗڟڤکگ\0ก\u{e3a}\0฿๛ກຂຄງຈຊຍດຕຖທນບປຜຝພຟມຢຣລວສຫ\0ອ\u{eb9}\u{ebb}\u{ebc}ຽເແໂໃໄໆ\u{ec8}\u{ec9}\u{eca}\u{ecb}\u{ecc}\u{ecd}\0໐໙ໜໝ\0ႠჅ\0აჸ჻ሀሁሂሃሄህሆ\0ለቆቈቊቋቌቍቐቑቒቓቔቕቖቘቚቛቜቝ\0በኆኈኊኋኌኍ\0ነኮኰኲኳኴኵኸኹኺኻኼኽኾዀዂዃዄዅወዉዊዋዌውዎዐዑዒዓዔዕዖ\0ዘዮ\0ደጎጐጒጓጔጕጘጙጚጛጜጝጞ\0ጠፆ\0ፈፚ\0፡፼\0ᚠᛰ\0ḀἕἘἙἚἛἜἝ\0ἠὅὈὉὊὋὌὍ\0ὐὗὙὛὝ\0Ὗώ\0ᾀᾴ\0ᾶῄ\0ῆΐῖῗῘῙῚΊ\0῝`ῲῳῴ\0ῶ῾\0\u{2000}\u{200d}\0‐‧\0\u{202f}⁒⁗\u{205f}\u{2060}\u{2061}\u{2062}\u{2063}⁰ⁱ\0⁴₎\0₠₱\0\u{20d0}\u{20ea}\0℀℺\0ℽ⅋\0⅓Ↄ\0←⌨\0⌫⏎\0␀␦\0⑀⑊\0①⓪\0─☓☖☗\0☙♽\0⚀⚉✁✂✃✄✆✇✈✉\0✑✧\0✩✿\0❃❋❍❏❐❑❒❖❘❙❚❛❜❝❞❡❢❣❤❥❦❧\0❶➔\0➘➯\0➱➾\0⟀⟊\0⟐⟫\0⟰⣿⤆⤇\0⨀⨝⨿⸘⸨⸩⸪⸫⸬⸭⸮〿\0\u{e000}\u{e019}\0\u{e700}\u{e72f}ﬀﬁﬂﬃﬄﬅﬆﬓﬔﬕﬖﬗ\0יִזּטּיּךּכּלּמּנּסּףּפּ\0צּﭏﭖﭗﭘﭙﭪﭫﭬﭭﭺﭻﭼﭽﮆﮇ\0ﮎﮕﯼﯽﯾﯿ\u{fe20}\u{fe21}\u{fe22}\u{fe23}ﹰﹲﹴﹶﹸﹺﹼﹽﹾ\0ﺀﻼ\0｡\u{ff9f}￼�")
        .leak();

    ::embedded_graphics::mono_font::MonoFont {
        image: ::embedded_graphics::image::ImageRaw::new(
            include_bytes!("raw/mono_9x15.data"),
            144u32,
        ),
        glyph_mapping: atlas,
        character_size: ::embedded_graphics::geometry::Size::new(9u32, 15u32),
        character_spacing: 0u32,
        baseline: 11u32,
        underline: ::embedded_graphics::mono_font::DecorationDimensions::new(13u32, 1u32),
        strikethrough: ::embedded_graphics::mono_font::DecorationDimensions::new(7u32, 1u32),
    }
}
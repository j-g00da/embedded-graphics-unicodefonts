use crate::atlas::FontAtlas;

/// **Danger**: leaking [`FontAtlas<'static>`] for the lifetime of the program
pub fn mono_9x15_bold_atlas() -> ::embedded_graphics::mono_font::MonoFont<'static> {
    let atlas = FontAtlas::from_mapping_str(8, "\0 ~\0\u{a0}ǵ\0Ǻțɘəʼʽˆˇ˘˙˚˛˜˝\0\u{300}\u{314}\u{316}\u{317}\u{323}\u{324}\u{325}\u{326}\u{330}\u{331}\u{332}ʹ͵ͺ;΄΅Ά·ΈΉΊΌ\0ΎΡ\0Σώ\0ϐϗ\0Ϛϡϰϱϲϳϴϵ\0ЀџѢѣ\0ѰѷҐґ\0את،؛؟\0ءغ\0ـ\u{652}\u{654}\u{655}\0٠٪پچڎڗڟڤکگ\0ก\u{e3a}\0฿๛ᚠᚢᚣᚦᚩᚪᚫᚱᚳᚷᚸᚹᚻᚾᛁᛄᛇᛈᛉᛋᛏᛒᛖᛗᛚᛝᛞᛟᛠᛡᛢᛣᛥ᛫᛬᛭\0Ḁẛ\0Ạỹ\0ἀἕἘἙἚἛἜἝ\0ἠὅὈὉὊὋὌὍ\0ὐὗὙὛὝ\0Ὗώ\0ᾀᾴ\0ᾶῄ\0ῆΐῖῗῘῙῚΊ\0῝`ῲῳῴ\0ῶ῾\0‐‧‰′″‴‵‶‷‹›‼‾⁄ⁿ₣₤₫€₯ℂ℅ℓℕ№ℚℝ™ℤΩ℮⅛⅜⅝⅞←↑→↓−≠≤≥⌈⌉⌊⌋⎺⎻⎼⎽␉␊␋␌␍␤─│┌┐└┘├┤┬┴┼╌╎\0╭╷▒▮◆♪⟨⟩ﭖﭗﭘﭙﭪﭫﭬﭭﭺﭻﭼﭽﮆﮇ\0ﮎﮕﯼﯽﯾﯿﹰﹲﹴﹶﹸﹺﹼﹽﹾ\0ﺀﻼ�")
        .leak();

    ::embedded_graphics::mono_font::MonoFont {
        image: ::embedded_graphics::image::ImageRaw::new(
            include_bytes!("raw/mono_9x15_bold.data"),
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
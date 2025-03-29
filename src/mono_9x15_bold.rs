pub const MONO_9X15_BOLD: ::embedded_graphics::mono_font::MonoFont = ::embedded_graphics::mono_font::MonoFont {
    image: ::embedded_graphics::image::ImageRaw::new(
        include_bytes!("raw/mono_9x15b.data"),
        144u32,
    ),
    glyph_mapping: &::embedded_graphics::mono_font::mapping::StrGlyphMapping::new(
        "⬚\0\u{20}~\0\u{a0}ǵ\0Ǻțɘəʼʽˆˇ\0˘˝\0\u{300}\u{314}\u{316}\u{317}\0\u{323}\u{326}\0\u{330}\u{332}ʹ͵ͺ;\0΄ΊΌ\0ΎΡ\0Σώ\0ϐϗ\0Ϛϡ\0ϰϵ\0ЀџѢѣ\0ѰѷҐґ\0את،؛؟\0ءغ\0ـ\u{652}\u{654}\u{655}\0٠٪پچڎڗڟڤکگ\0ก\u{e3a}\0฿๛ᚠᚢᚣᚦ\0ᚩᚫᚱᚳ\0ᚷᚹᚻᚾᛁᛄ\0ᛇᛉᛋᛏᛒᛖᛗᛚ\0ᛝᛣᛥ\0᛫᛭\0Ḁẛ\0Ạỹ\0ἀἕ\0ἘἝ\0ἠὅ\0ὈὍ\0ὐὗὙὛὝ\0Ὗώ\0ᾀᾴ\0ᾶῄ\0ῆΐ\0ῖΊ\0῝`\0ῲῴ\0ῶ῾\0‐‧‰\0′‷‹›‼‾⁄ⁿ₣₤₫€₯ℂ℅ℓℕ№ℚℝ™ℤΩ℮\0⅛⅞\0←↓−≠≤≥\0⌈⌋\0⎺⎽\0␉␍␤─│┌┐└┘├┤┬┴┼╌╎\0╭╷▒▮◆♪⟨⟩\0ﭖﭙ\0ﭪﭭ\0ﭺﭽﮆﮇ\0ﮎﮕ\0ﯼﯿﹰﹲﹴﹶﹸﹺ\0ﹼﹾ\0ﺀﻼ�",
        1687usize,
    ),
    character_size: ::embedded_graphics::geometry::Size::new(9u32, 15u32),
    character_spacing: 0u32,
    baseline: 11u32,
    underline: ::embedded_graphics::mono_font::DecorationDimensions::new(13u32, 1u32),
    strikethrough: ::embedded_graphics::mono_font::DecorationDimensions::new(7u32, 1u32),
};

# embedded-graphics-unicodefonts

Rust crate for `embedded-graphics` fonts with larger range of unicode characters such as drawing glyphs or braille.
Created for [Mousefood](https://github.com/j-g00da/mousefood).

## FontAtlas System

Fonts with `_atlas` suffix use a `FontAtlas` instead of `StrGlyphMapping` for glyph lookups, providing up to 80x
faster performance compared to non-atlas fonts.

```rust
use embedded_graphics_unicodefonts::mono_6x13_atlas;

let font = mono_6x13_atlas();
```

Both standard `MonoFont` constants and atlas-backed functions are available for each font size.


## Example

`MONO_6X10` (replaces `embedded-graphics::mono_font::ascii::FONT_6X10`)

![MONO_6X10](assets/basic_6x10.png)

Generated using [embedded-graphics/bdf](https://github.com/embedded-graphics/bdf)
from [xorg misc-misc font](https://gitlab.freedesktop.org/xorg/font/misc-misc).

All fonts are generated using [tools/bdf-atlas-converter/generate-fonts.sh][generate-fonts], which uses the
[bdf-atlas-converter][bdf-converter] tool. This CLI can also be used to generate embedded fonts from
user-supplied BDF files. Existing fonts are sourced from from [xorg misc-misc font][misc-misc].

 [generate-fonts]: tools/bdf-atlas-converter/generate-fonts.sh
 [bdf-converter]: tools/bdf-atlas-converter/README.md
 [misc-misc]: https://gitlab.freedesktop.org/xorg/font/misc-misc

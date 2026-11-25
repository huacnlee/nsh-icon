# nsh-lucide

[Lucide](https://lucide.dev) icons for [Native Shell (NSH)](https://nsh.dev) apps, as an
[nshm](https://nsh.dev/docs/nshm): all 1,818 icons of Lucide 1.43.0, drawn as vectors in the text color
around them, and **only the icons an app uses go into its binary and bundle**.

Use it instead of characters (`"⚙"`, `"✕"`, `"›"`) or emoji as icons: those depend on the fonts of each
platform, do not follow the theme's stroke and size, and screen readers read them out as text.

```sh
nsh add nsh-lucide                       # from crates.io
nsh add github:huacnlee/nsh-lucide       # or from GitHub
```

## TSX

```tsx
import { Text, View } from "nsh";
import { Icon } from "nsh-lucide";
import { ChevronRight, Search, Settings } from "nsh-lucide/icons";

<View class="flex-row items-center gap-2">
  <Icon icon={Search} class="text-muted-foreground" />
  <Text class="text-sm text-muted-foreground">Search</Text>
</View>

<Icon icon={Settings} size={20} label="Settings" class="text-foreground" />
<Icon icon={ChevronRight} strokeWidth={1.5} />
```

| Prop | Type | Default | |
| --- | --- | --- | --- |
| `icon` | `string` | required | An icon from `nsh-lucide/icons`, or the URL / `data:` URI of any SVG stroked with `currentColor` |
| `size` | `number` | `16` | Width and height in points |
| `strokeWidth` | `number` | `2` | Line width, out of the icon's 24 units |
| `label` | `string` | none | What screen readers say. Without it the icon is decorative and hidden from them |

The color is the text color: tint an icon with a `text-*` class on it or on a view around it.
`class` and `style` lay it out like any view.

Icons are named as on [lucide.dev](https://lucide.dev/icons) and in `lucide-react`, in PascalCase
(`chevron-down` is `ChevronDown`, `arrow-down-0-1` is `ArrowDown01`). Lucide's old names (`Home`,
`Edit`) still work, marked `@deprecated`, with the current name in the hint. Rename an import that
clashes with another one: `import { Image as ImageIcon } from "nsh-lucide/icons"`.

## Rust

```rust
use nsh_core::prelude::*;
use nsh_lucide::{Icon, icon, labeled_icon};

view()
    .flex_row()
    .items_center()
    .gap_2()
    .text_color(move || theme().muted_foreground)
    .child(icon(Icon::Search))
    .child(text("Search"))

// Sized and tinted like any element.
icon(Icon::Settings).size(px(20.0)).text_color(move || theme().primary)

// An icon that is the only content of a button says what it does.
view().on_press(close).child(labeled_icon(Icon::X, "Close"))

// It follows a signal.
icon(move || if dark.get() { Icon::Moon } else { Icon::Sun })
```

- `icon(src)`: a decorative icon, 16 points, hidden from screen readers. `src` is an `Icon`, a closure
  or signal giving one, or any SVG image source.
- `labeled_icon(src, label)`: an icon screen readers read as an image named `label`.
- `Icon::Search.src_with_stroke_width(1.5)`: the icon with thinner lines.
- `Icon::name()` is the Lucide name (`"search"`), and `Icon::data_uri()` the SVG document. An `Icon`
  is also an `ImageSrc`, so `svg(Icon::Search)` and `image(Icon::Search)` work.
- The TSX component is `nsh_lucide::tsx::Icon` (`tsx::icon().icon(Icon::Bell).size(20.0)`).

## Only the icons an app uses

This follows how [GPUI Kit's assets](https://github.com/longbridge/gpui-kit/tree/main/crates/assets)
handle Lucide: the whole catalog is available by name, but nothing pulls every payload into the app.

- **Rust**: each icon is an associated constant of `Icon` holding its SVG as a `&'static str`
  `data:` URI, generated at build time from `icons/`. A constant no code uses is never compiled in.
- **TSX**: each icon is an exported constant of `nsh-lucide/icons`. The bundler leaves out the
  exports the app does not import.
- **Drawing**: no copy, cache or table is built at run time. NSH parses an icon's drawing once,
  shares it between every view that shows it, and keeps it in its image budget only while one does.

A lookup by name at run time (names coming from data or settings) needs a table of the icons it can
find. List the icons explicitly, as GPUI Kit's `icon_assets!` does:

```rust
nsh_lucide::icon_set!(pub FileIcons, [File, FileText, FileImage, Folder]);

let glyph = FileIcons::get("file-text").unwrap_or(Icon::File);
```

Or turn on the `all` feature (`nsh add nsh-lucide --features all`) for `Icon::ALL` and
`Icon::from_name("search")` (old names too) over the whole catalog, which links every icon.

### What it costs

Rust, measured with `cargo build --release` (stripped) of the `examples/size_*.rs` programs on Linux
x86_64, Rust 1.99:

| Icons in the program | Binary size | Increase |
| --- | ---: | ---: |
| None (baseline) | 338,912 B | — |
| 2 (`Search`, `Check`) | 339,552 B | +640 B |
| 10 | 343,360 B | +4.3 KiB |
| All 1,818 (`all` feature, with `from_name`) | 1,852,904 B | +1.44 MiB |

TSX: the example app in `examples/tsx-app` imports 5 icons. Its release bundle (`nsh build --release`)
is 3.3 KB of JavaScript and holds those 5 icons, out of the 1,818 that `nsh-lucide/icons` exports.

An icon's SVG takes about 300 to 1,500 bytes. Binary size is not memory use: drawing an icon also
allocates for parsing and rasterizing it, as any SVG does. Cargo's and nsh's downloaded copy of the
crate still holds every SVG file.

## Development

```sh
cargo test                     # catalog, headless rendering, TSX declarations
cargo test --features all
cd examples/tsx-app && nsh check && nsh test
```

`lucide.json` pins the upstream release and its archive's SHA-256. To update Lucide, change its
`version`, `archive_url`, `sha256` and `icon_count`, then:

```sh
bun script/sync-lucide.ts                       # icons/*.svg, icons/aliases.txt, LICENSE-LUCIDE
bun script/sync-lucide.ts --check               # verify every file byte for byte
NSH_LUCIDE_UPDATE=1 cargo test --test tsx       # regenerate tsx/icons.ts
```

`--archive path/to/lucide.tar.gz` reads a local archive instead of downloading it. `codegen.rs`
generates both the Rust constants (from `build.rs`) and `tsx/icons.ts`, so the two always match.

## License

Code: Apache-2.0 ([LICENSE](LICENSE)). Lucide icons: ISC, with the icons derived from Feather under
MIT; see [LICENSE-LUCIDE](LICENSE-LUCIDE).

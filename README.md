# nsh-icon

Icons for [Native Shell (NSH)](https://nsh.dev) apps, as [nshm](https://nsh.dev/docs/nshm): popular icon sets drawn as
vectors in the text color around them. **Only the icons an app uses go into its binary and bundle**, and only
the sets it adds are downloaded and built.

| Set | Crate | Icons | Styles | Rust | TSX |
| --- | --- | ---: | --- | --- | --- |
| [Lucide](https://lucide.dev) 1.43.0 | `nsh-icon-lucide` | 1,818 | one | `Lucide::Search` | `import { Search } from "nsh-icon-lucide"` |
| [Heroicons](https://heroicons.com) 2.2.0 | `nsh-icon-heroicons` | 1,288 | outline, solid, mini, micro | `Heroicons::MagnifyingGlass` | `"nsh-icon-heroicons"` |
| [Tabler Icons](https://tabler.io/icons) 3.53.0 | `nsh-icon-tabler` | 6,563 | outline, filled | `Tabler::Search` | `"nsh-icon-tabler"` |
| [Phosphor](https://phosphoricons.com) 2.1.1 | `nsh-icon-phosphor` | 9,072 | 6 weights | `Phosphor::MagnifyingGlass` | `"nsh-icon-phosphor"` |
| [Remix Icon](https://remixicon.com) 4.8.0 | `nsh-icon-remix` | 3,188 | line, fill | `Remix::SearchLine` | `"nsh-icon-remix"` |

`nsh-icon` itself holds no icons: it draws them (`Icon`, `icon()`, `labeled_icon()`, `icon_set!`, and the TSX
`<Icon>`). Each set is a crate of its own that depends on it.

Use it instead of characters (`"⚙"`, `"✕"`, `"›"`) or emoji as icons: those depend on the fonts of each
platform, do not follow the theme's stroke and size, and screen readers read them out as text.

```sh
nsh add nsh-icon nsh-icon-lucide       # the drawing, and the sets the app uses
```

## Names and styles

Icons are named as upstream, in PascalCase, the way [shadcn.io](https://www.shadcn.io/icons) and
[Iconify](https://iconify.design) list them: one library per set, a style is a suffix of the name.

| Set | Styles |
| --- | --- |
| Lucide | `ChevronDown`, `ArrowDown01`; old names (`Home`, `Edit`) are deprecated aliases |
| Heroicons | `Home` (outline 24), `HomeSolid` (24), `Home20Solid` (mini), `Home16Solid` (micro) |
| Tabler | `Home` (outline), `HomeFilled`; old names are deprecated aliases |
| Phosphor | `House`, `HouseThin`, `HouseLight`, `HouseBold`, `HouseFill`, `HouseDuotone` |
| Remix | `HomeLine`, `HomeFill`; names that start with a digit start with `Icon` (`Icon24HoursFill`) |

Rename an import that clashes with another one: `import { Image as ImageIcon } from "nsh-icon-lucide"`.

## TSX

```tsx
import { Text, View } from "nsh";
import { Icon } from "nsh-icon";
import { HeartFill } from "nsh-icon-phosphor";
import { Search, Settings } from "nsh-icon-lucide";

<View class="flex-row items-center gap-2">
  <Icon icon={Search} class="text-muted-foreground" />
  <Text class="text-sm text-muted-foreground">Search</Text>
</View>

<Icon icon={Settings} size={20} label="Settings" class="text-foreground" />
<Icon icon={HeartFill} class="text-destructive" />
```

| Prop | Type | Default | |
| --- | --- | --- | --- |
| `icon` | `string` | required | An icon of a set, or the URL / `data:` URI of any SVG drawn with `currentColor` |
| `size` | `number` | `16` | Width and height in points |
| `strokeWidth` | `number` | the set's | Line width in the icon's units, for a stroked icon (Lucide, Tabler outline, Heroicons outline) |
| `label` | `string` | none | What screen readers say. Without it the icon is decorative and hidden from them |

The color is the text color: tint an icon with a `text-*` class on it or on a view around it.
`class` and `style` lay it out like any view.

## Rust

```rust
use nsh_core::prelude::*;
use nsh_icon::{icon, labeled_icon};
use nsh_icon_lucide::Lucide;

view()
    .flex_row()
    .items_center()
    .gap_2()
    .text_color(move || theme().muted_foreground)
    .child(icon(Lucide::Search))
    .child(text("Search"))

// Sized and tinted like any element.
icon(Lucide::Settings).size(px(20.0)).text_color(move || theme().primary)

// An icon that is the only content of a button says what it does.
view().on_press(close).child(labeled_icon(Lucide::X, "Close"))

// It follows a signal.
icon(move || if dark.get() { Lucide::Moon } else { Lucide::Sun })
```

- `icon(src)`: a decorative icon, 16 points, hidden from screen readers. `src` is an `Icon`, a closure
  or signal giving one, or any SVG image source.
- `labeled_icon(src, label)`: an icon screen readers read as an image named `label`.
- Every icon of every set is an `Icon`: `name()` is its upstream name (`"search"`), `data_uri()` its SVG
  document, `src_with_stroke_width(1.5)` a stroked icon with other lines. An `Icon` is also an `ImageSrc`,
  so `svg(Lucide::Search)` and `image(Lucide::Search)` work. `Icon::new(name, data_uri)` makes an app's own.
- Each set crate re-exports `Icon`, `icon`, `labeled_icon` and `icon_set!`: `use nsh_icon_lucide::{Lucide, icon}`.
- The TSX component is `nsh_icon::tsx::Icon` (`tsx::icon().icon(Lucide::Bell).size(20.0)`).

## Only what an app uses

This follows how [GPUI Kit's assets](https://github.com/longbridge/gpui-kit/tree/main/crates/assets) handle
Lucide, and, like shadcn.io, gives an app icons one at a time rather than a whole library:

- **Sets**: each is its own crate. An app downloads and compiles only the sets it adds.
- **Rust**: each icon is an associated constant of its set's type, holding its SVG as a `&'static str`
  `data:` URI. A constant no code uses is never compiled in.
- **TSX**: each icon is an exported constant of its set's module. The bundler leaves out the exports the
  app does not import.
- **Drawing**: no copy, cache or table is built at run time. NSH parses an icon's drawing once, shares
  it between every view that shows it, and keeps it in its image budget only while one does.

A lookup by name at run time (names coming from data or settings) needs a table of the icons it can find.
List the icons explicitly, as GPUI Kit's `icon_assets!` does:

```rust
use nsh_icon::Icon;
use nsh_icon_lucide::Lucide;

nsh_icon::icon_set!(pub FileIcons, [Lucide::File, Lucide::FileText, Lucide::Folder]);

let glyph: Icon = FileIcons::get("file-text").unwrap_or(Lucide::File);
```

Or turn on a set's `all` feature (`nsh add nsh-icon-lucide --features all`) for `Lucide::ALL` and
`Lucide::from_name("search")` (old names too) over the whole set, which links every icon of it.

### What it costs

Rust, measured with `cargo build --release` (stripped) of `sets/lucide/examples/size_*.rs` on Linux x86_64,
Rust 1.99:

| Lucide icons in the program | Binary size | Increase |
| --- | ---: | ---: |
| None (baseline) | 338,912 B | — |
| 2 (`Search`, `Check`) | 339,552 B | +640 B |
| 10 | 343,360 B | +4.3 KiB |
| All 1,818 (`all` feature, with `from_name`) | 1,634,952 B | +1.24 MiB |

TSX: the example app in `examples/tsx-app` imports 9 icons from Lucide, Heroicons and Phosphor. Its release
bundle (`nsh build --release`) is 6.1 KB of JavaScript and holds those 9 icons, out of the 12,178 the three
sets export.

The published crates hold generated sources, not SVG files: about 0.3 MB compressed for Lucide and
Heroicons, 1 MB for Tabler and Remix, 2.2 MB for Phosphor. An icon's SVG takes about 300 to 1,500 bytes.
Binary size is not memory use: drawing an icon also allocates for parsing and rasterizing it, as any SVG
does.

## Development

```sh
cargo test --workspace                     # the sets, headless rendering, TSX declarations
cargo test --workspace --all-features
cd examples/tsx-app && nsh check && nsh test
```

Each set lives in `sets/<set>/`, the crate `nsh-icon-<set>`:

- `set.json` pins the upstream release: its archive's URL and SHA-256, and its icon count.
- `icons/` holds its SVG files byte for byte, flattened into one directory under Iconify's names, with
  `aliases.txt` (old names, `<old> <current>`) and `LICENSE`. They stay in the repository; the crate
  does not publish them.
- `src/icons.rs` and `tsx/index.ts` are generated from them by the `codegen` crate, so Rust and TSX
  always match.

To update a set, change its `set.json`, then:

```sh
bun script/sync.ts lucide                         # sets/lucide: icons/, aliases.txt, LICENSE
bun script/sync.ts --check                        # verify every set byte for byte
NSH_ICON_UPDATE=1 cargo test -p nsh-icon-codegen  # regenerate src/icons.rs and tsx/index.ts
```

`--archive path/to/archive` reads a local copy of one set's archive instead of downloading it.

### Adding a set

1. Add `sets/<set>/set.json`, and how its archive's files map to icon names to `SOURCES` in
   `script/sync.ts`; run `bun script/sync.ts <set>`.
2. Add its `SetSpec` to `SETS` in `codegen/src/lib.rs`, and the crate's `Cargo.toml`, `nsh.json` and
   `src/lib.rs` as the other sets have them; run `NSH_ICON_UPDATE=1 cargo test -p nsh-icon-codegen`.

## License

`nsh-icon`: Apache-2.0 ([LICENSE](LICENSE)). Each set crate is under its icons' license, in
`sets/<set>/LICENSE`: Lucide ISC (its Feather-derived icons MIT), Heroicons, Tabler and Phosphor MIT,
Remix Icon Apache-2.0 (4.8.0, the last release under it).

---
name: nsh-icon
description: 'Icons in Native Shell (NSH) apps with the nsh-icon nshm and its icon sets: Lucide (nsh-icon-lucide), Heroicons (nsh-icon-heroicons), Tabler (nsh-icon-tabler), Phosphor (nsh-icon-phosphor) and Remix Icon (nsh-icon-remix). Rust: icon(Lucide::Search), labeled_icon(); TSX: <Icon icon={Search} /> from "nsh-icon", icons from "nsh-icon-lucide". Only the icons used reach the binary and bundle. Use whenever an NSH app shows an icon or glyph: a button, toolbar, menu, list row, tab, empty state, status or indicator. Never draw icons with characters or emoji.'
---

# Icons in NSH apps: nsh-icon

**An icon is never a character.** Do not write `"⚙"`, `"✕"`, `"×"`, `"›"`, `"▾"`, `"✓"`, `"+"`, `"…"`,
`"🔍"` or any emoji or glyph as an icon. Their look depends on each platform's fonts, they do not
match the stroke, size and color of the rest of the UI, and screen readers read them as text. Draw an
icon of a set with `nsh-icon`.

## Install

`nsh-icon` draws icons; each set is a crate of its own. Add the drawing and one set, the one the app
already uses, else Lucide:

```sh
nsh add nsh-icon nsh-icon-lucide
```

| Set | Crate | Rust type | Styles in the name |
| --- | --- | --- | --- |
| Lucide | `nsh-icon-lucide` | `Lucide` | one style |
| Heroicons | `nsh-icon-heroicons` | `Heroicons` | `Home`, `HomeSolid`, `Home20Solid`, `Home16Solid` |
| Tabler | `nsh-icon-tabler` | `Tabler` | `Home`, `HomeFilled` |
| Phosphor | `nsh-icon-phosphor` | `Phosphor` | `House`, `HouseBold`, `HouseFill`, `HouseDuotone`, `HouseLight`, `HouseThin` |
| Remix Icon | `nsh-icon-remix` | `Remix` | `HomeLine`, `HomeFill` |

Keep one set, and one style of it, throughout an app.

## TSX

```tsx
import { Icon } from "nsh-icon";
import { Search, Settings, X } from "nsh-icon-lucide";

<Icon icon={Search} class="text-muted-foreground" />            // decorative, 16pt
<Icon icon={Settings} size={20} label="Settings" />              // stands alone: label it
<View role="button" accessibilityLabel="Close" onPress={close}>
  <Icon icon={X} />
</View>
```

- Import each icon by name from its set (`nsh-icon-lucide`). Never use `import *` and a computed
  name: only the imported icons ship.
- Props: `icon` (required), `size` (points, default 16), `strokeWidth` (stroked icons), `label`.
- Color is the text color: use a `text-*` token class (`text-foreground`, `text-muted-foreground`,
  `text-primary`, `text-destructive`). Never a hex color.

## Rust

```rust
use nsh_icon_lucide::{Lucide, icon, labeled_icon};

icon(Lucide::Search)                                        // decorative, 16pt
icon(Lucide::Settings).size(px(20.0)).text_color(move || theme().primary)
labeled_icon(Lucide::X, "Close")                            // an icon-only control
icon(move || if open.get() { Lucide::ChevronUp } else { Lucide::ChevronDown })
```

To find icons by a name only known at run time, list them: `nsh_icon::icon_set!(pub StatusIcons,
[Lucide::CircleCheck, Lucide::CircleAlert, Lucide::CircleX]);` then `StatusIcons::get("circle-check")`.
Turn on a set's `all` feature (`Lucide::from_name`) only when any icon of it can be asked for: it links
all of them.

## Names

Upstream names in PascalCase, as on the set's site and in its React package: `chevron-down` is
`ChevronDown`, `arrow-down-0-1` is `ArrowDown01`. Lucide's and Tabler's old names (`Home`, `Edit`) still
compile but are deprecated: use the current name the warning gives. Common Lucide choices:

| Purpose | Icon |
| --- | --- |
| Close, clear, dismiss | `X` |
| Add, new | `Plus` |
| Remove an item | `Trash`, `Minus` |
| Confirm, selected | `Check` |
| Expand, menu trigger, select | `ChevronDown` |
| Navigate forward / back | `ChevronRight` / `ChevronLeft`, `ArrowLeft` |
| More actions | `Ellipsis`, `EllipsisVertical` |
| Search | `Search` |
| Settings | `Settings` |
| Edit | `Pencil`, `SquarePen` |
| Copy | `Copy` |
| Info / warning / error / success | `Info`, `TriangleAlert`, `CircleAlert`, `CircleCheck` |
| Loading | `LoaderCircle` with `class="animate-spin"` |
| Menu (navigation) | `Menu` |
| External link | `ExternalLink` |
| Theme | `Sun`, `Moon` |

## Rules

- An icon beside text that says the same thing is decorative: no `label`.
- An icon that is a control's only content has a name: `labeled_icon` / `label`, or the control's
  own `accessibilityLabel`.
- Keep icon sizes on the type scale: 16 for body text, 14 in dense rows, 20 in toolbars.
- One set, one style and one stroke width throughout an app.

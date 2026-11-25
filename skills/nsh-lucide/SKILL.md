---
name: nsh-lucide
description: 'Icons in Native Shell (NSH) apps with the nsh-lucide nshm: the 1,818 Lucide icons in Rust (nsh_lucide::Icon, icon(), labeled_icon()) and TSX (<Icon icon={Search} /> from "nsh-lucide", icons from "nsh-lucide/icons"), with only the icons used in the binary and bundle. Use whenever an NSH app shows an icon or glyph: a button, toolbar, menu, list row, tab, empty state, status or indicator. Never draw icons with characters or emoji.'
---

# Icons in NSH apps: nsh-lucide

**An icon is never a character.** Do not write `"⚙"`, `"✕"`, `"×"`, `"›"`, `"▾"`, `"✓"`, `"+"`, `"…"`,
`"🔍"` or any emoji or glyph as an icon. Their look depends on each platform's fonts, they do not
match the stroke, size and color of the rest of the UI, and screen readers read them as text. Use a
Lucide icon from `nsh-lucide`.

## Install

```sh
nsh add nsh-lucide
```

A Rust app gets `nsh-lucide` in its `Cargo.toml`; a TSX app can import `nsh-lucide` and
`nsh-lucide/icons`.

## TSX

```tsx
import { Icon } from "nsh-lucide";
import { Search, Settings, X } from "nsh-lucide/icons";

<Icon icon={Search} class="text-muted-foreground" />            // decorative, 16pt
<Icon icon={Settings} size={20} label="Settings" />              // stands alone: label it
<View role="button" accessibilityLabel="Close" onPress={close}>
  <Icon icon={X} />
</View>
```

- Import each icon by name from `nsh-lucide/icons`. Never use `import *` and a computed
  name: only the imported icons ship.
- Props: `icon` (required), `size` (points, default 16), `strokeWidth` (default 2), `label`.
- Color is the text color: use a `text-*` token class (`text-foreground`, `text-muted-foreground`,
  `text-primary`, `text-destructive`). Never a hex color.

## Rust

```rust
use nsh_lucide::{Icon, icon, labeled_icon};

icon(Icon::Search)                                        // decorative, 16pt
icon(Icon::Settings).size(px(20.0)).text_color(move || theme().primary)
labeled_icon(Icon::X, "Close")                            // an icon-only control
icon(move || if open.get() { Icon::ChevronUp } else { Icon::ChevronDown })
```

To find icons by a name only known at run time, list them: `nsh_lucide::icon_set!(pub
StatusIcons, [CircleCheck, CircleAlert, CircleX]);` then `StatusIcons::get("circle-check")`. Turn
on the `all` feature (`Icon::from_name`) only when any Lucide icon can be asked for: it links all
of them (about 1.4 MiB).

## Names

Lucide names in PascalCase, as on lucide.dev and in lucide-react: `chevron-down` is
`ChevronDown`, `arrow-down-0-1` is `ArrowDown01`, `circle-check` is `CircleCheck`. Old names (`Home`, `Edit`,
`CheckCircle`) still compile but are deprecated: use the current name the warning gives (`House`,
`SquarePen`, `CircleCheckBig`). Common choices:

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
- One stroke width throughout an app.

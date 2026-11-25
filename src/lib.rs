//! Icons for [Native Shell](https://nsh.dev) apps, drawn as vectors in the
//! text color around them, with only the icons an app uses in its binary and
//! bundle.
//!
//! This crate draws icons: the [`Icon`] type, [`icon`], [`labeled_icon`],
//! [`icon_set!`] and the TSX `<Icon>`. The icons come in sets, each a crate
//! of its own that an app adds when it uses it:
//!
//! | Set | Crate | Rust | TSX |
//! | --- | --- | --- | --- |
//! | [Lucide](https://lucide.dev) | `nsh-icon-lucide` | `Lucide::Search` | `import { Search } from "nsh-icon-lucide"` |
//! | [Heroicons](https://heroicons.com) | `nsh-icon-heroicons` | `Heroicons::MagnifyingGlass` | `"nsh-icon-heroicons"` |
//! | [Tabler Icons](https://tabler.io/icons) | `nsh-icon-tabler` | `Tabler::Search` | `"nsh-icon-tabler"` |
//! | [Phosphor](https://phosphoricons.com) | `nsh-icon-phosphor` | `Phosphor::MagnifyingGlass` | `"nsh-icon-phosphor"` |
//! | [Remix Icon](https://remixicon.com) | `nsh-icon-remix` | `Remix::SearchLine` | `"nsh-icon-remix"` |
//!
//! In Rust:
//!
//! ```ignore
//! use nsh_core::prelude::*;
//! use nsh_icon::icon;
//! use nsh_icon_lucide::Lucide;
//!
//! view()
//!     .flex_row()
//!     .items_center()
//!     .gap_2()
//!     .text_color(move || theme().muted_foreground)
//!     .child(icon(Lucide::Search))
//!     .child(text("Search"))
//! ```
//!
//! In TSX (`nsh add nsh-icon-lucide`):
//!
//! ```tsx
//! import { Icon } from "nsh-icon";
//! import { Search } from "nsh-icon-lucide";
//!
//! <Icon icon={Search} class="text-muted-foreground" />
//! ```
//!
//! # What ships
//!
//! Every icon is a constant: in Rust an associated constant of its set's
//! type, in TSX an exported string of its set's module. The compiler leaves
//! the constants no code uses out of the binary, and a packaged app's bundle
//! leaves out the TSX ones it does not import. A lookup by name at run time
//! needs the icons it can find in the binary: list them with [`icon_set!`],
//! or turn on a set crate's `all` feature for `from_name` over the whole set.

mod icon;
pub mod tsx;

pub use icon::Icon;

use nsh_core::element::{ImageSrc, Svg};
use nsh_core::prelude::*;

/// The size of an icon that sets none, in points: the line height of body
/// text.
pub const DEFAULT_SIZE: f32 = 16.0;

/// A decorative icon: [`DEFAULT_SIZE`] points, in the text color it inherits,
/// hidden from screen readers (the text or control beside it says what it
/// means).
///
/// `src` is an [`Icon`], a closure or signal giving one
/// (`move || if dark.get() { Lucide::Moon } else { Lucide::Sun }`), or any
/// SVG image source. Size it with `.size(px(20.0))` and tint it with
/// `.text_color(move || theme().primary)`.
pub fn icon(src: impl IntoReactive<ImageSrc>) -> Svg {
    svg(src)
        .size(px(DEFAULT_SIZE))
        .flex_shrink_0()
        .accessibility_elements_hidden(true)
}

/// An icon that stands alone and means something, such as the only content
/// of a button: screen readers say `label` for it, as an image.
pub fn labeled_icon(src: impl IntoReactive<ImageSrc>, label: impl Into<SharedString>) -> Svg {
    svg(src)
        .size(px(DEFAULT_SIZE))
        .flex_shrink_0()
        .accessible(true)
        .role(AccessibilityRole::Image)
        .label(label.into())
}

/// A set of icons an app finds by name at run time, holding only the icons
/// it lists, of one set or several:
///
/// ```ignore
/// use nsh_icon::Icon;
/// use nsh_icon_lucide::Lucide;
///
/// nsh_icon::icon_set!(pub FileIcons, [Lucide::File, Lucide::FileText, Lucide::Folder]);
///
/// let glyph = FileIcons::get("file-text").unwrap_or(Lucide::File);
/// ```
///
/// It makes a unit struct with `ALL`, the icons in the order listed, and
/// `get(name)`, the first listed icon of that name in its set (`None` for
/// any other).
#[macro_export]
macro_rules! icon_set {
    ($vis:vis $name:ident, [$($set:ident :: $icon:ident),* $(,)?]) => {
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
        $vis struct $name;

        impl $name {
            /// The icons of the set, in the order listed.
            pub const ALL: &'static [$crate::Icon] = &[$($set::$icon),*];

            /// The listed icon of this name.
            pub fn get(name: &str) -> ::core::option::Option<$crate::Icon> {
                Self::ALL.iter().copied().find(|icon| icon.name() == name)
            }
        }
    };
}

/// What a TSX app imports from `nsh-icon`: the `<Icon>` component. The
/// icons come from the sets' crates (`nsh-icon-lucide`).
#[native_module("nsh-icon")]
fn native(registry: &mut Registry) {
    registry.register::<tsx::Icon>();
}

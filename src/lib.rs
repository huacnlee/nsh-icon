//! [Lucide](https://lucide.dev) icons for [Native Shell](https://nsh.dev) apps,
//! drawn as vectors in the text color around them, with only the icons an
//! app uses in its binary and bundle.
//!
//! In Rust, name an icon with [`Icon`] and draw it with [`icon`]:
//!
//! ```ignore
//! use nsh_core::prelude::*;
//! use nsh_lucide::{Icon, icon};
//!
//! view()
//!     .flex_row()
//!     .items_center()
//!     .gap_2()
//!     .text_color(move || theme().muted_foreground)
//!     .child(icon(Icon::Search))
//!     .child(text("Search"))
//! ```
//!
//! In TSX (`nsh add nsh-lucide`), import the `<Icon>` component and the
//! icons from `nsh-lucide/icons`:
//!
//! ```tsx
//! import { Icon } from "nsh-lucide";
//! import { Search } from "nsh-lucide/icons";
//!
//! <Icon icon={Search} class="text-muted-foreground" />
//! ```
//!
//! # What ships
//!
//! Every icon is a constant: in Rust an associated constant of [`Icon`],
//! in TSX an exported string of `nsh-lucide/icons`. The compiler leaves the
//! constants no code uses out of the binary, and a packaged app's bundle
//! leaves out the TSX ones it does not import. A lookup by name at run time
//! needs the icons it can find in the binary: name them with [`icon_set!`],
//! or turn on the `all` feature for [`Icon::from_name`] over the whole
//! catalog.

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
/// (`move || if dark.get() { Icon::Moon } else { Icon::Sun }`), or any
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

/// A set of icons an app finds by Lucide name at run time, holding only the
/// icons it lists:
///
/// ```ignore
/// nsh_lucide::icon_set!(pub FileIcons, [File, FileText, FileImage, Folder]);
///
/// let glyph = FileIcons::get("file-text").unwrap_or(Icon::File);
/// ```
///
/// It makes a unit struct with `ALL`, the icons in the order listed, and
/// `get(name)`, the listed icon of a current Lucide name (`None` for any
/// other).
#[macro_export]
macro_rules! icon_set {
    ($vis:vis $set:ident, [$($icon:ident),* $(,)?]) => {
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
        $vis struct $set;

        impl $set {
            /// The icons of the set, in the order listed.
            pub const ALL: &'static [$crate::Icon] = &[$($crate::Icon::$icon),*];

            /// The icon of the set with this Lucide name.
            pub fn get(name: &str) -> ::core::option::Option<$crate::Icon> {
                Self::ALL.iter().copied().find(|icon| icon.name() == name)
            }
        }
    };
}

/// What a TSX app imports from `nsh-lucide`: the `<Icon>` component. The
/// icons themselves come from `nsh-lucide/icons`, the TSX of this crate.
#[native_module("nsh-lucide")]
fn native(registry: &mut Registry) {
    registry.register::<tsx::Icon>();
}

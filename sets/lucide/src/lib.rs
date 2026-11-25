//! The [Lucide](https://lucide.dev) icons for [Native Shell](https://nsh.dev) apps, drawn by
//! [`nsh_icon`] in the text color around them: [`Lucide`].
//!
//! ```ignore
//! use nsh_icon_lucide::{Lucide, icon};
//!
//! view().child(icon(Lucide::Search))
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
//! Every icon is a constant: an app's binary holds the ones its code names,
//! and its bundle the ones its TSX imports.

mod icons;

use nsh_core::prelude::*;

pub use icons::Lucide;
/// What draws the icons, from [`nsh_icon`].
pub use nsh_icon::{Icon, icon, icon_set, labeled_icon};

/// The icons reach TSX as TSX (`nsh-icon-lucide`, `tsx/index.ts`), drawn by
/// `nsh-icon`'s `<Icon>`; this module exports nothing, and makes the crate
/// an nshm.
#[native_module("nsh-icon-lucide/native")]
fn native(_registry: &mut Registry) {}

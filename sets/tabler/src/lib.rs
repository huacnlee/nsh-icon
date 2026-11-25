//! The [Tabler Icons](https://tabler.io/icons) icons for [Native Shell](https://nsh.dev) apps, drawn by
//! [`nsh_icon`] in the text color around them: [`Tabler`].
//!
//! ```ignore
//! use nsh_icon_tabler::{Tabler, icon};
//!
//! view().child(icon(Tabler::Search))
//! ```
//!
//! In TSX (`nsh add nsh-icon-tabler`):
//!
//! ```tsx
//! import { Icon } from "nsh-icon";
//! import { Search } from "nsh-icon-tabler";
//!
//! <Icon icon={Search} class="text-muted-foreground" />
//! ```
//!
//! Every icon is a constant: an app's binary holds the ones its code names,
//! and its bundle the ones its TSX imports.

mod icons;

use nsh_core::prelude::*;

pub use icons::Tabler;
/// What draws the icons, from [`nsh_icon`].
pub use nsh_icon::{Icon, icon, icon_set, labeled_icon};

/// The icons reach TSX as TSX (`nsh-icon-tabler`, `tsx/index.ts`), drawn by
/// `nsh-icon`'s `<Icon>`; this module exports nothing, and makes the crate
/// an nshm.
#[native_module("nsh-icon-tabler/native")]
fn native(_registry: &mut Registry) {}

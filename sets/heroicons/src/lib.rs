//! The [Heroicons](https://heroicons.com) icons for [Native Shell](https://nsh.dev) apps, drawn by
//! [`nsh_icon`] in the text color around them: [`Heroicons`].
//!
//! ```ignore
//! use nsh_icon_heroicons::{Heroicons, icon};
//!
//! view().child(icon(Heroicons::MagnifyingGlass))
//! ```
//!
//! In TSX (`nsh add nsh-icon-heroicons`):
//!
//! ```tsx
//! import { Icon } from "nsh-icon";
//! import { MagnifyingGlass } from "nsh-icon-heroicons";
//!
//! <Icon icon={MagnifyingGlass} class="text-muted-foreground" />
//! ```
//!
//! Every icon is a constant: an app's binary holds the ones its code names,
//! and its bundle the ones its TSX imports.

mod icons;

use nsh_core::prelude::*;

pub use icons::Heroicons;
/// What draws the icons, from [`nsh_icon`].
pub use nsh_icon::{Icon, icon, icon_set, labeled_icon};

/// The icons reach TSX as TSX (`nsh-icon-heroicons`, `tsx/index.ts`), drawn by
/// `nsh-icon`'s `<Icon>`; this module exports nothing, and makes the crate
/// an nshm.
#[native_module("nsh-icon-heroicons/native")]
fn native(_registry: &mut Registry) {}

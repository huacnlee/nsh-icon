//! The [Phosphor](https://phosphoricons.com) icons for [Native Shell](https://nsh.dev) apps, drawn by
//! [`nsh_icon`] in the text color around them: [`Phosphor`].
//!
//! ```ignore
//! use nsh_icon_phosphor::{Phosphor, icon};
//!
//! view().child(icon(Phosphor::MagnifyingGlass))
//! ```
//!
//! In TSX (`nsh add nsh-icon-phosphor`):
//!
//! ```tsx
//! import { Icon } from "nsh-icon";
//! import { MagnifyingGlass } from "nsh-icon-phosphor";
//!
//! <Icon icon={MagnifyingGlass} class="text-muted-foreground" />
//! ```
//!
//! Every icon is a constant: an app's binary holds the ones its code names,
//! and its bundle the ones its TSX imports.

mod icons;

use nsh_core::prelude::*;

pub use icons::Phosphor;
/// What draws the icons, from [`nsh_icon`].
pub use nsh_icon::{Icon, icon, icon_set, labeled_icon};

/// The icons reach TSX as TSX (`nsh-icon-phosphor`, `tsx/index.ts`), drawn by
/// `nsh-icon`'s `<Icon>`; this module exports nothing, and makes the crate
/// an nshm.
#[native_module("nsh-icon-phosphor/native")]
fn native(_registry: &mut Registry) {}

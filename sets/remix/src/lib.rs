//! The [Remix Icon](https://remixicon.com) icons for [Native Shell](https://nsh.dev) apps, drawn by
//! [`nsh_icon`] in the text color around them: [`Remix`].
//!
//! ```ignore
//! use nsh_icon_remix::{Remix, icon};
//!
//! view().child(icon(Remix::SearchLine))
//! ```
//!
//! In TSX (`nsh add nsh-icon-remix`):
//!
//! ```tsx
//! import { Icon } from "nsh-icon";
//! import { SearchLine } from "nsh-icon-remix";
//!
//! <Icon icon={SearchLine} class="text-muted-foreground" />
//! ```
//!
//! Every icon is a constant: an app's binary holds the ones its code names,
//! and its bundle the ones its TSX imports.

mod icons;

use nsh_core::prelude::*;

pub use icons::Remix;
/// What draws the icons, from [`nsh_icon`].
pub use nsh_icon::{Icon, icon, icon_set, labeled_icon};

/// The icons reach TSX as TSX (`nsh-icon-remix`, `tsx/index.ts`), drawn by
/// `nsh-icon`'s `<Icon>`; this module exports nothing, and makes the crate
/// an nshm.
#[native_module("nsh-icon-remix/native")]
fn native(_registry: &mut Registry) {}

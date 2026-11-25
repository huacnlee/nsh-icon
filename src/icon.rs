//! `Icon`: a Lucide icon as a compile-time SVG `data:` URI.

use nsh_core::element::{ImageSrc, IntoReactive, Reactive, SharedString};

/// A Lucide icon: `Icon::Search`, `Icon::ChevronDown`, named as
/// [lucide.dev](https://lucide.dev/icons) and `lucide-react` name them.
///
/// Each icon is an associated constant holding its SVG document as a
/// `&'static str` `data:` URI, written when the crate builds. An app's
/// binary holds the icons its code names and none of the others: a constant
/// no code uses is never compiled in. Nothing is allocated or encoded when
/// one is drawn; NSH parses its drawing once and shares it with every view
/// that shows the icon, while one does.
///
/// Lucide's old names (`Home`, `Edit`) are kept as deprecated aliases of the
/// current ones (`House`, `SquarePen`).
///
/// A lookup by name at run time needs a table of the icons it can find:
/// list them with [`icon_set!`](crate::icon_set), or turn on the `all`
/// feature for [`Icon::ALL`] and [`Icon::from_name`], which link the
/// whole catalog.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Icon {
    name: &'static str,
    data_uri: &'static str,
}

impl Icon {
    const fn new(name: &'static str, data_uri: &'static str) -> Self {
        Self { name, data_uri }
    }

    /// Its Lucide name (`chevron-down`).
    pub const fn name(self) -> &'static str {
        self.name
    }

    /// Its SVG document as a `data:image/svg+xml` URI, stroked with
    /// `currentColor`: an image source for [`nsh_core::element::svg`] (drawn
    /// in the element's text color) and [`nsh_core::element::image`].
    pub const fn data_uri(self) -> &'static str {
        self.data_uri
    }

    /// Its image source: [`Self::data_uri`], borrowed, not copied.
    pub fn src(self) -> ImageSrc {
        ImageSrc::from(self.data_uri)
    }

    /// Its image source drawn with lines `width` units wide out of its 24
    /// (Lucide's own are 2). It makes a new source, and a drawing NSH caches
    /// apart from the icon's at width 2.
    pub fn src_with_stroke_width(self, width: f32) -> ImageSrc {
        let width = if width.is_finite() && width > 0.0 { width } else { 2.0 };
        if width == 2.0 {
            return self.src();
        }
        ImageSrc::from(
            self.data_uri
                .replacen("stroke-width='2'", &format!("stroke-width='{width}'"), 1),
        )
    }
}

/// Its [`Icon::data_uri`], borrowed. It makes an icon an [`ImageSrc`] too, and
/// what `<Icon>`'s `icon` prop takes: `.icon(Icon::Search)`.
impl From<Icon> for SharedString {
    fn from(icon: Icon) -> Self {
        SharedString::from(icon.data_uri)
    }
}

impl IntoReactive<ImageSrc> for Icon {
    fn into_reactive(self) -> Reactive<ImageSrc> {
        Reactive::Static(self.src())
    }
}

#[cfg(feature = "all")]
impl Icon {
    /// Every icon of the catalog, in Lucide name order (`all` feature: it
    /// links every icon).
    pub const ALL: &'static [Icon] = ALL;

    /// The icon of a Lucide name, current (`house`) or old (`home`); `None`
    /// for a name Lucide does not have (`all` feature: it links every icon).
    pub fn from_name(name: &str) -> Option<Icon> {
        lookup(name)
    }
}

include!(concat!(env!("OUT_DIR"), "/icons.rs"));

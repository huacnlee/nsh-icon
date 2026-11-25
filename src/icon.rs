//! `Icon`: an icon of any set as a compile-time SVG `data:` URI.

use nsh_core::element::{ImageSrc, IntoReactive, Reactive, SharedString};

/// An icon: `Lucide::Search` of `nsh-icon-lucide`, `Heroicons::HomeSolid` of
/// `nsh-icon-heroicons`, or one of an app's own.
///
/// Each icon of a set is an associated constant holding its SVG document as
/// a `&'static str` `data:` URI. An app's binary holds the icons its code
/// names and none of the others: a constant no code uses is never compiled
/// in. Nothing is allocated or encoded when one is drawn; NSH parses its
/// drawing once and shares it with every view that shows the icon, while
/// one does.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Icon {
    name: &'static str,
    data_uri: &'static str,
}

impl Icon {
    /// An icon named `name` whose SVG document is `data_uri`, a
    /// `data:image/svg+xml` URI drawn with `currentColor`, as the sets'
    /// constants are made:
    ///
    /// ```
    /// use nsh_icon::Icon;
    ///
    /// const DOT: Icon = Icon::new(
    ///     "dot",
    ///     "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' \
    ///      fill='currentColor'%3E%3Ccircle cx='12' cy='12' r='4'/%3E%3C/svg%3E",
    /// );
    /// ```
    pub const fn new(name: &'static str, data_uri: &'static str) -> Self {
        Self { name, data_uri }
    }

    /// Its name in its set (`chevron-down`).
    pub const fn name(self) -> &'static str {
        self.name
    }

    /// Its SVG document as a `data:image/svg+xml` URI, drawn with
    /// `currentColor`: an image source for [`nsh_core::element::svg`] (drawn
    /// in the element's text color) and [`nsh_core::element::image`].
    pub const fn data_uri(self) -> &'static str {
        self.data_uri
    }

    /// Its image source: [`Self::data_uri`], borrowed, not copied.
    pub fn src(self) -> ImageSrc {
        ImageSrc::from(self.data_uri)
    }

    /// Its image source drawn with lines `width` units wide, for an icon of
    /// a stroked set (Lucide's lines are 2 of its 24 units). It makes a new
    /// source, which NSH caches apart from the icon's own; a filled icon, or
    /// a width that is not a positive number, keeps the icon's.
    pub fn src_with_stroke_width(self, width: f32) -> ImageSrc {
        match with_stroke_width(self.data_uri, width) {
            Some(uri) => ImageSrc::from(uri),
            None => self.src(),
        }
    }
}

/// `uri` with every `stroke-width='…'` set to `width`; `None` when it has
/// none, has that width already, or `width` is not a positive number.
pub(crate) fn with_stroke_width(uri: &str, width: f32) -> Option<String> {
    const ATTRIBUTE: &str = "stroke-width='";
    if !(width.is_finite() && width > 0.0) || !uri.contains(ATTRIBUTE) {
        return None;
    }
    let width = width.to_string();
    let mut out = String::with_capacity(uri.len() + 8);
    let mut rest = uri;
    let mut changed = false;
    while let Some(start) = rest.find(ATTRIBUTE) {
        let value = start + ATTRIBUTE.len();
        let end = value + rest[value..].find('\'')?;
        changed |= rest[value..end] != width;
        out.push_str(&rest[..value]);
        out.push_str(&width);
        rest = &rest[end..];
    }
    out.push_str(rest);
    changed.then_some(out)
}

/// Its [`Icon::data_uri`], borrowed. It makes an icon an [`ImageSrc`] too, and
/// what `<Icon>`'s `icon` prop takes: `.icon(Lucide::Search)`.
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

#[cfg(test)]
mod tests {
    use super::with_stroke_width;

    #[test]
    fn a_stroke_width_replaces_every_one() {
        let uri = "data:image/svg+xml,%3Csvg stroke-width='2'%3E%3Cpath stroke-width='2'/%3E%3C/svg%3E";
        assert_eq!(
            with_stroke_width(uri, 1.5).as_deref(),
            Some("data:image/svg+xml,%3Csvg stroke-width='1.5'%3E%3Cpath stroke-width='1.5'/%3E%3C/svg%3E")
        );
        assert_eq!(with_stroke_width(uri, 2.0), None);
        assert_eq!(with_stroke_width(uri, f32::NAN), None);
        assert_eq!(with_stroke_width(uri, 0.0), None);
        assert_eq!(with_stroke_width("data:image/svg+xml,%3Csvg fill='currentColor'/%3E", 1.0), None);
    }
}

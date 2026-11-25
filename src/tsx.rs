//! `<Icon>`: the component TSX imports from `nsh-icon`.

use nsh_core::element::ImageSrc;
use nsh_core::prelude::*;

/// The props of [`Icon`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct IconProps {
    /// The icon: a constant of a set (`Search` of `nsh-icon-lucide`), or the
    /// URL or `data:` URI of any SVG drawn with `currentColor`. Rust sets an
    /// [`crate::Icon`]: `.icon(Lucide::Search)`.
    pub icon: Prop<SharedString>,
    /// Its width and height in points (16, the line height of body text,
    /// when left out).
    #[prop(default = 16.0)]
    pub size: Prop<f32>,
    /// The width of its lines in the icon's own units, for an icon of a
    /// stroked set (Lucide's and Tabler's are 2 of 24, Heroicons' outline
    /// 1.5); left out, the set's own.
    pub stroke_width: Prop<Option<f32>>,
    /// What screen readers say for an icon that stands alone and means
    /// something; left out, the icon is decorative and hidden from them.
    pub label: Prop<Option<SharedString>>,
}

/// An icon, drawn in the inherited text color: tint it with a text
/// class (`class="text-primary"`).
#[component]
pub fn Icon(props: IconProps) -> impl IntoElement {
    svg(move || {
        let src = props.icon.get();
        match props.stroke_width.get().and_then(|width| crate::icon::with_stroke_width(&src, width)) {
            Some(uri) => ImageSrc::from(uri),
            None => ImageSrc::from(src),
        }
    })
    .size(move || px(props.size.get()))
    .flex_shrink_0()
    .accessible(move || props.label.get().is_some())
    .accessibility_elements_hidden(move || props.label.get().is_none())
    .role(AccessibilityRole::Image)
    .label(move || props.label.get().unwrap_or_default())
}

//! `<Icon>`: the component TSX imports from `nsh-lucide`.

use nsh_core::element::ImageSrc;
use nsh_core::prelude::*;

/// The props of [`Icon`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct IconProps {
    /// The icon: a constant of `nsh-lucide/icons` (`Search`), or the URL or
    /// `data:` URI of any SVG drawn with `currentColor`. Rust sets an
    /// [`crate::Icon`]: `.icon(Icon::Search)`.
    pub icon: Prop<SharedString>,
    /// Its width and height in points (16, the line height of body text,
    /// when left out).
    #[prop(default = 16.0)]
    pub size: Prop<f32>,
    /// The width of its lines, out of the icon's 24 units (Lucide's 2 when
    /// left out). It applies to Lucide icons.
    pub stroke_width: Prop<Option<f32>>,
    /// What screen readers say for an icon that stands alone and means
    /// something; left out, the icon is decorative and hidden from them.
    pub label: Prop<Option<SharedString>>,
}

/// A Lucide icon, stroked in the inherited text color: tint it with a text
/// class (`class="text-primary"`).
#[component]
pub fn Icon(props: IconProps) -> impl IntoElement {
    svg(move || {
        let src = props.icon.get();
        match props.stroke_width.get() {
            Some(width) if width.is_finite() && width > 0.0 && width != 2.0 => ImageSrc::from(
                src.as_str()
                    .replacen("stroke-width='2'", &format!("stroke-width='{width}'"), 1),
            ),
            _ => ImageSrc::from(src),
        }
    })
    .size(move || px(props.size.get()))
    .flex_shrink_0()
    .accessible(move || props.label.get().is_some())
    .accessibility_elements_hidden(move || props.label.get().is_none())
    .role(AccessibilityRole::Image)
    .label(move || props.label.get().unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use nsh_engine::{EngineConfig, Root};
    use nsh_testing::Headless;

    #[component]
    fn Icons() -> impl IntoElement {
        view()
            .child(
                icon()
                    .icon(crate::Icon::Bell)
                    .size(20.0)
                    .stroke_width(1.5)
                    .label("Notifications"),
            )
            .child(icon().icon(crate::Icon::Search).test_id("search"))
    }

    #[test]
    fn the_tsx_icon_draws_and_is_read_out_when_labeled() -> Result<(), String> {
        let mut app = Headless::start(EngineConfig::new(Root::of::<Icons>())).unwrap();
        app.wait_for("label:Notifications")?;
        let search = app
            .screen()
            .layout_nodes()
            .into_iter()
            .find(|node| node.test_id.as_deref() == Some("search"))
            .expect("the decorative icon")
            .bounds;
        assert_eq!((search.size.width.0, search.size.height.0), (16.0, 16.0));
        Ok(())
    }
}

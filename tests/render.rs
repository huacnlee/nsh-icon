//! Icons drawn by a real engine, headless.

use nsh_core::prelude::*;
use nsh_engine::{EngineConfig, Root};
use nsh_lucide::{Icon, icon, labeled_icon};
use nsh_testing::Headless;

#[component]
fn Icons() -> impl IntoElement {
    let dark = signal(false);
    view()
        .gap(px(8.0))
        .child(icon(Icon::Search).test_id("search"))
        .child(icon(Icon::Settings).size(px(24.0)).test_id("settings"))
        .child(labeled_icon(Icon::Trash, "Delete"))
        .child(
            view()
                .test_id("theme")
                .on_press(move || dark.update(|d| *d = !*d))
                .child(icon(move || if dark.get() { Icon::Moon } else { Icon::Sun })),
        )
}

fn bounds(app: &Headless, id: &str) -> nsh_core::geometry::Bounds<nsh_core::geometry::Points> {
    app.screen()
        .layout_nodes()
        .into_iter()
        .find(|node| node.test_id.as_deref() == Some(id))
        .unwrap_or_else(|| panic!("no view {id}"))
        .bounds
}

#[test]
fn icons_lay_out_at_their_size() -> Result<(), String> {
    let mut app = Headless::start(EngineConfig::new(Root::of::<Icons>())).unwrap();
    app.first_render()?;
    let search = bounds(&app, "search");
    assert_eq!((search.size.width.0, search.size.height.0), (16.0, 16.0));
    let settings = bounds(&app, "settings");
    assert_eq!((settings.size.width.0, settings.size.height.0), (24.0, 24.0));
    Ok(())
}

#[test]
fn a_labeled_icon_is_read_out() -> Result<(), String> {
    let mut app = Headless::start(EngineConfig::new(Root::of::<Icons>())).unwrap();
    app.wait_for("label:Delete")?;
    Ok(())
}

#[test]
fn a_reactive_icon_follows_its_signal() -> Result<(), String> {
    let mut app = Headless::start(EngineConfig::new(Root::of::<Icons>())).unwrap();
    app.first_render()?;
    app.press("id:theme")?;
    app.settle();
    Ok(())
}

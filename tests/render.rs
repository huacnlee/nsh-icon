//! Icons drawn by a real engine, headless.

use nsh_core::prelude::*;
use nsh_engine::{EngineConfig, Root};
use nsh_icon::{icon, labeled_icon, tsx};
use nsh_icon_lucide::Lucide;
use nsh_testing::Headless;

#[component]
fn Icons() -> impl IntoElement {
    let dark = signal(false);
    view()
        .gap(px(8.0))
        .child(icon(Lucide::Search).test_id("search"))
        .child(icon(Lucide::Settings).size(px(24.0)).test_id("settings"))
        .child(labeled_icon(Lucide::Trash, "Delete"))
        .child(
            view()
                .test_id("theme")
                .on_press(move || dark.update(|d| *d = !*d))
                .child(icon(move || if dark.get() { Lucide::Moon } else { Lucide::Sun })),
        )
        // The TSX component, as Rust builds it.
        .child(tsx::icon().icon(Lucide::Bell).size(20.0).stroke_width(1.5).label("Notifications"))
        .child(tsx::icon().icon(Lucide::Search).test_id("tsx-search"))
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
    let tsx_search = bounds(&app, "tsx-search");
    assert_eq!((tsx_search.size.width.0, tsx_search.size.height.0), (16.0, 16.0));
    Ok(())
}

#[test]
fn a_labeled_icon_is_read_out() -> Result<(), String> {
    let mut app = Headless::start(EngineConfig::new(Root::of::<Icons>())).unwrap();
    app.wait_for("label:Delete")?;
    app.wait_for("label:Notifications")?;
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

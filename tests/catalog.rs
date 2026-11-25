//! The catalog: names, aliases, the `data:` URIs and what a binary links.

use nsh_core::element::ImageSrc;
use nsh_lucide::Icon;

#[test]
fn icons_are_named_as_lucide_names_them() {
    assert_eq!(Icon::Search.name(), "search");
    assert_eq!(Icon::ChevronDown.name(), "chevron-down");
    assert_eq!(Icon::ArrowDown01.name(), "arrow-down-0-1");
    assert_ne!(Icon::Search, Icon::Check);
}

#[test]
#[allow(deprecated)]
fn old_names_are_the_current_icons() {
    assert_eq!(Icon::Home, Icon::House);
    assert_eq!(Icon::Home.name(), "house");
}

#[test]
fn an_icon_is_an_svg_document_stroked_in_the_text_color() {
    let uri = Icon::Search.data_uri();
    let (mime, bytes) = nsh_core::image::decode_data_uri(uri).unwrap();
    assert_eq!(mime, "image/svg+xml");
    let svg = String::from_utf8(bytes).unwrap();
    assert_eq!(
        svg,
        "<svg xmlns='http://www.w3.org/2000/svg' width='24' height='24' viewBox='0 0 24 24' \
         fill='none' stroke='currentColor' stroke-width='2' stroke-linecap='round' \
         stroke-linejoin='round'><path d='m21 21-4.34-4.34'/><circle cx='11' cy='11' r='8'/></svg>"
    );
    // The same document as `ImageSrc::svg_markup` encodes, in fewer bytes.
    let decoded_markup = {
        let src = ImageSrc::svg_markup(&svg);
        let uri = format!("{src:?}");
        assert!(uri.contains("data:image/svg+xml,"), "{uri}");
        uri.len()
    };
    assert!(uri.len() < decoded_markup);
}

#[test]
fn an_icon_is_its_image_source() {
    assert_eq!(ImageSrc::from(Icon::Check), ImageSrc::from(Icon::Check.data_uri()));
    assert_eq!(Icon::Check.src_with_stroke_width(2.0), Icon::Check.src());
    assert_eq!(Icon::Check.src_with_stroke_width(f32::NAN), Icon::Check.src());
    let thin = Icon::Check.src_with_stroke_width(1.5);
    assert_eq!(
        thin,
        ImageSrc::from(Icon::Check.data_uri().replace("stroke-width='2'", "stroke-width='1.5'"))
    );
}

nsh_lucide::icon_set!(FileIcons, [File, FileText, Folder]);

#[test]
fn a_set_finds_its_icons_by_name() {
    assert_eq!(FileIcons::ALL, &[Icon::File, Icon::FileText, Icon::Folder]);
    assert_eq!(FileIcons::get("file-text"), Some(Icon::FileText));
    assert_eq!(FileIcons::get("search"), None);
}

#[cfg(feature = "all")]
#[test]
fn the_whole_catalog_is_found_by_name() {
    assert_eq!(Icon::ALL.len(), 1818);
    assert!(Icon::ALL.windows(2).all(|w| w[0].name() < w[1].name()));
    assert_eq!(Icon::from_name("search"), Some(Icon::Search));
    assert_eq!(Icon::from_name("home"), Some(Icon::House));
    assert_eq!(Icon::from_name("no-such-icon"), None);
}

/// The binary of this test holds the icons it names, and none of the others:
/// what an app pays for is the icons it uses.
#[cfg(not(feature = "all"))]
#[test]
fn a_binary_links_only_the_icons_it_names() {
    let binary = std::fs::read(std::env::current_exe().unwrap()).unwrap();
    // The paths are written backwards, so that this test's own strings are
    // not what it finds.
    let holds = |reversed: &str| {
        let needle: Vec<u8> = reversed.bytes().rev().collect();
        binary.windows(needle.len()).any(|w| w == needle)
    };
    // `search`, which this file names (its path is unique to it).
    assert!(holds("43.4-43.4-12 12m"));
    // Paths unique to icons this test never names.
    for (icon, reversed) in [
        ("rocket", "5-0 5-0 26.1-80.1c2-4 55.-30.3s5v51 21M"),
        ("bike", "2h3 2 3-4 3-3l41V5.71 21M"),
        ("zap", "9 9l-165.1-474.2-00 5.1 5.1a4 419.51M"),
    ] {
        assert!(!holds(reversed), "{icon} is in the binary");
    }
}

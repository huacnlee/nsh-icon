//! The Lucide set: names, aliases, the `data:` URIs and what a binary links.

use nsh_core::element::ImageSrc;
use nsh_icon_lucide::Lucide;

#[test]
fn icons_are_named_as_lucide_names_them() {
    assert_eq!(Lucide::Search.name(), "search");
    assert_eq!(Lucide::ChevronDown.name(), "chevron-down");
    assert_eq!(Lucide::ArrowDown01.name(), "arrow-down-0-1");
    assert_ne!(Lucide::Search, Lucide::Check);
}

#[test]
#[allow(deprecated)]
fn old_names_are_the_current_icons() {
    assert_eq!(Lucide::Home, Lucide::House);
    assert_eq!(Lucide::Home.name(), "house");
}

#[test]
fn an_icon_is_an_svg_document_stroked_in_the_text_color() {
    let uri = Lucide::Search.data_uri();
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
    assert_eq!(ImageSrc::from(Lucide::Check), ImageSrc::from(Lucide::Check.data_uri()));
    assert_eq!(Lucide::Check.src_with_stroke_width(2.0), Lucide::Check.src());
    assert_eq!(Lucide::Check.src_with_stroke_width(f32::NAN), Lucide::Check.src());
    let thin = Lucide::Check.src_with_stroke_width(1.5);
    assert_eq!(
        thin,
        ImageSrc::from(Lucide::Check.data_uri().replace("stroke-width='2'", "stroke-width='1.5'"))
    );
}

nsh_icon_lucide::icon_set!(FileIcons, [Lucide::File, Lucide::FileText, Lucide::Folder]);

#[test]
fn a_set_finds_its_icons_by_name() {
    assert_eq!(FileIcons::ALL, &[Lucide::File, Lucide::FileText, Lucide::Folder]);
    assert_eq!(FileIcons::get("file-text"), Some(Lucide::FileText));
    assert_eq!(FileIcons::get("search"), None);
}

#[cfg(feature = "all")]
#[test]
fn the_whole_catalog_is_found_by_name() {
    assert_eq!(Lucide::ALL.len(), 1818);
    assert!(Lucide::ALL.windows(2).all(|w| w[0].name() < w[1].name()));
    assert_eq!(Lucide::from_name("search"), Some(Lucide::Search));
    assert_eq!(Lucide::from_name("home"), Some(Lucide::House));
    assert_eq!(Lucide::from_name("no-such-icon"), None);
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

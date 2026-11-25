//! Every set's generated sources are current. After a sync, write them with
//! `NSH_ICON_UPDATE=1 cargo test -p nsh-icon-codegen`.

use nsh_icon_codegen::{SETS, data_uri, generated, pascal_case, read_catalog, root};

#[test]
fn the_generated_sources_are_current() {
    let update = std::env::var_os("NSH_ICON_UPDATE").is_some();
    let mut stale = Vec::new();
    for set in SETS {
        for (path, content) in generated(set) {
            if update {
                std::fs::create_dir_all(path.parent().unwrap()).unwrap();
                std::fs::write(&path, &content).unwrap();
            }
            if std::fs::read_to_string(&path).ok().as_deref() != Some(content.as_str()) {
                stale.push(path.strip_prefix(root()).unwrap().display().to_string());
            }
        }
    }
    assert!(stale.is_empty(), "out of date: {stale:?}; run NSH_ICON_UPDATE=1 cargo test -p nsh-icon-codegen");
}

#[test]
fn every_set_is_an_nshm_exporting_its_icons_to_tsx() {
    for set in SETS {
        let dir = root().join("sets").join(set.id);
        let manifest = std::fs::read_to_string(dir.join("nsh.json")).unwrap();
        let export = format!(r#""nsh-icon-{}": "tsx/index.ts""#, set.id);
        assert!(manifest.contains(&export), "{}: missing {export}", set.id);
        let cargo = std::fs::read_to_string(dir.join("Cargo.toml")).unwrap();
        assert!(cargo.contains(&format!("name = \"nsh-icon-{}\"", set.id)), "{}: crate name", set.id);
        assert!(dir.join("LICENSE").is_file(), "{}: LICENSE", set.id);
    }
}

#[test]
fn every_icon_is_an_svg_in_the_text_color() {
    for set in SETS {
        let catalog = read_catalog(set);
        for icon in &catalog.icons {
            let uri = &icon.data_uri;
            assert!(uri.starts_with("data:image/svg+xml,%3Csvg "), "{}: {}", set.id, icon.name);
            assert!(uri.ends_with("%3C/svg%3E"), "{}: {}", set.id, icon.name);
            assert!(uri.contains("currentColor"), "{}: {} has its own color", set.id, icon.name);
            assert!(!uri.contains(['"', '\n', '<', '>', '#']), "{}: {}", set.id, icon.name);
        }
    }
}

#[test]
fn names_become_identifiers() {
    assert_eq!(pascal_case("chevron-down"), "ChevronDown");
    assert_eq!(pascal_case("arrow-down-0-1"), "ArrowDown01");
    assert_eq!(pascal_case("home-20-solid"), "Home20Solid");
    assert_eq!(pascal_case("24-hours-fill"), "Icon24HoursFill");
}

#[test]
fn an_svg_file_becomes_a_compact_uri() {
    let lucide = "<svg\n  xmlns=\"http://www.w3.org/2000/svg\"\n  viewBox=\"0 0 24 24\"\n  stroke-width=\"2\"\n>\n  <path d=\"m21 21-4.34-4.34\" />\n</svg>\n";
    assert_eq!(
        data_uri("search", lucide),
        "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' stroke-width='2'%3E%3Cpath d='m21 21-4.34-4.34'/%3E%3C/svg%3E"
    );
    let heroicons = "<svg xmlns=\"http://www.w3.org/2000/svg\" fill=\"none\" viewBox=\"0 0 24 24\" aria-hidden=\"true\" data-slot=\"icon\">\n  <path d=\"M1 1\"/>\n</svg>\n";
    assert_eq!(
        data_uri("x", heroicons),
        "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24'%3E%3Cpath d='M1 1'/%3E%3C/svg%3E"
    );
    let tabler = "<!--\ntags: [home]\n-->\n<svg class=\"icon\" viewBox=\"0 0 24 24\"><path d=\"M5 12l-2 0\" /></svg>";
    assert_eq!(
        data_uri("home", tabler),
        "data:image/svg+xml,%3Csvg viewBox='0 0 24 24'%3E%3Cpath d='M5 12l-2 0'/%3E%3C/svg%3E"
    );
}

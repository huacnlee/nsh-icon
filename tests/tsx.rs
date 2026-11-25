//! `tsx/icons.ts`, what TSX imports from `nsh-lucide/icons`, is generated
//! from the same catalog as `Icon`. Regenerate it after a Lucide sync:
//! `NSH_LUCIDE_UPDATE=1 cargo test --test tsx`.

#[allow(dead_code)]
#[path = "../codegen.rs"]
mod codegen;

use std::path::Path;

#[test]
fn the_tsx_icons_are_current() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let generated = codegen::tsx_source(&codegen::read_catalog(root));
    let path = root.join("tsx/icons.ts");
    if std::env::var_os("NSH_LUCIDE_UPDATE").is_some() {
        std::fs::write(&path, &generated).unwrap();
    }
    let current = std::fs::read_to_string(&path).unwrap_or_default();
    assert!(
        current == generated,
        "tsx/icons.ts is out of date: run NSH_LUCIDE_UPDATE=1 cargo test --test tsx"
    );
}

#[test]
fn rust_and_tsx_hold_the_same_icons() {
    let ts = std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("tsx/icons.ts")).unwrap();
    let line = format!("export const Search: string = {:?};", nsh_lucide::Icon::Search.data_uri());
    assert!(ts.contains(&line), "missing {line}");
    assert!(ts.contains("export const Home: string = House;"));
    let catalog = codegen::read_catalog(Path::new(env!("CARGO_MANIFEST_DIR")));
    assert_eq!(catalog.icons.len(), 1818);
    assert_eq!(
        ts.matches("\nexport const ").count(),
        catalog.icons.len() + catalog.aliases.len()
    );
}

#[test]
fn the_nshm_exports_the_icons_to_tsx() {
    let manifest = std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("nsh.json")).unwrap();
    assert!(manifest.contains(r#""nsh-lucide/icons": "tsx/icons.ts""#), "{manifest}");
}

// The one generator of the catalog, from `icons/` (written by
// `script/sync-lucide.ts`): the Rust constants (`build.rs`, into `OUT_DIR`)
// and the TSX constants (`tsx/icons.ts`, checked in and kept current by
// `tests/tsx.rs`). Both name an icon the same way (`ChevronDown`), and both
// hold the same `data:` URI, so a mixed app draws one cached drawing for it.
//
// Plain `std` only: `build.rs` includes this file with `include!`.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::path::Path;

/// The attributes every Lucide SVG's root carries (checked for each file).
const LUCIDE_ROOT: &str = r#"xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round""#;

/// One icon of the catalog.
pub struct CatalogIcon {
    /// Lucide's name (`chevron-down`).
    pub name: String,
    /// Its Rust constant and TSX export (`ChevronDown`).
    pub ident: String,
    /// The whole SVG document as a compact `data:` URI.
    pub data_uri: String,
}

/// A deprecated name Lucide still answers to.
pub struct CatalogAlias {
    pub name: String,
    pub ident: String,
    /// The icon's `ident`.
    pub target: String,
    pub target_name: String,
}

pub struct Catalog {
    pub version: String,
    pub icons: Vec<CatalogIcon>,
    pub aliases: Vec<CatalogAlias>,
}

/// `chevron-down` → `ChevronDown`, `arrow-down-0-1` → `ArrowDown01`.
pub fn pascal_case(name: &str) -> String {
    let ident: String = name
        .split(['-', '_', '.'])
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            let first = chars.next().unwrap();
            format!("{}{}", first.to_ascii_uppercase(), chars.as_str().to_ascii_lowercase())
        })
        .collect();
    assert!(
        ident.starts_with(|c: char| c.is_ascii_alphabetic()) && ident.chars().all(|c| c.is_ascii_alphanumeric()),
        "icon name {name:?} makes no identifier"
    );
    ident
}

/// The SVG document of a Lucide file as a `data:` URI: the same root, single
/// quotes, no whitespace between elements, and only `<` and `>` escaped,
/// which every NSH backend and browser reads.
pub fn data_uri(name: &str, svg: &str) -> String {
    let svg = svg.trim();
    let open_end = svg.find('>').unwrap_or_else(|| panic!("{name}: no <svg> root"));
    let root = svg[..open_end]
        .strip_prefix("<svg")
        .unwrap_or_else(|| panic!("{name}: no <svg> root"));
    let root = root.split_whitespace().collect::<Vec<_>>().join(" ");
    assert_eq!(root, LUCIDE_ROOT, "{name}: an <svg> root other than Lucide's");
    let body = svg[open_end + 1..]
        .strip_suffix("</svg>")
        .unwrap_or_else(|| panic!("{name}: no </svg>"));
    let mut markup = format!("<svg {}>", LUCIDE_ROOT.replace('"', "'"));
    for line in body.lines() {
        markup.push_str(&line.trim().replace(" />", "/>"));
    }
    markup.push_str("</svg>");
    for c in ['\'', '%', '#', '\\'] {
        assert!(!body.contains(c), "{name}: {c:?} in the drawing");
    }
    let markup = markup.replace('"', "'");
    let mut uri = String::with_capacity(markup.len() + 40);
    uri.push_str("data:image/svg+xml,");
    for c in markup.chars() {
        match c {
            '<' => uri.push_str("%3C"),
            '>' => uri.push_str("%3E"),
            c => uri.push(c),
        }
    }
    uri
}

/// Reads `icons/` (the SVG files and `aliases.txt`) and `lucide.json`.
pub fn read_catalog(root: &Path) -> Catalog {
    let manifest = std::fs::read_to_string(root.join("lucide.json")).expect("lucide.json");
    let version = manifest
        .split("\"version\"")
        .nth(1)
        .and_then(|rest| rest.split('"').nth(1))
        .expect("lucide.json: version")
        .to_owned();

    let mut by_name = BTreeMap::new();
    for entry in std::fs::read_dir(root.join("icons")).expect("icons directory") {
        let path = entry.expect("icons directory entry").path();
        if path.extension().and_then(|e| e.to_str()) != Some("svg") {
            continue;
        }
        let name = path.file_stem().unwrap().to_str().expect("UTF-8 icon name").to_owned();
        let svg = std::fs::read_to_string(&path).expect("icon file");
        by_name.insert(name.clone(), data_uri(&name, &svg));
    }
    assert!(!by_name.is_empty(), "icons/ has no SVG files");

    let mut idents = BTreeSet::new();
    let mut icons = Vec::with_capacity(by_name.len());
    for (name, data_uri) in by_name {
        let ident = pascal_case(&name);
        assert!(idents.insert(ident.clone()), "two icons are named {ident}");
        icons.push(CatalogIcon { name, ident, data_uri });
    }

    let mut aliases = Vec::new();
    let list = std::fs::read_to_string(root.join("icons/aliases.txt")).unwrap_or_default();
    for line in list.lines().filter(|l| !l.is_empty() && !l.starts_with('#')) {
        let (name, target_name) = line.split_once(' ').expect("aliases.txt: <alias> <icon>");
        let target = icons
            .iter()
            .find(|icon| icon.name == target_name)
            .unwrap_or_else(|| panic!("alias {name} of a missing icon {target_name}"));
        let ident = pascal_case(name);
        // An old name that is another icon's name now stays that icon.
        if !idents.insert(ident.clone()) {
            continue;
        }
        aliases.push(CatalogAlias {
            name: name.to_owned(),
            ident,
            target: target.ident.clone(),
            target_name: target_name.to_owned(),
        });
    }
    Catalog { version, icons, aliases }
}

/// The constants of `Icon`, included by `src/icon_name.rs`.
pub fn rust_source(catalog: &Catalog) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "// Generated from Lucide {} by codegen.rs.\n", catalog.version);
    out.push_str("#[allow(non_upper_case_globals)]\nimpl Icon {\n");
    for icon in &catalog.icons {
        let _ = writeln!(
            out,
            "    /// Lucide [`{name}`](https://lucide.dev/icons/{name}).\n    pub const {ident}: Icon = Icon::new({name:?}, {uri:?});",
            name = icon.name,
            ident = icon.ident,
            uri = icon.data_uri,
        );
    }
    for alias in &catalog.aliases {
        let _ = writeln!(
            out,
            "    /// Lucide's old name `{name}` of [`Icon::{target}`].\n    #[deprecated(note = \"Lucide renamed `{name}` to `{target_name}`: use `Icon::{target}`\")]\n    pub const {ident}: Icon = Icon::{target};",
            name = alias.name,
            ident = alias.ident,
            target = alias.target,
            target_name = alias.target_name,
        );
    }
    out.push_str("}\n\n");

    out.push_str("/// Every icon of the catalog, in name order.\n#[cfg(feature = \"all\")]\npub(crate) const ALL: &[Icon] = &[\n");
    for icon in &catalog.icons {
        let _ = writeln!(out, "    Icon::{},", icon.ident);
    }
    out.push_str("];\n\n");

    out.push_str("/// An icon of the catalog by its Lucide name, current or old.\n#[cfg(feature = \"all\")]\npub(crate) fn lookup(name: &str) -> Option<Icon> {\n    Some(match name {\n");
    for icon in &catalog.icons {
        let _ = writeln!(out, "        {:?} => Icon::{},", icon.name, icon.ident);
    }
    for alias in &catalog.aliases {
        let _ = writeln!(out, "        {:?} => Icon::{},", alias.name, alias.target);
    }
    out.push_str("        _ => return None,\n    })\n}\n");
    out
}

/// `tsx/icons.ts`: what TSX imports from `nsh-lucide/icons`.
pub fn tsx_source(catalog: &Catalog) -> String {
    let mut out = String::new();
    let _ = write!(
        out,
        concat!(
            "// Lucide {version} icons for `<Icon>` from \"nsh-lucide\". Generated by codegen.rs: do not edit.\n",
            "//\n",
            "//   import {{ Icon }} from \"nsh-lucide\";\n",
            "//   import {{ ChevronDown, Search }} from \"nsh-lucide/icons\";\n",
            "//   <Icon icon={{Search}} class=\"text-muted-foreground\" />\n",
            "//\n",
            "// Each icon is a constant SVG `data:` URI. A packaged app's bundle keeps the ones it\n",
            "// imports and leaves out the rest. Lucide: ISC License, see LICENSE-LUCIDE.\n\n",
        ),
        version = catalog.version
    );
    for icon in &catalog.icons {
        let _ = writeln!(
            out,
            "/** Lucide [`{name}`](https://lucide.dev/icons/{name}). */\nexport const {ident}: string = {uri:?};",
            name = icon.name,
            ident = icon.ident,
            uri = icon.data_uri,
        );
    }
    for alias in &catalog.aliases {
        let _ = writeln!(
            out,
            "/** @deprecated Lucide renamed `{name}` to `{target_name}`: use {{@link {target}}}. */\nexport const {ident}: string = {target};",
            name = alias.name,
            ident = alias.ident,
            target = alias.target,
            target_name = alias.target_name,
        );
    }
    out
}

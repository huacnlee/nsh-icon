//! The generator of the icon sets' sources (`publish = false`).
//!
//! Each set's SVG files are in `sets/<set>/icons/` (synced from its pinned
//! release by `script/sync.ts`). This writes the set crate's Rust constants
//! (`sets/<set>/src/icons.rs`) and TSX constants (`sets/<set>/tsx/index.ts`),
//! both checked in, so the published crates need neither the SVG files nor a
//! build script. Both name an icon the same way (`ChevronDown`) and hold the
//! same `data:` URI, so a mixed app draws one cached drawing for it.
//!
//! `cargo test -p nsh-icon-codegen` checks they are current;
//! `NSH_ICON_UPDATE=1 cargo test -p nsh-icon-codegen` writes them.

use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

/// An icon set: a crate `nsh-icon-<id>` in `sets/<id>/`.
pub struct SetSpec {
    /// Its directory under `sets/` and its crate's suffix (`lucide`).
    pub id: &'static str,
    /// The Rust type holding its icons (`Lucide`).
    pub ty: &'static str,
    /// Its name in prose (`Lucide`).
    pub title: &'static str,
    /// Its project's site.
    pub home: &'static str,
    /// The page of an icon, `{name}` replaced by its name, when the site has one.
    pub page: Option<&'static str>,
    /// Its license, as SPDX.
    pub license: &'static str,
    /// How its names say an icon's style, for the docs.
    pub styles: &'static str,
}

/// Every set, in the order the docs list them.
pub const SETS: &[SetSpec] = &[
    SetSpec {
        id: "lucide",
        ty: "Lucide",
        title: "Lucide",
        home: "https://lucide.dev",
        page: Some("https://lucide.dev/icons/{name}"),
        license: "ISC",
        styles: "One style: lines 2 units wide on a 24-unit grid.",
    },
    SetSpec {
        id: "heroicons",
        ty: "Heroicons",
        title: "Heroicons",
        home: "https://heroicons.com",
        page: None,
        license: "MIT",
        styles: "Four styles, as suffixes of the name: outline (24, no suffix: `Home`), solid (24: \
                 `HomeSolid`), mini (20: `Home20Solid`) and micro (16: `Home16Solid`).",
    },
    SetSpec {
        id: "tabler",
        ty: "Tabler",
        title: "Tabler Icons",
        home: "https://tabler.io/icons",
        page: None,
        license: "MIT",
        styles: "Two styles: outline (no suffix: `Home`) and filled (`HomeFilled`), which fewer icons have.",
    },
    SetSpec {
        id: "phosphor",
        ty: "Phosphor",
        title: "Phosphor",
        home: "https://phosphoricons.com",
        page: None,
        license: "MIT",
        styles: "Six weights, as suffixes of the name: regular (no suffix: `House`), `HouseThin`, \
                 `HouseLight`, `HouseBold`, `HouseFill` and `HouseDuotone`.",
    },
    SetSpec {
        id: "remix",
        ty: "Remix",
        title: "Remix Icon",
        home: "https://remixicon.com",
        page: None,
        license: "Apache-2.0",
        styles: "Two styles, as suffixes of the name: line (`HomeLine`) and fill (`HomeFill`). \
                 Names that start with a digit start with `Icon` (`Icon24HoursFill`).",
    },
];

/// One icon of a set.
pub struct CatalogIcon {
    /// Its upstream name (`chevron-down`).
    pub name: String,
    /// Its Rust constant and TSX export (`ChevronDown`).
    pub ident: String,
    /// The whole SVG document as a compact `data:` URI.
    pub data_uri: String,
}

/// An old name the set still answers to.
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

/// The repository's root.
pub fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf()
}

/// `chevron-down` → `ChevronDown`, `arrow-down-0-1` → `ArrowDown01`,
/// `24-hours-fill` → `Icon24HoursFill`.
pub fn pascal_case(name: &str) -> String {
    let mut ident: String = name
        .split(['-', '_', '.'])
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            let first = chars.next().unwrap();
            format!("{}{}", first.to_ascii_uppercase(), chars.as_str().to_ascii_lowercase())
        })
        .collect();
    if ident.starts_with(|c: char| c.is_ascii_digit()) {
        ident.insert_str(0, "Icon");
    }
    assert!(
        ident.starts_with(|c: char| c.is_ascii_alphabetic()) && ident.chars().all(|c| c.is_ascii_alphanumeric()),
        "icon name {name:?} makes no identifier"
    );
    ident
}

/// Removes `name="..."` from an element's attributes.
fn remove_attribute(tag: &str, name: &str) -> String {
    let needle = format!(" {name}=\"");
    match tag.find(&needle) {
        Some(start) => {
            let value = start + needle.len();
            let end = value + tag[value..].find('"').expect("closed attribute") + 1;
            format!("{}{}", &tag[..start], &tag[end..])
        }
        None => tag.to_owned(),
    }
}

/// An SVG file as a compact `data:` URI: its elements as written, without
/// comments, the whitespace between them or attributes that only matter in
/// a web page (`class`, `aria-hidden`, `data-slot`); single quotes, and only
/// `<`, `>`, `#` and `%` escaped, which every NSH backend and browser reads.
pub fn data_uri(name: &str, svg: &str) -> String {
    // Comments (Tabler's metadata).
    let mut text = String::with_capacity(svg.len());
    let mut rest = svg;
    while let Some(start) = rest.find("<!--") {
        text.push_str(&rest[..start]);
        let end = rest[start..].find("-->").unwrap_or_else(|| panic!("{name}: unclosed comment"));
        rest = &rest[start + end + 3..];
    }
    text.push_str(rest);
    let text = text.trim();
    assert!(text.starts_with("<svg") && text.ends_with("</svg>"), "{name}: not an <svg> document");
    assert!(!text.contains('\''), "{name}: a single quote in the drawing");

    // Whitespace: one space where there was any, none around tags.
    let mut collapsed = String::with_capacity(text.len());
    for c in text.chars() {
        if c.is_whitespace() {
            if !collapsed.ends_with(' ') {
                collapsed.push(' ');
            }
        } else {
            collapsed.push(c);
        }
    }
    let markup = collapsed.replace("> <", "><").replace(" />", "/>").replace(" >", ">");
    let root_end = markup.find('>').unwrap();
    let mut root = markup[..root_end].to_owned();
    for attribute in ["class", "aria-hidden", "data-slot"] {
        root = remove_attribute(&root, attribute);
    }
    let markup = format!("{root}{}", &markup[root_end..]).replace('"', "'");

    let mut uri = String::with_capacity(markup.len() + 40);
    uri.push_str("data:image/svg+xml,");
    for c in markup.chars() {
        match c {
            '<' => uri.push_str("%3C"),
            '>' => uri.push_str("%3E"),
            '#' => uri.push_str("%23"),
            '%' => uri.push_str("%25"),
            c => uri.push(c),
        }
    }
    uri
}

/// Reads `sets/<id>/`: `set.json`, the SVG files of `icons/` and
/// `aliases.txt` (`<old name> <icon>` lines), when there is one.
pub fn read_catalog(set: &SetSpec) -> Catalog {
    let dir = root().join("sets").join(set.id);
    let manifest = std::fs::read_to_string(dir.join("set.json")).expect("set.json");
    let field = |key: &str| {
        manifest
            .split(&format!("\"{key}\""))
            .nth(1)
            .and_then(|rest| rest.split([':', ',', '\n']).nth(1))
            .map(|v| v.trim().trim_matches('"').to_owned())
            .unwrap_or_else(|| panic!("{}: set.json has no {key}", set.id))
    };
    let version = field("version");
    let icon_count: usize = field("icon_count").parse().expect("set.json: icon_count");

    let mut files = Vec::new();
    for entry in std::fs::read_dir(dir.join("icons")).expect("icons directory") {
        let path = entry.expect("icons directory entry").path();
        if path.extension().and_then(|e| e.to_str()) == Some("svg") {
            files.push(path);
        }
    }
    files.sort_by(|a, b| a.file_stem().cmp(&b.file_stem()));
    assert_eq!(files.len(), icon_count, "{}: icons/ against set.json's icon_count", set.id);

    let mut idents = BTreeSet::new();
    let mut icons = Vec::with_capacity(files.len());
    for path in files {
        let name = path.file_stem().unwrap().to_str().expect("UTF-8 icon name").to_owned();
        let svg = std::fs::read_to_string(&path).expect("icon file");
        let ident = pascal_case(&name);
        assert!(idents.insert(ident.clone()), "two {} icons are named {ident}", set.title);
        icons.push(CatalogIcon { data_uri: data_uri(&name, &svg), name, ident });
    }

    let mut aliases = Vec::new();
    let list = std::fs::read_to_string(dir.join("aliases.txt")).unwrap_or_default();
    for line in list.lines().filter(|l| !l.is_empty() && !l.starts_with('#')) {
        let (name, target_name) = line.split_once(' ').expect("aliases.txt: <alias> <icon>");
        // An old name of an icon the release no longer has stays out.
        let Some(target) = icons.iter().find(|icon| icon.name == target_name) else {
            continue;
        };
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

fn icon_doc(set: &SetSpec, name: &str) -> String {
    match set.page {
        Some(page) => format!("{} [`{name}`]({}).", set.title, page.replace("{name}", name)),
        None => format!("{} `{name}`.", set.title),
    }
}

/// `sets/<id>/src/icons.rs`: the set's type and its constants; `ALL` and
/// `from_name` behind the crate's `all` feature.
pub fn rust_source(set: &SetSpec, catalog: &Catalog) -> String {
    let SetSpec { ty, title, home, license, styles, .. } = set;
    let mut out = String::new();
    let _ = writeln!(
        out,
        concat!(
            "// {title} {version}, generated by the nsh-icon-codegen crate: do not edit.\n\n",
            "use nsh_icon::Icon;\n\n",
            "/// The [{title}]({home}) icons ({title} {version}, {count} icons, {license} License):\n",
            "/// `{ty}::Search` is an [`Icon`], drawn by [`icon`](nsh_icon::icon). Each is a\n",
            "/// constant: a binary holds the ones its code names, and none of the others.\n",
            "///\n",
            "/// Names are the upstream names in PascalCase (`chevron-down` is `ChevronDown`).\n",
            "/// {styles}\n",
            "///\n",
            "/// The `all` feature adds `ALL` and `from_name`, a lookup by name over the whole\n",
            "/// set, which links every icon; [`icon_set!`](nsh_icon::icon_set) links only those\n",
            "/// it lists.\n",
            "#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]\n",
            "pub struct {ty};\n",
        ),
        title = title,
        version = catalog.version,
        home = home,
        count = catalog.icons.len(),
        license = license,
        styles = styles,
        ty = ty,
    );
    let _ = writeln!(out, "#[allow(non_upper_case_globals)]\nimpl {ty} {{");
    for icon in &catalog.icons {
        let _ = writeln!(
            out,
            "    /// {doc}\n    pub const {ident}: Icon = Icon::new({name:?}, {uri:?});",
            doc = icon_doc(set, &icon.name),
            name = icon.name,
            ident = icon.ident,
            uri = icon.data_uri,
        );
    }
    for alias in &catalog.aliases {
        let _ = writeln!(
            out,
            "    /// {title}'s old name `{name}` of [`{ty}::{target}`].\n    #[deprecated(note = \"{title} renamed `{name}` to `{target_name}`: use `{ty}::{target}`\")]\n    pub const {ident}: Icon = {ty}::{target};",
            name = alias.name,
            ident = alias.ident,
            target = alias.target,
            target_name = alias.target_name,
        );
    }
    out.push_str("}\n\n");

    let _ = writeln!(out, "#[cfg(feature = \"all\")]\nimpl {ty} {{");
    out.push_str("    /// Every icon of the set, in name order (`all` feature).\n    pub const ALL: &'static [Icon] = &[\n");
    for icon in &catalog.icons {
        let _ = writeln!(out, "        {ty}::{},", icon.ident);
    }
    out.push_str("    ];\n\n");
    out.push_str(
        "    /// The icon of a name, current or old (`all` feature); `None` for a\n    /// name the set does not have.\n    pub fn from_name(name: &str) -> Option<Icon> {\n        Some(match name {\n",
    );
    for icon in &catalog.icons {
        let _ = writeln!(out, "            {:?} => {ty}::{},", icon.name, icon.ident);
    }
    for alias in &catalog.aliases {
        let _ = writeln!(out, "            {:?} => {ty}::{},", alias.name, alias.target);
    }
    out.push_str("            _ => return None,\n        })\n    }\n}\n");
    out
}

/// `sets/<id>/tsx/index.ts`: what TSX imports from `nsh-icon-<id>`.
pub fn tsx_source(set: &SetSpec, catalog: &Catalog) -> String {
    let SetSpec { id, title, license, .. } = set;
    let mut out = String::new();
    let _ = write!(
        out,
        concat!(
            "// {title} {version} icons for `<Icon>` from \"nsh-icon\". Generated by the nsh-icon-codegen\n",
            "// crate: do not edit.\n",
            "//\n",
            "//   import {{ Icon }} from \"nsh-icon\";\n",
            "//   import {{ Search }} from \"nsh-icon-{id}\";\n",
            "//   <Icon icon={{Search}} class=\"text-muted-foreground\" />\n",
            "//\n",
            "// Each icon is a constant SVG `data:` URI. A packaged app's bundle keeps the ones it\n",
            "// imports and leaves out the rest. {title}: {license} License, see LICENSE.\n\n",
        ),
        title = title,
        version = catalog.version,
        id = id,
        license = license,
    );
    for icon in &catalog.icons {
        let _ = writeln!(
            out,
            "/** {doc} */\nexport const {ident}: string = {uri:?};",
            doc = icon_doc(set, &icon.name),
            ident = icon.ident,
            uri = icon.data_uri,
        );
    }
    for alias in &catalog.aliases {
        let _ = writeln!(
            out,
            "/** @deprecated {title} renamed `{name}` to `{target_name}`: use {{@link {target}}}. */\nexport const {ident}: string = {target};",
            name = alias.name,
            ident = alias.ident,
            target = alias.target,
            target_name = alias.target_name,
        );
    }
    out
}

/// The generated files of a set and their contents.
pub fn generated(set: &SetSpec) -> Vec<(PathBuf, String)> {
    let catalog = read_catalog(set);
    let dir = root().join("sets").join(set.id);
    vec![
        (dir.join("src/icons.rs"), rust_source(set, &catalog)),
        (dir.join("tsx/index.ts"), tsx_source(set, &catalog)),
    ]
}

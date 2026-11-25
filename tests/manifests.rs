//! Each set's nsh.json asks for the nsh-icon of this release: `nsh add` resolves
//! a set's nshm dependencies from it, so a stale requirement stops apps adding both.

use std::fs;
use std::path::Path;

#[test]
fn every_set_requires_this_minor_of_nsh_icon() {
    let version = env!("CARGO_PKG_VERSION");
    let minor = version.rsplit_once('.').unwrap().0;
    let wanted = format!("\"nsh-icon\": \"^{minor}\"");
    let sets = Path::new(env!("CARGO_MANIFEST_DIR")).join("sets");
    let mut checked = 0;
    for entry in fs::read_dir(&sets).unwrap() {
        let manifest = entry.unwrap().path().join("nsh.json");
        let text = fs::read_to_string(&manifest).unwrap();
        assert!(
            text.contains(&wanted),
            "{} should require {wanted} (nsh-icon is {version})",
            manifest.display()
        );
        checked += 1;
    }
    assert!(checked >= 5, "found only {checked} sets");
}

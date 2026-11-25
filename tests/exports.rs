//! What `#[native_module("nsh-icon")]` exports to TSX, read from the
//! declarations `nsh` writes for an app (`.nsh/types/nsh-icon.d.ts`).

fn declarations() -> String {
    nsh_core::element::declarations(&nsh_icon::registry(), &[])
}

#[test]
fn the_module_is_imported_as_nsh_icon() {
    assert_eq!(nsh_icon::registry().specifier(), Some("nsh-icon"));
}

#[test]
fn icon_is_declared_with_its_props() {
    let ts = declarations();
    for line in [
        "export declare const Icon: ((props: IconProps) => JSX.Element);",
        "  icon: string;",
        "  size?: number;",
        "  strokeWidth?: number",
        "  label?: string",
    ] {
        assert!(ts.contains(line), "missing {line:?} in\n{ts}");
    }
}

#[test]
fn every_export_is_documented() {
    assert_eq!(nsh_icon::registry().undocumented(), Vec::<String>::new());
}

//! What `#[native_module("nsh-lucide")]` exports to TSX, read from the
//! declarations `nsh` writes for an app (`.nsh/types/nsh-lucide.d.ts`).

fn declarations() -> String {
    nsh_core::element::declarations(&nsh_lucide::registry(), &[])
}

#[test]
fn the_module_is_imported_as_nsh_lucide() {
    assert_eq!(nsh_lucide::registry().specifier(), Some("nsh-lucide"));
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
    assert_eq!(nsh_lucide::registry().undocumented(), Vec::<String>::new());
}

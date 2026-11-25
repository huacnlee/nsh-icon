// Generates `Icon`'s constants from `icons/` (see codegen.rs).

include!("codegen.rs");

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=codegen.rs");
    println!("cargo:rerun-if-changed=lucide.json");
    println!("cargo:rerun-if-changed=icons");
    let root = std::path::PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let out = std::path::PathBuf::from(std::env::var_os("OUT_DIR").expect("OUT_DIR"));
    let catalog = read_catalog(&root);
    std::fs::write(out.join("icons.rs"), rust_source(&catalog)).expect("write icons.rs");
}

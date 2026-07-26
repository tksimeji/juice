use std::path::PathBuf;

fn main() {
    let linker_script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("linker.ld");

    println!("cargo:rustc-link-arg=-T{}", linker_script.display());

    println!("cargo:rerun-if-changed=linker.ld");
}

use std::{env, path::PathBuf};

fn main() {
    let vendor_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("vendor");
    let nuked_opn2_dir = vendor_dir.join("Nuked-OPN2");

    cargo_config();
    bindings(&nuked_opn2_dir);
    build(&nuked_opn2_dir);
}

fn cargo_config() {
    println!("cargo:rustc-link-lib=static=nuked-opn2");
}

fn build(source_dir: &PathBuf) {
    cc::Build::new()
        .file(&source_dir.join("ym3438.c"))
        .compile("nuked-opn2");
}

fn bindings(source_dir: &PathBuf) {
    let header_file = source_dir.join("ym3438.h");
    let bindings_file = PathBuf::from(env::var("OUT_DIR").unwrap()).join("nuked-opn2-bindings.rs");

    bindgen::Builder::default()
        .header(header_file.to_str().unwrap())
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(bindings_file)
        .expect("Couldn't write bindings!");
}
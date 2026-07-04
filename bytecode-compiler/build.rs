use std::path::PathBuf;
use walkdir::WalkDir;

fn main() {
    let mut build = cc::Build::new();

    build.include("../c/include");

    for entry in WalkDir::new("../c/src")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "c"))
    {
        build.file(entry.path());
    }

    build.compile("clib");

    println!("cargo:rustc-link-lib=static=clib");

    println!("cargo:rerun-if-changed=c/src");
    println!("cargo:rerun-if-changed=c/include");

    let bindings = bindgen::Builder::default()
        .header("../c/include/compiler/runtime.h")
        .clang_arg("-I../c/include")
        .generate()
        .expect("Failed to generate bindings");

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}


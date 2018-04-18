extern crate bindgen;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;

fn main() {
    let lib = pkg_config::probe_library("vips").unwrap();

    let mut builder = bindgen::Builder::default()
        .header("wrapper.h");

    for path in lib.include_paths.iter() {
        let path = path.as_os_str().to_str().unwrap();
        builder = builder.clang_arg(format!("-I{}", path));
    }

    let bindings =
        builder
        .blacklist_type("FP_NAN")
        .blacklist_type("FP_INFINITE")
        .blacklist_type("FP_ZERO")
        .blacklist_type("FP_SUBNORMAL")
        .blacklist_type("FP_NORMAL")
        // Finish the builder and generate the bindings.
        .generate()
        .expect("Unable to generate bindings");

    // // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

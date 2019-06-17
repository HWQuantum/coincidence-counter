extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let nix_cflags = env::var("NIX_CFLAGS_COMPILE").unwrap();
    println!("cargo:rustc-link-lib=hh400");

    let bindings_partial = bindgen::Builder::default().header("wrapper.h");
    if let Ok(nix_cflags) = env::var("NIX_CFLAGS_COMPILE") {
        let bindings = bindings_partial
            .clang_args(nix_cflags.split(" "))
            .generate()
            .expect("Unable to generate bindings");
        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");
    } else {
        let bindings = bindings_partial
            .generate()
            .expect("Unable to generate bindings");

        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");
    }
}

use std::env;
use cargo_metadata::*;
use std::path::*;

use cbindgen::Builder;

fn main() {
    let path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let meta = MetadataCommand::new()
        .manifest_path("./Cargo.toml")
        .current_dir(&path)
        .exec()
        .unwrap();

    println!("{:?}", meta);

    if cfg!(feature = "bindings") {
        // assuming target dir isn't tweaked
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        println!(
            "cargo:rustc-link-search=native={}/inst/usr/lib64",
            manifest_dir,
        );
        println!("cargo:rustc-link-lib=dylib=gobject_example");
    }

    if cfg!(feature = "capi") {
        let out = std::env::var("OUT_DIR").unwrap();
        let out = Path::new(&out);
        let out_include = out.join("capi/include/");
        std::fs::create_dir_all(&out_include).unwrap();

        Builder::new()
            .with_crate(&path)
            .with_gobject(true)
            .generate()
            .unwrap()
            .write_to_file(out_include.join("ex.h"));
    }
}

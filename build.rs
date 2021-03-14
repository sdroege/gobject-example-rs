use std::env;

fn main() {
    if cfg!(feature = "bindings") {
        // assuming target dir isn't tweaked
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let profile = env::var("PROFILE").unwrap();
        println!(
            "cargo:rustc-link-search=native={}/target/{}",
            manifest_dir, profile
        );
        println!("cargo:rustc-link-lib=dylib=gobject_example");
    }
}

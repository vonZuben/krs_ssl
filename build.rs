// build.rs

use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let krs_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    // PROFILE - release for release builds, debug for other builds.
    let build_type = env::var("PROFILE").unwrap();

    Command::new("make")
        .args(&[
            build_type])
        .status()
        .unwrap();

    println!("cargo:rustc-link-search=native={}", krs_dir + "/openssl");
    println!("cargo:rustc-link-search=native={}", out_dir);
    // println!("cargo:rustc-link-lib=static=hello");
}

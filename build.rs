extern crate cmake;
use std::env;

use cmake::Config;

fn main() {
    let dst = Config::new("rsmi")
        .define("ROCM_DIR", "/opt/rocm")
        .very_verbose(true)
        .build();
    println!("cargo:rustc-link-search=native={}/build", dst.display());

    if !cfg!(feature = "vendored") {

    } else {
        println!("cargo:rustc-link-search=native=/opt/rocm/lib");
        println!("cargo:rustc-link-search=native=/opt/rocm/lib64");
        println!("cargo:rustc-link-lib=rocm_smi64");
        println!("cargo:rustc-link-lib=rsmi64");
    }
}

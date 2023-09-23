extern crate cmake;
use std::{env, process::Command, path::Path};

use cmake::Config;

fn main() {
    let dst = Config::new("rsmi")
        .define("ROCM_DIR", "/opt/rocm")
        .very_verbose(true)
        .build();
    println!("cargo:rustc-link-search=native={}/build", dst.display());

    if cfg!(feature = "vendored") {
        if !Path::new("rocm_smi_lib/src").exists() {
            let _ = Command::new("git").args(&["submodule", "update", "--init", "rocm_smi_lib"]).status();
        }
    } else {
        println!("cargo:rustc-link-search=native=/opt/rocm/lib");
        println!("cargo:rustc-link-search=native=/opt/rocm/lib64");
        println!("cargo:rustc-link-lib=rocm_smi64");
        println!("cargo:rustc-link-lib=rsmi64");
    }

}

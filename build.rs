extern crate cmake;

use cmake::Config;

fn main() {
    let dst = Config::new("rsmi").out_dir("./rsmi").build();
    println!("cargo::rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-search=native=rsmi/build/");
    println!("cargo:rustc-link-search=native=/opt/rocm/lib");
    println!("cargo:rustc-link-search=native=/opt/rocm/lib64");
    println!("cargo:rustc-link-lib=rsmi64");
    println!("cargo:rustc-link-lib=rocm_smi64");
}
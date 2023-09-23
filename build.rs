extern crate cmake;
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command, io,
};

use cmake::Config;

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn main() {
    if cfg!(feature = "vendored") {
        if !Path::new("rocm_smi_lib/src").exists() {
            let _ = Command::new("git")
                .args(&["submodule", "update", "--init", "rocm_smi_lib"])
                .status();
        }

        let rocm_build = Config::new("rocm_smi_lib")
            .very_verbose(true)
            .build();
        let dest = rocm_build.display().to_string();
        println!("cargo:warning={}", dest);
        copy_dir_all(dest.clone()+"/lib64", dest.clone()+"/lib").unwrap();
        let dst = Config::new("rsmi")
            .define("ROCM_DIR", dest)
            .very_verbose(true)
            .build();

        println!("cargo:rustc-link-search=native={}/build", dst.display());
        println!("cargo:rustc-link-lib=rocm_smi64");
        println!("cargo:rustc-link-lib=rsmi64");
    } else {
        let dst = Config::new("rsmi")
            .define("ROCM_DIR", "/opt/rocm")
            .very_verbose(true)
            .build();

        println!("cargo:rustc-link-search=native={}/build", dst.display());
        println!("cargo:rustc-link-lib=rocm_smi64");
        println!("cargo:rustc-link-lib=rsmi64");
    }
}

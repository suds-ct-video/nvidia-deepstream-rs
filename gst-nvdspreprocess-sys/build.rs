extern crate bindgen;

use std::env;
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rerun-if-env-changed=NVDS_ROOT");
    println!("cargo:rerun-if-env-changed=CUDA_PATH");
    println!("cargo::rerun-if-changed=wrapper.h");

    let root = std::env::var("NVDS_ROOT").unwrap_or("/opt/nvidia/deepstream/deepstream".into());

    let cuda_root = std::env::var("CUDA_PATH").unwrap_or("/usr/local/cuda".into());
    let cuda_src = Path::new(&cuda_root).join("include");

    let pk = pkg_config::Config::new().probe("gstreamer-1.0").unwrap();
    for path in &pk.link_paths {
        println!("cargo:rustc-link-search={:?}", path);
    }

    println!("Got root={:?}", root);
    let root_path = std::path::Path::new(&root)
        .join("sources")
        .join("includes")
        .canonicalize()
        .unwrap();
    let gst_pre_root_path = std::path::Path::new(&root)
        .join("sources")
        .join("gst-plugins")
        .join("gst-nvdspreprocess")
        .canonicalize()
        .unwrap();

    let pk_video = pkg_config::Config::new()
        .probe("gstreamer-video-1.0")
        .into_iter()
        .map(|x| x.include_paths.into_iter())
        .flatten()
        .map(|x| format!("-I{}", x.to_str().unwrap()));

    cc::Build::new()
        .cpp(true)
        .file("wrapper.cpp")
        .include(&root_path)
        .include(&gst_pre_root_path.join("include"))
        .includes(&pk.include_paths)
        .include("src")
        .compile("nvdspreprocess-wrapper");

    println!("cargo::rustc-link-lib=nvdspreprocess-wrapper");

    let mut bindings = bindgen::Builder::default()
        .header("wrapper.cpp")
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++11")
        .allowlist_file(".*nvdspreprocess.*")
        .allowlist_file("wrapper.cpp")
        .clang_arg(format!("-I{}", root_path.display()))
        .clang_arg(format!(
            "-I{}",
            gst_pre_root_path.join("include").to_str().unwrap()
        ))
        .clang_arg(format!("-I{}", cuda_src.display()))
        .clang_arg(format!("-I{}", gst_pre_root_path.display()))
        .clang_args(
            pk.include_paths
                .iter()
                .map(|x| format!("-I{}", x.display())),
        )
        .clang_args(pk_video);
    for path in &pk.include_paths {
        bindings = bindings.clang_arg(format!("-I{}", path.to_str().unwrap()).as_str());
    }

    bindings = bindings.parse_callbacks(Box::new(bindgen::CargoCallbacks));

    let bindings = bindings.generate().expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

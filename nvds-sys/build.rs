extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let pk = pkg_config::Config::new().probe("gstreamer-1.0").unwrap();
    for path in &pk.link_paths {
        println!("cargo:rustc-link-search={:?}", path);
    }

    println!("cargo:rerun-if-env-changed=NVDS_ROOT");
    let root = std::env::var("NVDS_ROOT").unwrap_or("/opt/nvidia/deepstream/deepstream".into());
    println!("Got root={:?}", root);
    let root_path = std::path::Path::new(&root)
        .join("sources")
        .join("includes")
        .canonicalize()
        .unwrap();

    let mut bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", root_path.display()));
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

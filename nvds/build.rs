fn main() {
    let root = std::env::var("NVDS_ROOT").unwrap_or("/opt/nvidia/deepstream/deepstream".into());
    println!("cargo:rustc-link-search={}/lib/", root);

    if cfg!(feature = "logger") {
        println!("cargo:rustc-link-lib=dylib=nvds_logger");
    }

    if cfg!(feature = "meta") {
        println!("cargo:rustc-link-lib=dylib=nvdsgst_meta");
        println!("cargo:rustc-link-lib=dylib=nvds_meta");
    }

    if cfg!(feature = "obj_encode") {
        println!("cargo:rustc-link-lib=dylib=nvds_batch_jpegenc");
    }

    if cfg!(feature = "surface") {
        println!("cargo:rustc-link-lib=dylib=nvbufsurface");
    }

    if cfg!(feature = "surface_transform") {
        println!("cargo:rustc-link-lib=dylib=nvbufsurftransform");
    }

    if cfg!(feature = "utils") {
        println!("cargo:rustc-link-lib=dylib=nvds_utils");
    }

    if cfg!(feature = "yaml") {
        println!("cargo:rustc-link-lib=dylib=nvds_yml_parser");
    }
}

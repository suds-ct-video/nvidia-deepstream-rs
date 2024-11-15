NVIDIA DeepStream SDK for Rust
=====

[LICENSE (MIT)](LICENSE)

## Notice

* WIP

## Supported Versions
This crate follows the versioning strategy of [`gstreamer-rs`](https://crates.io/crates/gstreamer) in that versions of the linked library's API interface are exposed by create features.

| DeepStream Version | Crate Feature |
|--------------------|---------------|
| 7.0.0              | v6_4          |
| 6.4.0              | v6_4          |
| 6.3.0              | v6_3          |
| 6.2.0              | v6_2          |
| 6.1.1              |               |

## Building

If your deepstream installation is in a non-standard location or if you have multiple version installed installed at the same time, please set the `NVDS_ROOT` environment variable before building to specify the root of your chosen deepstream installation. By default, it's set to
```sh
export NVDS_ROOT=/opt/nvidia/deepstream/deepstream
```

## How to use

* Define in Cargo.toml

```toml
[dependencies]

nvds = { git="https://github.com/aosoft/nvidia-deepstream-rs", features = ["all"] }
```

### features

| feature name      | header file          | linking dylib           |
|-------------------|----------------------|-------------------------|
| logger            | nvds_logger.h        | nvds_logger             |
| meta              | nvdsmeta.h           | nvdsgst_meta, nvds_meta |
| obj_encode        | nvds_obj_encode.h    | nvds_batch_jpegenc      |
| surface           | nvbufsurface.h       | nvbufsurface            |
| surface_transform | nvbufsurftransform.h | nvbufsurftransform      |
| utils             | nvds_version.h       | nvds_utils              |
| yaml              | nvds_yml_parser.h    | nvds_yml_parser         |


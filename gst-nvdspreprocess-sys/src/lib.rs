#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// HACK: Somebody at nvidia forgot to make `nvdspreprocess` a C only header
// HACK: therefore we have to manually impl `Clone` as the structs aren't PODs
impl Clone for NvDsPreProcessTensorMeta {
    fn clone(&self) -> Self {
        unsafe { copy_nvds_preprocess_tensor_meta(self as *const _ as *mut _) }
    }
}

impl Clone for GstNvDsPreProcessBatchMeta {
    fn clone(&self) -> Self {
        unsafe { copy_gst_nvds_preprocess_batch_meta(self as *const _ as *mut _) }
    }
}

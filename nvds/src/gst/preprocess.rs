use crate::{meta::MetaType, WrapperExt};
use gst_nvdspreprocess_sys::NvDsRoiMeta;

#[derive(Clone)]
pub struct PreProcessBatchMeta(gst_nvdspreprocess_sys::GstNvDsPreProcessBatchMeta);

impl PreProcessBatchMeta {
    #[inline]
    #[allow(dead_code)]
    fn from_native_type(n: gst_nvdspreprocess_sys::GstNvDsPreProcessBatchMeta) -> Self {
        Self(n)
    }
}
impl crate::WrapperExt for PreProcessBatchMeta {
    type NativeType = gst_nvdspreprocess_sys::GstNvDsPreProcessBatchMeta;
    #[inline]
    fn as_native_type(&self) -> Self::NativeType {
        self.0.clone()
    }
    #[inline]
    fn from_native_type_ref(n: &Self::NativeType) -> &Self {
        unsafe { std::mem::transmute(n) }
    }
    #[inline]
    fn as_native_type_ref(&self) -> &Self::NativeType {
        &self.0
    }
    #[inline]
    fn from_native_type_mut(n: &mut Self::NativeType) -> &mut Self {
        unsafe { std::mem::transmute(n) }
    }
    #[inline]
    fn as_native_type_mut(&mut self) -> &mut Self::NativeType {
        &mut self.0
    }
    #[inline]
    unsafe fn from_native_type_ptr(n: *mut Self::NativeType) -> *mut Self {
        unsafe { std::mem::transmute(n) }
    }
    #[inline]
    unsafe fn as_native_type_ptr(&self) -> *mut Self::NativeType {
        self.as_native_type_ref() as *const _ as *mut _
    }
}

impl Default for PreProcessBatchMeta {
    fn default() -> Self {
        unsafe { std::mem::zeroed::<Self>() }
    }
}

impl PreProcessBatchMeta {
    pub fn tensor_meta(&self) -> &PreProcessTensorMeta {
        unsafe {
            let ptr =
                PreProcessTensorMeta::from_native_type_ptr(self.as_native_type_ref().tensor_meta);
            ptr.as_ref().unwrap()
        }
    }

    pub fn roi_vector(&self) -> &[NvDsRoiMeta] {
        unsafe {
            let rois_ptr = gst_nvdspreprocess_sys::get_roi_vec_ptr(
                &self.as_native_type_ref().roi_vector as *const _ as *mut _,
            );
            let rois_size = gst_nvdspreprocess_sys::get_roi_vec_size(
                &self.as_native_type_ref().roi_vector as *const _ as *mut _,
            );
            if !rois_ptr.is_null() && rois_ptr.is_aligned() {
                let raw_rois = std::slice::from_raw_parts(rois_ptr, rois_size);
                raw_rois
            } else {
                unreachable!()
            }
        }
    }

    pub fn target_unique_ids(&self) -> &[u64] {
        unsafe {
            let ptr = gst_nvdspreprocess_sys::get_target_unique_ids_ptr(
                &self.as_native_type_ref().target_unique_ids as *const _ as *mut _,
            );
            let size = gst_nvdspreprocess_sys::get_target_unique_ids_size(
                &self.as_native_type_ref().target_unique_ids as *const _ as *mut _,
            );
            if !ptr.is_null() && ptr.is_aligned() {
                let rois = std::slice::from_raw_parts(ptr, size);
                rois
            } else {
                unreachable!()
            }
        }
    }
}

pub struct PreProcessTensorMeta(gst_nvdspreprocess_sys::NvDsPreProcessTensorMeta);

impl PreProcessTensorMeta {
    #[inline]
    #[allow(dead_code)]
    fn from_native_type(n: gst_nvdspreprocess_sys::NvDsPreProcessTensorMeta) -> Self {
        Self(n)
    }
}
impl crate::WrapperExt for PreProcessTensorMeta {
    type NativeType = gst_nvdspreprocess_sys::NvDsPreProcessTensorMeta;
    #[inline]
    fn as_native_type(&self) -> Self::NativeType {
        self.0.clone()
    }
    #[inline]
    fn from_native_type_ref(n: &Self::NativeType) -> &Self {
        unsafe { std::mem::transmute(n) }
    }
    #[inline]
    fn as_native_type_ref(&self) -> &Self::NativeType {
        &self.0
    }
    #[inline]
    fn from_native_type_mut(n: &mut Self::NativeType) -> &mut Self {
        unsafe { std::mem::transmute(n) }
    }
    #[inline]
    fn as_native_type_mut(&mut self) -> &mut Self::NativeType {
        &mut self.0
    }
    #[inline]
    unsafe fn from_native_type_ptr(n: *mut Self::NativeType) -> *mut Self {
        unsafe { std::mem::transmute(n) }
    }
    #[inline]
    unsafe fn as_native_type_ptr(&self) -> *mut Self::NativeType {
        self.as_native_type_ref() as *const _ as *mut _
    }
}
impl Default for PreProcessTensorMeta {
    fn default() -> Self {
        unsafe { std::mem::zeroed::<Self>() }
    }
}

impl PreProcessTensorMeta {
    pub fn meta_id(&self) -> u32 {
        self.as_native_type_ref().meta_id
    }
    pub fn gpu_id(&self) -> u32 {
        self.as_native_type_ref().gpu_id
    }
    pub fn buffer_size(&self) -> u64 {
        self.as_native_type_ref().buffer_size
    }
    pub fn tensor_name(&self) -> Option<&str> {
        let stdstr = &self.as_native_type_ref().tensor_name;
        unsafe {
            // let len = gst_nvdspreprocess_sys::get_cstr_length(stdstr);
            let raw_cstr = gst_nvdspreprocess_sys::get_cstr(stdstr as *const _ as *mut _);
            if !raw_cstr.is_null() {
                let cstr = std::ffi::CStr::from_ptr(raw_cstr);
                cstr.to_str().ok()
            } else {
                None
            }
        }
    }
    pub fn tensor_shape(&self) -> &[i32] {
        unsafe {
            let ptr = gst_nvdspreprocess_sys::get_tensor_shape_ptr(
                &self.as_native_type_ref().tensor_shape as *const _ as *mut _,
            );
            let size = gst_nvdspreprocess_sys::get_tensor_shape_size(
                &self.as_native_type_ref().tensor_shape as *const _ as *mut _,
            );
            if !ptr.is_null() && ptr.is_aligned() {
                let rois = std::slice::from_raw_parts(ptr, size);
                rois
            } else {
                unreachable!()
            }
        }
    }
}

use crate::meta::{BaseMetaType, UserMeta};

impl UserMeta {
    /// Get [`PreProcessBatchMeta`] stored in our data if it exists
    pub fn preprocess_batch_meta(&self) -> Option<&PreProcessBatchMeta> {
        if self.base_meta().meta_type() == MetaType::Base(BaseMetaType::PreprocessBatchMeta) {
            unsafe { self.user_meta_data() }
        } else {
            None
        }
    }
}

pub mod audio;
pub mod dewarper;
pub mod latency;
pub mod optical_flow;
pub mod osd;
pub mod schema;
#[cfg(not(feature = "v6_4"))]
pub mod tracker;

use crate::WrapperExt;
use gstreamer::glib::GStr;
use std::marker::PhantomData;
use std::ptr::NonNull;

crate::wrapper_impl_ref_type!(RoiMeta, nvidia_deepstream_sys::NvDsRoiMeta);

impl RoiMeta {
    pub fn roi(&self) -> &osd::RectParams {
        osd::RectParams::from_native_type_ref(&self.as_native_type_ref().roi)
    }

    pub fn roi_polygon(&self, index: usize) -> Option<(u32, u32)> {
        let p = &self.as_native_type_ref().roi_polygon;
        if index < p.len() {
            Some((p[index][0], p[index][1]))
        } else {
            None
        }
    }

    pub fn frame_meta(&self) -> &FrameMeta {
        unsafe { FrameMeta::from_native_type_ref(&*self.as_native_type_ref().frame_meta) }
    }

    pub fn scale_ratio_x(&self) -> f64 {
        self.as_native_type_ref().scale_ratio_x
    }

    pub fn scale_ratio_y(&self) -> f64 {
        self.as_native_type_ref().scale_ratio_y
    }

    pub fn offset_left(&self) -> f64 {
        self.as_native_type_ref().offset_left
    }

    pub fn offset_top(&self) -> f64 {
        self.as_native_type_ref().scale_ratio_y
    }

    pub fn classifier_meta_list(&self) -> Option<MetaList<ClassifierMeta>> {
        NonNull::new(self.as_native_type_ref().classifier_meta_list)
            .map(|v| MetaList::<ClassifierMeta>::new(v))
    }

    pub fn roi_user_meta_list(&self) -> Option<MetaList<UserMeta>> {
        NonNull::new(self.as_native_type_ref().roi_user_meta_list)
            .map(|v| MetaList::<UserMeta>::new(v))
    }

    pub fn add_classifier_meta(&mut self, classifier_meta: &ClassifierMeta) {
        unsafe {
            nvidia_deepstream_sys::nvds_add_classifier_meta_to_roi(
                self.as_native_type_ref() as *const _ as _,
                classifier_meta.as_native_type_ref() as *const _ as _,
            );
        }
    }

    pub fn remove_classifier_meta(&mut self, classifier_meta: &ClassifierMeta) {
        unsafe {
            nvidia_deepstream_sys::nvds_remove_classifier_meta_from_roi(
                self.as_native_type_ref() as *const _ as _,
                classifier_meta.as_native_type_ref() as *const _ as _,
            );
        }
    }

    pub fn add_user_meta(&self, meta: &UserMeta) {
        unsafe {
            nvidia_deepstream_sys::nvds_add_user_meta_to_roi(
                self.as_native_type_ref() as *const _ as _,
                meta.as_native_type_ref() as *const _ as _,
            );
        }
    }

    pub fn remove_user_meta(&self, meta: &UserMeta) {
        unsafe {
            nvidia_deepstream_sys::nvds_remove_user_meta_from_roi(
                self.as_native_type_ref() as *const _ as _,
                meta.as_native_type_ref() as *const _ as _,
            );
        }
    }
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BaseMetaType {
    InvalidMeta = nvidia_deepstream_sys::NvDsMetaType_NVDS_INVALID_META as _,
    BatchMeta = nvidia_deepstream_sys::NvDsMetaType_NVDS_BATCH_META as _,
    FrameMeta = nvidia_deepstream_sys::NvDsMetaType_NVDS_FRAME_META as _,
    ObjMeta = nvidia_deepstream_sys::NvDsMetaType_NVDS_OBJ_META as _,
    DisplayMeta = nvidia_deepstream_sys::NvDsMetaType_NVDS_DISPLAY_META as _,
    ClassifierMeta = nvidia_deepstream_sys::NvDsMetaType_NVDS_CLASSIFIER_META as _,
    LabelInfoMeta = nvidia_deepstream_sys::NvDsMetaType_NVDS_LABEL_INFO_META as _,
    UserMeta = nvidia_deepstream_sys::NvDsMetaType_NVDS_USER_META as _,
    PayloadMeta = nvidia_deepstream_sys::NvDsMetaType_NVDS_PAYLOAD_META as _,
    EventMsgMeta = nvidia_deepstream_sys::NvDsMetaType_NVDS_EVENT_MSG_META as _,
    OpticalFlowMeta = nvidia_deepstream_sys::NvDsMetaType_NVDS_OPTICAL_FLOW_META as _,
    LatencyMeasurementMeta = nvidia_deepstream_sys::NvDsMetaType_NVDS_LATENCY_MEASUREMENT_META as _,
    TensorOutputMeta = nvidia_deepstream_sys::NvDsMetaType_NVDSINFER_TENSOR_OUTPUT_META as _,
    InferSegmentationMeta = nvidia_deepstream_sys::NvDsMetaType_NVDSINFER_SEGMENTATION_META as _,
    CropImageDataMeta = nvidia_deepstream_sys::NvDsMetaType_NVDS_CROP_IMAGE_META as _,
    TrackerPastFrameMeta = nvidia_deepstream_sys::NvDsMetaType_NVDS_TRACKER_PAST_FRAME_META as _,
    AudioBatchMeta = nvidia_deepstream_sys::NvDsMetaType_NVDS_AUDIO_BATCH_META as _,
    AudioFrameMeta = nvidia_deepstream_sys::NvDsMetaType_NVDS_AUDIO_FRAME_META as _,
    PreprocessFrameMeta = nvidia_deepstream_sys::NvDsMetaType_NVDS_PREPROCESS_FRAME_META as _,
    PreprocessBatchMeta = nvidia_deepstream_sys::NvDsMetaType_NVDS_PREPROCESS_BATCH_META as _,
    CustomMsgBlob = nvidia_deepstream_sys::NvDsMetaType_NVDS_CUSTOM_MSG_BLOB as _,
    ReservedMeta = nvidia_deepstream_sys::NvDsMetaType_NVDS_RESERVED_META as _,
    GstCustomMeta = nvidia_deepstream_sys::NvDsMetaType_NVDS_GST_CUSTOM_META as _,
    StartUserMeta = nvidia_deepstream_sys::NvDsMetaType_NVDS_START_USER_META as _,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MetaType {
    Base(BaseMetaType),
    User(i32),
}

impl From<nvidia_deepstream_sys::NvDsMetaType> for MetaType {
    fn from(value: nvidia_deepstream_sys::NvDsMetaType) -> Self {
        if value < nvidia_deepstream_sys::NvDsMetaType_NVDS_START_USER_META {
            MetaType::Base(unsafe { std::mem::transmute(value) })
        } else {
            MetaType::User(value as _)
        }
    }
}

impl MetaType {
    pub fn to_native_meta_type(self) -> nvidia_deepstream_sys::NvDsMetaType {
        match self {
            MetaType::Base(x) => x as _,
            MetaType::User(x) => x as _,
        }
    }
}

pub struct MetaListIterator<'a, T>
where
    T: WrapperExt,
{
    current: Option<NonNull<nvidia_deepstream_sys::GList>>,
    phantom: PhantomData<&'a T>,
}

impl<'a, T: WrapperExt> Iterator for MetaListIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|cur| unsafe {
            self.current = NonNull::new(cur.as_ref().next);
            T::from_native_type_ref(&*(cur.as_ref().data as *const T::NativeType))
        })
    }
}

pub struct MetaList<'a, T>
where
    T: WrapperExt,
{
    list: NonNull<nvidia_deepstream_sys::GList>,
    phantom: PhantomData<&'a T>,
}

impl<'a, T: WrapperExt> MetaList<'a, T> {
    pub fn new(list: NonNull<nvidia_deepstream_sys::GList>) -> MetaList<'a, T> {
        MetaList {
            list,
            phantom: PhantomData,
        }
    }

    pub fn iter(&self) -> MetaListIterator<T> {
        MetaListIterator::<T> {
            current: Some(self.list),
            phantom: PhantomData,
        }
    }
}

crate::wrapper_impl_ref_type!(Meta, nvidia_deepstream_sys::NvDsMeta);

crate::wrapper_impl_ref_type!(MetaPool, nvidia_deepstream_sys::NvDsMetaPool);

impl MetaPool {
    pub fn meta_type(&self) -> MetaType {
        self.as_native_type_ref().meta_type.into()
    }

    pub fn max_elements_in_pool(&self) -> u32 {
        self.as_native_type_ref().max_elements_in_pool
    }

    pub fn element_size(&self) -> u32 {
        self.as_native_type_ref().element_size
    }

    pub fn num_empty_elements(&self) -> u32 {
        self.as_native_type_ref().num_empty_elements
    }

    pub fn num_full_elements(&self) -> u32 {
        self.as_native_type_ref().num_full_elements
    }

    pub fn empty_list(&self) -> Option<MetaList<Meta>> {
        NonNull::new(self.as_native_type_ref().empty_list).map(|v| MetaList::<Meta>::new(v))
    }

    pub fn full_list(&self) -> Option<MetaList<Meta>> {
        NonNull::new(self.as_native_type_ref().full_list).map(|v| MetaList::<Meta>::new(v))
    }
}

crate::wrapper_impl_ref_type!(BaseMeta, nvidia_deepstream_sys::NvDsBaseMeta);

impl BaseMeta {
    pub fn batch_meta(&self) -> Option<&BatchMeta> {
        NonNull::new(self.as_native_type_ref().batch_meta)
            .map(|p| unsafe { BatchMeta::from_native_type_ref(p.as_ref()) })
    }

    pub fn batch_meta_mut(&mut self) -> Option<&mut BatchMeta> {
        NonNull::new(self.as_native_type_ref().batch_meta)
            .map(|mut p| unsafe { BatchMeta::from_native_type_mut(p.as_mut()) })
    }

    pub fn meta_type(&self) -> MetaType {
        self.as_native_type_ref().meta_type.into()
    }

    pub unsafe fn user_context(&self) -> *mut () {
        self.as_native_type_ref().uContext as _
    }
}

crate::wrapper_impl_ref_type!(BatchMeta, nvidia_deepstream_sys::NvDsBatchMeta);

pub trait BatchMetaExt {
    fn base_meta<'a>(&'a self) -> &'a BaseMeta;
    fn max_frames_in_batch(&self) -> u32;
    fn num_frames_in_batch(&self) -> u32;
    fn frame_meta_pool(&self) -> &MetaPool;
    fn obj_meta_pool(&self) -> &MetaPool;
    fn classifier_meta_pool(&self) -> &MetaPool;
    fn display_meta_pool(&self) -> &MetaPool;
    fn user_meta_pool(&self) -> &MetaPool;
    fn label_info_meta_pool(&self) -> &MetaPool;
    fn frame_meta_list(&self) -> MetaList<FrameMeta>;
    fn batch_user_meta_list(&self) -> MetaList<UserMeta>;
    fn acquire_meta_lock(&mut self);
    fn release_meta_lock(&mut self);
    fn acquire_obj_meta_from_pool(&self) -> Option<&mut ObjectMeta>;
    fn acquire_classifier_meta_from_pool(&self) -> Option<&mut ClassifierMeta>;
    fn acquire_label_info_meta_from_pool(&self) -> Option<&mut LabelInfo>;
    fn acquire_user_meta_from_pool(&self) -> Option<&mut UserMeta>;
    fn acquire_display_meta_from_pool(&self) -> Option<&mut DisplayMeta>;
    fn get_current_metadata_info(&self) -> bool;
    fn clear_meta_list(&mut self, meta_list: &MetaList<UserMeta>, meta_pool: &MetaPool);
}

impl crate::mem::NvdsDrop for BatchMeta {
    fn drop(p: NonNull<Self::NativeType>) {
        unsafe {
            nvidia_deepstream_sys::nvds_destroy_batch_meta(p.as_ptr());
        }
    }
}

impl BatchMeta {
    pub fn create(max_batch_size: u32) -> Option<crate::mem::NvdsBox<BatchMeta>> {
        crate::mem::NvdsBox::new(|| unsafe {
            NonNull::new(nvidia_deepstream_sys::nvds_create_batch_meta(
                max_batch_size,
            ))
        })
    }

    pub fn acquire_frame_meta_from_pool(&self) -> Option<&mut FrameMeta> {
        unsafe {
            NonNull::new(nvidia_deepstream_sys::nvds_acquire_frame_meta_from_pool(
                self.as_native_type_ptr(),
            ))
            .map(|mut p| FrameMeta::from_native_type_mut(p.as_mut()))
        }
    }

    pub fn add_frame_meta(&mut self, meta: &FrameMeta) {
        unsafe {
            nvidia_deepstream_sys::nvds_add_frame_meta_to_batch(
                self.as_native_type_ref() as *const _ as _,
                meta.as_native_type_ref() as *const _ as _,
            );
        }
    }

    pub fn remove_frame_meta(&mut self, meta: &FrameMeta) {
        unsafe {
            nvidia_deepstream_sys::nvds_remove_frame_meta_from_batch(
                self.as_native_type_ref() as *const _ as _,
                meta.as_native_type_ref() as *const _ as _,
            );
        }
    }

    pub fn add_user_meta(&self, meta: &UserMeta) {
        unsafe {
            nvidia_deepstream_sys::nvds_add_user_meta_to_batch(
                self.as_native_type_ref() as *const _ as _,
                meta.as_native_type_ref() as *const _ as _,
            );
        }
    }

    pub fn remove_user_meta(&self, meta: &UserMeta) {
        unsafe {
            nvidia_deepstream_sys::nvds_remove_user_meta_from_batch(
                self.as_native_type_ref() as *const _ as _,
                meta.as_native_type_ref() as *const _ as _,
            );
        }
    }

    pub fn clear_frame_meta_list(&mut self, meta_list: &MetaList<FrameMeta>) {
        unsafe {
            nvidia_deepstream_sys::nvds_clear_frame_meta_list(
                self.as_native_type_mut() as _,
                meta_list.list.as_ptr(),
            );
        }
    }

    pub fn clear_user_meta_list(&mut self, meta_list: &MetaList<UserMeta>) {
        unsafe {
            nvidia_deepstream_sys::nvds_clear_batch_user_meta_list(
                self.as_native_type_mut() as _,
                meta_list.list.as_ptr(),
            );
        }
    }
}

impl<T: WrapperExt<NativeType = nvidia_deepstream_sys::NvDsBatchMeta>> BatchMetaExt for T {
    fn base_meta(&self) -> &BaseMeta {
        BaseMeta::from_native_type_ref(&self.as_native_type_ref().base_meta)
    }

    fn max_frames_in_batch(&self) -> u32 {
        self.as_native_type_ref().max_frames_in_batch
    }

    fn num_frames_in_batch(&self) -> u32 {
        self.as_native_type_ref().num_frames_in_batch
    }

    fn frame_meta_pool(&self) -> &MetaPool {
        unsafe { MetaPool::from_native_type_ref(&*self.as_native_type_ref().frame_meta_pool) }
    }

    fn obj_meta_pool(&self) -> &MetaPool {
        unsafe { MetaPool::from_native_type_ref(&*self.as_native_type_ref().obj_meta_pool) }
    }

    fn classifier_meta_pool(&self) -> &MetaPool {
        unsafe { MetaPool::from_native_type_ref(&*self.as_native_type_ref().classifier_meta_pool) }
    }

    fn display_meta_pool(&self) -> &MetaPool {
        unsafe { MetaPool::from_native_type_ref(&*self.as_native_type_ref().display_meta_pool) }
    }

    fn user_meta_pool(&self) -> &MetaPool {
        unsafe { MetaPool::from_native_type_ref(&*self.as_native_type_ref().user_meta_pool) }
    }

    fn label_info_meta_pool(&self) -> &MetaPool {
        unsafe { MetaPool::from_native_type_ref(&*self.as_native_type_ref().label_info_meta_pool) }
    }

    fn frame_meta_list(&self) -> MetaList<FrameMeta> {
        MetaList::<FrameMeta>::new(NonNull::new(self.as_native_type_ref().frame_meta_list).unwrap())
    }

    fn batch_user_meta_list(&self) -> MetaList<UserMeta> {
        MetaList::<UserMeta>::new(
            NonNull::new(self.as_native_type_ref().batch_user_meta_list).unwrap(),
        )
    }

    fn acquire_meta_lock(&mut self) {
        unsafe {
            nvidia_deepstream_sys::nvds_acquire_meta_lock(self.as_native_type_mut() as _);
        }
    }

    fn release_meta_lock(&mut self) {
        unsafe {
            nvidia_deepstream_sys::nvds_release_meta_lock(self.as_native_type_mut() as _);
        }
    }

    fn acquire_obj_meta_from_pool(&self) -> Option<&mut ObjectMeta> {
        unsafe {
            NonNull::new(nvidia_deepstream_sys::nvds_acquire_obj_meta_from_pool(
                self.as_native_type_ptr(),
            ))
            .map(|mut p| ObjectMeta::from_native_type_mut(p.as_mut()))
        }
    }

    fn acquire_classifier_meta_from_pool(&self) -> Option<&mut ClassifierMeta> {
        unsafe {
            NonNull::new(
                nvidia_deepstream_sys::nvds_acquire_classifier_meta_from_pool(
                    self.as_native_type_ptr(),
                ),
            )
            .map(|mut p| ClassifierMeta::from_native_type_mut(p.as_mut()))
        }
    }

    fn acquire_label_info_meta_from_pool(&self) -> Option<&mut LabelInfo> {
        unsafe {
            NonNull::new(
                nvidia_deepstream_sys::nvds_acquire_label_info_meta_from_pool(
                    self.as_native_type_ptr(),
                ),
            )
            .map(|mut p| LabelInfo::from_native_type_mut(p.as_mut()))
        }
    }

    fn acquire_user_meta_from_pool(&self) -> Option<&mut UserMeta> {
        unsafe {
            NonNull::new(nvidia_deepstream_sys::nvds_acquire_user_meta_from_pool(
                self.as_native_type_ptr(),
            ))
            .map(|mut p| UserMeta::from_native_type_mut(p.as_mut()))
        }
    }

    fn acquire_display_meta_from_pool(&self) -> Option<&mut DisplayMeta> {
        unsafe {
            NonNull::new(nvidia_deepstream_sys::nvds_acquire_display_meta_from_pool(
                self.as_native_type_ptr(),
            ))
            .map(|mut p| DisplayMeta::from_native_type_mut(p.as_mut()))
        }
    }

    fn get_current_metadata_info(&self) -> bool {
        unsafe {
            nvidia_deepstream_sys::nvds_get_current_metadata_info(
                &self.as_native_type_ref() as *const _ as _
            ) != 0
        }
    }

    fn clear_meta_list(&mut self, meta_list: &MetaList<UserMeta>, meta_pool: &MetaPool) {
        unsafe {
            nvidia_deepstream_sys::nvds_clear_meta_list(
                self.as_native_type_mut() as _,
                meta_list.list.as_ptr(),
                meta_pool.as_native_type_ref() as *const _ as _,
            );
        }
    }
}

crate::wrapper_impl_ref_type!(FrameMeta, nvidia_deepstream_sys::NvDsFrameMeta);

impl FrameMeta {
    pub fn base_meta(&self) -> &BaseMeta {
        BaseMeta::from_native_type_ref(&self.as_native_type_ref().base_meta)
    }

    pub fn pad_index(&self) -> u32 {
        self.as_native_type_ref().pad_index
    }

    pub fn batch_id(&self) -> u32 {
        self.as_native_type_ref().batch_id
    }

    pub fn frame_num(&self) -> i32 {
        self.as_native_type_ref().frame_num
    }

    pub fn buf_pts(&self) -> u64 {
        self.as_native_type_ref().buf_pts
    }

    pub fn ntp_timestamp(&self) -> u64 {
        self.as_native_type_ref().ntp_timestamp
    }
    pub fn source_id(&self) -> u32 {
        self.as_native_type_ref().source_id
    }

    pub fn num_surfaces_per_frame(&self) -> i32 {
        self.as_native_type_ref().num_surfaces_per_frame
    }

    pub fn source_frame_width(&self) -> u32 {
        self.as_native_type_ref().source_frame_width
    }

    pub fn source_frame_height(&self) -> u32 {
        self.as_native_type_ref().source_frame_height
    }

    pub fn surface_type(&self) -> u32 {
        self.as_native_type_ref().surface_type
    }

    pub fn surface_index(&self) -> u32 {
        self.as_native_type_ref().surface_index
    }

    pub fn num_obj_meta(&self) -> u32 {
        self.as_native_type_ref().num_obj_meta
    }

    pub fn infer_done(&self) -> bool {
        self.as_native_type_ref().bInferDone != 0
    }

    pub fn obj_meta_list(&self) -> Option<MetaList<ObjectMeta>> {
        NonNull::new(self.as_native_type_ref().obj_meta_list)
            .map(|v| MetaList::<ObjectMeta>::new(v))
    }

    pub fn display_meta_list(&self) -> Option<MetaList<DisplayMeta>> {
        NonNull::new(self.as_native_type_ref().display_meta_list)
            .map(|v| MetaList::<DisplayMeta>::new(v))
    }

    pub fn frame_user_meta_list(&self) -> Option<MetaList<UserMeta>> {
        NonNull::new(self.as_native_type_ref().frame_user_meta_list)
            .map(|v| MetaList::<UserMeta>::new(v))
    }

    pub fn misc_frame_info(&self) -> [i64; 4usize] {
        self.as_native_type_ref().misc_frame_info
    }

    pub fn pipeline_width(&self) -> u32 {
        self.as_native_type_ref().pipeline_width
    }

    pub fn pipeline_height(&self) -> u32 {
        self.as_native_type_ref().pipeline_height
    }

    pub fn add_obj_meta(&mut self, obj_meta: &ObjectMeta, parent_meta: Option<&ObjectMeta>) {
        unsafe {
            nvidia_deepstream_sys::nvds_add_obj_meta_to_frame(
                self.as_native_type_ref() as *const _ as _,
                obj_meta.as_native_type_ref() as *const _ as _,
                parent_meta.map_or(std::ptr::null_mut(), |p| {
                    p.as_native_type_ref() as *const _ as _
                }),
            );
        }
    }

    pub fn remove_obj_meta(&mut self, obj_meta: &ObjectMeta) {
        unsafe {
            nvidia_deepstream_sys::nvds_remove_obj_meta_from_frame(
                self.as_native_type_ref() as *const _ as _,
                obj_meta.as_native_type_ref() as *const _ as _,
            );
        }
    }

    pub fn add_display_meta(&self, meta: &DisplayMeta) {
        unsafe {
            nvidia_deepstream_sys::nvds_add_display_meta_to_frame(
                self.as_native_type_ref() as *const _ as _,
                meta.as_native_type_ref() as *const _ as _,
            );
        }
    }

    pub fn remove_display_meta(&self, meta: &DisplayMeta) {
        unsafe {
            nvidia_deepstream_sys::nvds_remove_display_meta_from_frame(
                self.as_native_type_ref() as *const _ as _,
                meta.as_native_type_ref() as *const _ as _,
            );
        }
    }

    pub fn add_user_meta(&self, meta: &UserMeta) {
        unsafe {
            nvidia_deepstream_sys::nvds_add_user_meta_to_frame(
                self.as_native_type_ref() as *const _ as _,
                meta.as_native_type_ref() as *const _ as _,
            );
        }
    }

    pub fn remove_user_meta(&self, meta: &UserMeta) {
        unsafe {
            nvidia_deepstream_sys::nvds_remove_user_meta_from_frame(
                self.as_native_type_ref() as *const _ as _,
                meta.as_native_type_ref() as *const _ as _,
            );
        }
    }

    pub fn clear_obj_meta_list(&mut self, meta_list: &MetaList<ObjectMeta>) {
        unsafe {
            nvidia_deepstream_sys::nvds_clear_obj_meta_list(
                self.as_native_type_mut() as _,
                meta_list.list.as_ptr(),
            );
        }
    }

    pub fn clear_display_meta_list(&mut self, meta_list: &MetaList<DisplayMeta>) {
        unsafe {
            nvidia_deepstream_sys::nvds_clear_display_meta_list(
                self.as_native_type_mut() as _,
                meta_list.list.as_ptr(),
            );
        }
    }

    pub fn clear_user_meta_list(&mut self, meta_list: &MetaList<UserMeta>) {
        unsafe {
            nvidia_deepstream_sys::nvds_clear_frame_user_meta_list(
                self.as_native_type_mut() as _,
                meta_list.list.as_ptr(),
            );
        }
    }

    pub fn copy_to(&self, dst_frame_meta: &mut FrameMeta) {
        unsafe {
            nvidia_deepstream_sys::nvds_copy_frame_meta(
                self.as_native_type_ref() as *const _ as _,
                dst_frame_meta.as_native_type_mut() as _,
            )
        }
    }
}

crate::wrapper_impl_ref_type!(ObjectMeta, nvidia_deepstream_sys::NvDsObjectMeta);

impl ObjectMeta {
    pub fn base_meta(&self) -> &BaseMeta {
        BaseMeta::from_native_type_ref(&self.as_native_type_ref().base_meta)
    }

    pub fn parent(&self) -> Option<&ObjectMeta> {
        NonNull::new(self.as_native_type_ref().parent)
            .map(|p| ObjectMeta::from_native_type_ref(unsafe { p.as_ref() }))
    }

    pub fn unique_component_id(&self) -> i32 {
        self.as_native_type_ref().unique_component_id
    }

    pub fn class_id(&self) -> i32 {
        self.as_native_type_ref().class_id
    }

    pub fn object_id(&self) -> u64 {
        self.as_native_type_ref().object_id
    }

    pub fn detector_bbox_info(&self) -> &crate::bounding_box::Info {
        crate::bounding_box::Info::from_native_type_ref(
            &self.as_native_type_ref().detector_bbox_info,
        )
    }

    pub fn tracker_bbox_info(&self) -> &crate::bounding_box::Info {
        crate::bounding_box::Info::from_native_type_ref(
            &self.as_native_type_ref().tracker_bbox_info,
        )
    }

    pub fn confidence(&self) -> f32 {
        self.as_native_type_ref().confidence
    }

    pub fn tracker_confidence(&self) -> f32 {
        self.as_native_type_ref().tracker_confidence
    }

    pub fn rect_params(&self) -> &osd::RectParams {
        osd::RectParams::from_native_type_ref(&self.as_native_type_ref().rect_params)
    }

    pub fn mask_params(&self) -> &osd::MaskParams {
        osd::MaskParams::from_native_type_ref(&self.as_native_type_ref().mask_params)
    }

    pub fn text_params(&self) -> &osd::TextParams {
        osd::TextParams::from_native_type_ref(&self.as_native_type_ref().text_params)
    }

    pub fn obj_label(&self) -> &GStr {
        unsafe { GStr::from_ptr(&self.as_native_type_ref().obj_label as _) }
    }

    pub fn classifier_meta_list(&self) -> Option<MetaList<ClassifierMeta>> {
        NonNull::new(self.as_native_type_ref().classifier_meta_list)
            .map(|v| MetaList::<ClassifierMeta>::new(v))
    }

    pub fn obj_user_meta_list(&self) -> Option<MetaList<UserMeta>> {
        NonNull::new(self.as_native_type_ref().obj_user_meta_list)
            .map(|v| MetaList::<UserMeta>::new(v))
    }

    pub fn misc_obj_info(&self) -> [i64; 4usize] {
        self.as_native_type_ref().misc_obj_info
    }

    pub fn add_classifier_meta(&self, meta: &ClassifierMeta) {
        unsafe {
            nvidia_deepstream_sys::nvds_add_classifier_meta_to_object(
                self.as_native_type_ref() as *const _ as _,
                meta.as_native_type_ref() as *const _ as _,
            );
        }
    }

    pub fn remove_classifier_meta(&self, meta: &ClassifierMeta) {
        unsafe {
            nvidia_deepstream_sys::nvds_remove_classifier_meta_from_obj(
                self.as_native_type_ref() as *const _ as _,
                meta.as_native_type_ref() as *const _ as _,
            );
        }
    }

    pub fn add_user_meta(&self, meta: &UserMeta) {
        unsafe {
            nvidia_deepstream_sys::nvds_add_user_meta_to_obj(
                self.as_native_type_ref() as *const _ as _,
                meta.as_native_type_ref() as *const _ as _,
            );
        }
    }

    pub fn remove_user_meta(&self, meta: &UserMeta) {
        unsafe {
            nvidia_deepstream_sys::nvds_remove_user_meta_from_object(
                self.as_native_type_ref() as *const _ as _,
                meta.as_native_type_ref() as *const _ as _,
            );
        }
    }

    pub fn clear_classifier_meta_list(&mut self, meta_list: &MetaList<ClassifierMeta>) {
        unsafe {
            nvidia_deepstream_sys::nvds_clear_classifier_meta_list(
                self.as_native_type_mut() as _,
                meta_list.list.as_ptr(),
            );
        }
    }

    pub fn clear_user_meta_list(&mut self, meta_list: &MetaList<UserMeta>) {
        unsafe {
            nvidia_deepstream_sys::nvds_clear_obj_user_meta_list(
                self.as_native_type_mut() as _,
                meta_list.list.as_ptr(),
            );
        }
    }

    pub fn copy_to(&self, dst_object_meta: &mut ObjectMeta) {
        unsafe {
            nvidia_deepstream_sys::nvds_copy_obj_meta(
                self.as_native_type_ref() as *const _ as _,
                dst_object_meta.as_native_type_mut() as _,
            )
        }
    }
}

crate::wrapper_impl_ref_type!(ClassifierMeta, nvidia_deepstream_sys::NvDsClassifierMeta);

impl ClassifierMeta {
    pub fn base_meta(&self) -> &BaseMeta {
        BaseMeta::from_native_type_ref(&self.as_native_type_ref().base_meta)
    }

    pub fn num_labels(&self) -> u32 {
        self.as_native_type_ref().num_labels
    }

    pub fn unique_component_id(&self) -> i32 {
        self.as_native_type_ref().unique_component_id
    }

    pub fn label_info_list(&self) -> Option<MetaList<LabelInfo>> {
        NonNull::new(self.as_native_type_ref().label_info_list)
            .map(|v| MetaList::<LabelInfo>::new(v))
    }

    pub fn classifier_type(&self) -> &GStr {
        unsafe { GStr::from_ptr(self.as_native_type_ref().classifier_type) }
    }

    pub fn add_label_info_meta(&self, meta: &LabelInfo) {
        unsafe {
            nvidia_deepstream_sys::nvds_add_label_info_meta_to_classifier(
                self.as_native_type_ref() as *const _ as _,
                meta.as_native_type_ref() as *const _ as _,
            );
        }
    }

    pub fn remove_label_info_meta(&self, meta: &LabelInfo) {
        unsafe {
            nvidia_deepstream_sys::nvds_remove_label_info_meta_from_classifier(
                self.as_native_type_ref() as *const _ as _,
                meta.as_native_type_ref() as *const _ as _,
            );
        }
    }

    pub fn clear_label_info_meta_list(&mut self, meta_list: &MetaList<LabelInfo>) {
        unsafe {
            nvidia_deepstream_sys::nvds_clear_label_info_meta_list(
                self.as_native_type_mut() as _,
                meta_list.list.as_ptr(),
            );
        }
    }

    pub fn copy_to(&self, dst_classifier_meta: &mut ClassifierMeta) {
        unsafe {
            nvidia_deepstream_sys::nvds_copy_classifier_meta(
                self.as_native_type_ref() as *const _ as _,
                dst_classifier_meta.as_native_type_mut() as _,
            )
        }
    }
}

crate::wrapper_impl_ref_type!(LabelInfo, nvidia_deepstream_sys::NvDsLabelInfo);

impl LabelInfo {
    pub fn base_meta(&self) -> &BaseMeta {
        BaseMeta::from_native_type_ref(&self.as_native_type_ref().base_meta)
    }

    pub fn num_classes(&self) -> u32 {
        self.as_native_type_ref().num_classes
    }

    pub fn result_label(&self) -> &GStr {
        unsafe {
            match NonNull::new(self.as_native_type_ref().pResult_label) {
                Some(p) => GStr::from_ptr(p.as_ptr()),
                None => GStr::from_ptr(&self.as_native_type_ref().result_label as _),
            }
        }
    }

    pub fn result_class_id(&self) -> u32 {
        self.as_native_type_ref().result_class_id
    }

    pub fn label_id(&self) -> u32 {
        self.as_native_type_ref().label_id
    }

    pub fn result_prob(&self) -> f32 {
        self.as_native_type_ref().result_prob
    }

    pub fn copy_to(&self, dst_label_info: &mut LabelInfo) {
        unsafe {
            nvidia_deepstream_sys::nvds_copy_label_info_meta(
                self.as_native_type_ref() as *const _ as _,
                dst_label_info.as_native_type_mut() as _,
            )
        }
    }
}

crate::wrapper_impl_ref_type!(DisplayMeta, nvidia_deepstream_sys::NvDsDisplayMeta);

impl DisplayMeta {
    pub fn base_meta(&self) -> &BaseMeta {
        BaseMeta::from_native_type_ref(&self.as_native_type_ref().base_meta)
    }

    pub fn num_rects(&self) -> u32 {
        self.as_native_type_ref().num_rects
    }

    pub fn num_labels(&self) -> u32 {
        self.as_native_type_ref().num_labels
    }

    pub fn num_lines(&self) -> u32 {
        self.as_native_type_ref().num_lines
    }

    pub fn num_arrows(&self) -> u32 {
        self.as_native_type_ref().num_arrows
    }

    pub fn num_circles(&self) -> u32 {
        self.as_native_type_ref().num_rects
    }

    pub fn rect_params(&self) -> &[osd::RectParams] {
        unsafe {
            std::slice::from_raw_parts(
                self.as_native_type_ref().rect_params.as_ptr() as _,
                self.as_native_type_ref().num_rects as usize,
            )
        }
    }

    pub fn text_params(&self) -> &[osd::TextParams] {
        unsafe {
            std::slice::from_raw_parts(
                self.as_native_type_ref().text_params.as_ptr() as _,
                self.as_native_type_ref().num_labels as usize,
            )
        }
    }

    pub fn line_params(&self) -> &[osd::LineParams] {
        unsafe {
            std::slice::from_raw_parts(
                self.as_native_type_ref().line_params.as_ptr() as _,
                self.as_native_type_ref().num_lines as usize,
            )
        }
    }

    pub fn arrow_params(&self) -> &[osd::ArrowParams] {
        unsafe {
            std::slice::from_raw_parts(
                self.as_native_type_ref().arrow_params.as_ptr() as _,
                self.as_native_type_ref().num_arrows as usize,
            )
        }
    }

    pub fn circle_params(&self) -> &[osd::CircleParams] {
        unsafe {
            std::slice::from_raw_parts(
                self.as_native_type_ref().circle_params.as_ptr() as _,
                self.as_native_type_ref().num_circles as usize,
            )
        }
    }

    pub fn misc_osd_data(&self) -> &[i64] {
        &self.as_native_type_ref().misc_osd_data
    }

    pub fn copy_to(&self, dst_display_meta: &mut DisplayMeta) {
        unsafe {
            nvidia_deepstream_sys::nvds_copy_display_meta(
                self.as_native_type_ref() as *const _ as _,
                dst_display_meta.as_native_type_mut() as _,
            )
        }
    }
}

pub struct DisplayMetaBuilder<'a> {
    rect_params: Option<&'a mut [osd::RectParamsBuilder]>,
    text_params: Option<&'a mut [osd::TextParamsBuilder]>,
    line_params: Option<&'a mut [osd::LineParamsBuilder]>,
    arrow_params: Option<&'a mut [osd::ArrowParamsBuilder]>,
    circle_params: Option<&'a mut [osd::CircleParamsBuilder]>,
    misc_osd_data: Option<&'a [i64]>,
}

impl<'a> DisplayMetaBuilder<'a> {
    pub fn new() -> DisplayMetaBuilder<'a> {
        DisplayMetaBuilder {
            rect_params: None,
            text_params: None,
            line_params: None,
            arrow_params: None,
            circle_params: None,
            misc_osd_data: None,
        }
    }

    pub fn rect_params(mut self, value: &'a mut [osd::RectParamsBuilder]) -> Self {
        self.rect_params = Some(value);
        self
    }

    pub fn text_params(mut self, value: &'a mut [osd::TextParamsBuilder]) -> Self {
        self.text_params = Some(value);
        self
    }

    pub fn line_params(mut self, value: &'a mut [osd::LineParamsBuilder]) -> Self {
        self.line_params = Some(value);
        self
    }

    pub fn arrow_params(mut self, value: &'a mut [osd::ArrowParamsBuilder]) -> Self {
        self.arrow_params = Some(value);
        self
    }

    pub fn circle_params(mut self, value: &'a mut [osd::CircleParamsBuilder]) -> Self {
        self.circle_params = Some(value);
        self
    }

    pub fn misc_osd_data(mut self, value: &'a [i64]) -> Self {
        self.misc_osd_data = Some(value);
        self
    }

    pub fn build<BM: BatchMetaExt>(
        self,
        display_meta_pool_batch_meta: &BM,
    ) -> Option<&DisplayMeta> {
        display_meta_pool_batch_meta
            .acquire_display_meta_from_pool()
            .map(|display_meta| {
                if let Some(params) = self.rect_params {
                    let len = std::cmp::min(
                        display_meta.as_native_type_ref().rect_params.len(),
                        params.len(),
                    );

                    display_meta.as_native_type_mut().num_rects = len as _;
                    for i in 0..len {
                        let mut builder = osd::RectParamsBuilder::new();
                        std::mem::swap(&mut params[i], &mut builder);
                        display_meta.as_native_type_mut().rect_params[i] =
                            *builder.build().as_native_type_ref();
                    }
                }

                if let Some(params) = self.text_params {
                    let len = std::cmp::min(
                        display_meta.as_native_type_ref().text_params.len(),
                        params.len(),
                    );

                    display_meta.as_native_type_mut().num_labels = len as _;
                    for i in 0..len {
                        let mut builder = osd::TextParamsBuilder::new();
                        std::mem::swap(&mut params[i], &mut builder);
                        osd::TextParams::from_native_type_mut(
                            &mut display_meta.as_native_type_mut().text_params[i],
                        )
                        .drop_ref();
                        display_meta.as_native_type_mut().text_params[i] =
                            builder.build().to_glib_full();
                    }
                }

                if let Some(params) = self.line_params {
                    let len = std::cmp::min(
                        display_meta.as_native_type_ref().line_params.len(),
                        params.len(),
                    );

                    display_meta.as_native_type_mut().num_lines = len as _;
                    for i in 0..len {
                        let mut builder = osd::LineParamsBuilder::new();
                        std::mem::swap(&mut params[i], &mut builder);
                        display_meta.as_native_type_mut().line_params[i] =
                            *builder.build().as_native_type_ref();
                    }
                }

                if let Some(params) = self.arrow_params {
                    let len = std::cmp::min(
                        display_meta.as_native_type_ref().arrow_params.len(),
                        params.len(),
                    );

                    display_meta.as_native_type_mut().num_arrows = len as _;
                    for i in 0..len {
                        let mut builder = osd::ArrowParamsBuilder::new();
                        std::mem::swap(&mut params[i], &mut builder);
                        display_meta.as_native_type_mut().arrow_params[i] =
                            *builder.build().as_native_type_ref();
                    }
                }

                if let Some(params) = self.circle_params {
                    let len = std::cmp::min(
                        display_meta.as_native_type_ref().circle_params.len(),
                        params.len(),
                    );

                    display_meta.as_native_type_mut().num_circles = len as _;
                    for i in 0..len {
                        let mut builder = osd::CircleParamsBuilder::new();
                        std::mem::swap(&mut params[i], &mut builder);
                        display_meta.as_native_type_mut().circle_params[i] =
                            *builder.build().as_native_type_ref();
                    }
                }

                if let Some(params) = self.misc_osd_data {
                    let len = std::cmp::min(
                        display_meta.as_native_type_ref().misc_osd_data.len(),
                        params.len(),
                    );

                    for i in 0..len {
                        display_meta.as_native_type_mut().misc_osd_data[i] = params[i];
                    }
                }

                display_meta as _
            })
    }
}

crate::wrapper_impl_ref_type!(UserMeta, nvidia_deepstream_sys::NvDsUserMeta);

impl UserMeta {
    pub fn get_user_meta_type(name: &GStr) -> MetaType {
        unsafe {
            MetaType::from(nvidia_deepstream_sys::nvds_get_user_meta_type(
                name.as_ptr() as _,
            ))
        }
    }

    pub fn user_custom_meta_type() -> MetaType {
        unsafe {
            Self::get_user_meta_type(GStr::from_ptr(b"NVIDIA.USER.CUSTOM_META\0".as_ptr() as _))
        }
    }

    pub fn dummy_bbox_meta_type() -> MetaType {
        unsafe {
            Self::get_user_meta_type(GStr::from_ptr(b"NVIDIA.DUMMY.BBOX.META\0".as_ptr() as _))
        }
    }

    pub fn base_meta(&self) -> &BaseMeta {
        BaseMeta::from_native_type_ref(&self.as_native_type_ref().base_meta)
    }

    pub unsafe fn user_meta_data<T: Clone>(&self) -> Option<&T> {
        NonNull::new(self.as_native_type_ref().user_meta_data as *mut T).map(|p| p.as_ref())
    }

    pub fn new<T: Clone, BM: BatchMetaExt>(
        user_meta_pool_batch_meta: &BM,
        meta_type: MetaType,
        meta_data: Box<T>,
    ) -> Option<&UserMeta> {
        user_meta_pool_batch_meta
            .acquire_user_meta_from_pool()
            .map(|user_meta| unsafe {
                user_meta.as_native_type_mut().base_meta.meta_type =
                    meta_type.to_native_meta_type();
                user_meta.as_native_type_mut().base_meta.copy_func =
                    Some(Self::base_meta_copy_func::<T>);
                user_meta.as_native_type_mut().base_meta.release_func =
                    Some(Self::base_meta_release_func::<T>);
                user_meta.as_native_type_mut().user_meta_data = Box::into_raw(meta_data) as _;
                std::mem::transmute(user_meta)
            })
    }

    extern "C" fn base_meta_copy_func<T: Clone>(
        data: nvidia_deepstream_sys::gpointer,
        _: nvidia_deepstream_sys::gpointer,
    ) -> nvidia_deepstream_sys::gpointer {
        unsafe {
            let user_meta = data as *mut nvidia_deepstream_sys::NvDsUserMeta;
            if user_meta == std::ptr::null_mut() {
                return std::ptr::null_mut();
            }
            let src_user_meta_data = (*user_meta).user_meta_data as *mut T;
            if src_user_meta_data == std::ptr::null_mut() {
                return std::ptr::null_mut();
            }

            let dst_user_meta_data = Box::<T>::new((*src_user_meta_data).clone());
            Box::into_raw(dst_user_meta_data) as _
        }
    }

    extern "C" fn base_meta_release_func<T: Clone>(
        data: nvidia_deepstream_sys::gpointer,
        _: nvidia_deepstream_sys::gpointer,
    ) {
        unsafe {
            let user_meta = data as *mut nvidia_deepstream_sys::NvDsUserMeta;
            if user_meta == std::ptr::null_mut() {
                return;
            }
            let user_meta_data = (*user_meta).user_meta_data;
            if user_meta_data == std::ptr::null_mut() {
                return;
            }
            let _ = Box::from_raw(user_meta_data as *mut T);
            (*user_meta).user_meta_data = std::ptr::null_mut();
        }
    }
}

crate::wrapper_impl_value_type!(FaceBoxes, nvidia_deepstream_sys::faceboxes);

impl FaceBoxes {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> FaceBoxes {
        FaceBoxes::from_native_type(nvidia_deepstream_sys::faceboxes {
            x,
            y,
            width,
            height,
        })
    }

    pub fn x(&self) -> f32 {
        self.as_native_type_ref().x
    }

    pub fn y(&self) -> f32 {
        self.as_native_type_ref().y
    }

    pub fn width(&self) -> f32 {
        self.as_native_type_ref().width
    }

    pub fn height(&self) -> f32 {
        self.as_native_type_ref().height
    }
}

impl MetaList<'_, FrameMeta> {
    pub fn get_nth_frame_meta(&self, index: u32) -> Option<&FrameMeta> {
        unsafe {
            NonNull::new(nvidia_deepstream_sys::nvds_get_nth_frame_meta(
                self.list.as_ptr(),
                index,
            ))
            .map(|mut p| FrameMeta::from_native_type_ref(p.as_mut()))
        }
    }

    pub fn copy_to_batch_meta(&self, dst_batch_meta: &mut BatchMeta) {
        unsafe {
            nvidia_deepstream_sys::nvds_copy_frame_meta_list(
                self.list.as_ptr(),
                dst_batch_meta.as_native_type_mut() as _,
            )
        }
    }
}

impl MetaList<'_, DisplayMeta> {
    pub fn copy_to_frame_meta(&self, dst_frame_meta: &mut FrameMeta) {
        unsafe {
            nvidia_deepstream_sys::nvds_copy_display_meta_list(
                self.list.as_ptr(),
                dst_frame_meta.as_native_type_mut() as _,
            )
        }
    }
}

impl MetaList<'_, ObjectMeta> {
    pub fn copy_to_frame_meta(&self, dst_frame_meta: &mut FrameMeta) {
        unsafe {
            nvidia_deepstream_sys::nvds_copy_obj_meta_list(
                self.list.as_ptr(),
                dst_frame_meta.as_native_type_mut() as _,
            )
        }
    }
}

impl MetaList<'_, ClassifierMeta> {
    pub fn copy_to_obj_meta(&self, dst_object_meta: &mut ObjectMeta) {
        unsafe {
            nvidia_deepstream_sys::nvds_copy_classification_list(
                self.list.as_ptr(),
                dst_object_meta.as_native_type_mut() as _,
            )
        }
    }

    pub fn copy_to_audio_frame_meta(&self, dst_frame_meta: &mut audio::AudioFrameMeta) {
        unsafe {
            nvidia_deepstream_sys::nvds_copy_audio_classification_list(
                self.list.as_ptr(),
                dst_frame_meta.as_native_type_mut() as _,
            )
        }
    }
}

impl MetaList<'_, LabelInfo> {
    pub fn copy_to_obj_meta(&self, dst_classifier_meta: &mut ClassifierMeta) {
        unsafe {
            nvidia_deepstream_sys::nvds_copy_label_info_list(
                self.list.as_ptr(),
                dst_classifier_meta.as_native_type_mut() as _,
            )
        }
    }
}

impl MetaList<'_, UserMeta> {
    pub fn copy_to_batch_meta(&self, dst_batch_meta: &mut BatchMeta) {
        unsafe {
            nvidia_deepstream_sys::nvds_copy_batch_user_meta_list(
                self.list.as_ptr(),
                dst_batch_meta.as_native_type_mut() as _,
            )
        }
    }

    pub fn copy_to_frame_meta(&self, dst_frame_meta: &mut FrameMeta) {
        unsafe {
            nvidia_deepstream_sys::nvds_copy_frame_user_meta_list(
                self.list.as_ptr(),
                dst_frame_meta.as_native_type_mut() as _,
            )
        }
    }

    pub fn copy_to_obj_meta(&self, dst_object_meta: &mut ObjectMeta) {
        unsafe {
            nvidia_deepstream_sys::nvds_copy_obj_user_meta_list(
                self.list.as_ptr(),
                dst_object_meta.as_native_type_mut() as _,
            )
        }
    }

    pub fn copy_to_audio_batch_meta(&self, dst_batch_meta: &mut audio::AudioBatchMeta) {
        unsafe {
            nvidia_deepstream_sys::nvds_copy_audio_batch_user_meta_list(
                self.list.as_ptr(),
                dst_batch_meta.as_native_type_mut() as _,
            )
        }
    }
}

pub trait BufferExt: 'static {
    fn get_nvds_batch_meta(&self) -> Option<&mut BatchMeta>;
}

impl BufferExt for gstreamer::Buffer {
    fn get_nvds_batch_meta(&self) -> Option<&mut BatchMeta> {
        unsafe {
            let batch_meta = nvidia_deepstream_sys::gst_buffer_get_nvds_batch_meta(
                self.as_mut_ptr() as *mut nvidia_deepstream_sys::GstBuffer,
            );

            if batch_meta != std::ptr::null_mut() {
                Some(BatchMeta::from_native_type_mut(&mut *batch_meta))
            } else {
                None
            }
        }
    }
}

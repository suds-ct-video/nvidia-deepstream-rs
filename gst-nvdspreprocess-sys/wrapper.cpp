//  meta
struct NvDsRoiMeta;
#include <nvds_roi_meta.h>
#include <nvdsmeta.h>
/* #include <nvdspreprocess_interface.h> */ // Needs CUDA
#include <nvdspreprocess_meta.h>

const char *get_cstr(std::string &s) { return s.c_str(); }

const int *get_tensor_shape_ptr(std::vector<int> &ts) { return ts.data(); }
// HACK: is it necessary to have size for all templated vecs?
size_t get_tensor_shape_size(std::vector<int> &ts) { return ts.size(); }

const NvDsRoiMeta *get_roi_vec_ptr(std::vector<NvDsRoiMeta> &rs) {
  return rs.data();
}
size_t get_roi_vec_size(std::vector<NvDsRoiMeta> &rs) { return rs.size(); }

const guint64 *get_target_unique_ids_ptr(std::vector<guint64> &rs) {
  return rs.data();
}
size_t get_target_unique_ids_size(std::vector<guint64> &rs) {
  return rs.size();
}

NvDsPreProcessTensorMeta
copy_nvds_preprocess_tensor_meta(NvDsPreProcessTensorMeta &meta) {
  return NvDsPreProcessTensorMeta(meta);
}

GstNvDsPreProcessBatchMeta
copy_gst_nvds_preprocess_batch_meta(GstNvDsPreProcessBatchMeta &meta) {
  return GstNvDsPreProcessBatchMeta(meta);
}

/* const char *get_(std::string s) { return s.c_str(); } */

/* int get_rois(std::vector<NvDsRoiMeta> rois) {} */

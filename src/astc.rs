use crate::RgbaSurface;
use crate::bindings::kernel_astc;

/*

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct astc_enc_context {
    pub width: i32,
    pub height: i32,
    pub channels: i32,
    pub dual_plane: bool,
    pub partitions: i32,
    pub color_endpoint_pairs: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct astc_enc_settings {
    pub block_width: i32,
    pub block_height: i32,
    pub channels: i32,
    pub fastSkipTreshold: i32,
    pub refineIterations: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct astc_block {
    pub width: i32,
    pub height: i32,
    pub dual_plane: bool,
    pub weight_range: i32,
    pub weights: [u8; 64usize],
    pub color_component_selector: i32,
    pub partitions: i32,
    pub partition_id: i32,
    pub color_endpoint_pairs: i32,
    pub channels: i32,
    pub color_endpoint_modes: [i32; 4usize],
    pub endpoint_range: i32,
    pub endpoints: [u8; 18usize],
}

extern "C" {
    #[link_name = "\u{1}_astc_encode_ispc"]
    pub fn astc_encode_ispc(
        src: *mut rgba_surface,
        block_scores: *mut f32,
        dst: *mut u8,
        list: *mut u64,
        list_context: *mut astc_enc_context,
        settings: *mut astc_enc_settings,
    );
}
extern "C" {
    #[link_name = "\u{1}_astc_rank_ispc"]
    pub fn astc_rank_ispc(
        src: *mut rgba_surface,
        xx: i32,
        yy: i32,
        mode_buffer: *mut u32,
        settings: *mut astc_enc_settings,
    );
}
extern "C" {
    #[link_name = "\u{1}_get_programCount"]
    pub fn get_programCount() -> i32;
}

*/
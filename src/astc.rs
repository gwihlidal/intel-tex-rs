#![allow(dead_code)]
#![allow(unused_variables)]

use crate::bindings::kernel_astc;
use crate::RgbaSurface;

#[derive(Debug, Copy, Clone)]
pub struct EncodeSettings {
    pub block_width: u32,
    pub block_height: u32,
    pub channels: u32,
    pub fast_skip_threshold: u32,
    pub refine_iterations: u32,
}

/*
#[derive(Debug, Copy, Clone)]
pub struct EncodeContext {
    pub width: u32,
    pub height: u32,
    pub channels: u32,
    pub dual_plane: bool,
    pub partitions: u32,
    pub color_endpoint_pairs: u32,
}

#[derive(Copy, Clone)]
pub struct Block {
    pub width: u32,
    pub height: u32,
    pub dual_plane: bool,
    pub weight_range: u32,
    pub weights: [u8; 64usize],
    pub color_component_selector: u32,
    pub partitions: u32,
    pub partition_id: u32,
    pub color_endpoint_pairs: u32,
    pub channels: u32,
    pub color_endpoint_modes: [u32; 4usize],
    pub endpoint_range: u32,
    pub endpoints: [u8; 18usize],
}
*/

#[inline(always)]
pub fn opaque_fast_settings(block_width: u32, block_height: u32) -> EncodeSettings {
    EncodeSettings {
        block_width,
        block_height,
        channels: 3,
        fast_skip_threshold: 5,
        refine_iterations: 2,
    }
}

#[inline(always)]
pub fn alpha_fast_settings(block_width: u32, block_height: u32) -> EncodeSettings {
    EncodeSettings {
        block_width,
        block_height,
        channels: 4,
        fast_skip_threshold: 5,
        refine_iterations: 2,
    }
}

#[inline(always)]
pub fn alpha_slow_settings(block_width: u32, block_height: u32) -> EncodeSettings {
    EncodeSettings {
        block_width,
        block_height,
        channels: 4,
        fast_skip_threshold: 64,
        refine_iterations: 2,
    }
}

fn can_store(value: i32, bits: i32) -> bool {
    if value < 0 {
        return false;
    }

    if value >= 1 << bits {
        return false;
    }

    true
}

fn astc_encode(
    settings: &mut kernel_astc::astc_enc_settings,
    surface: &mut kernel_astc::rgba_surface,
    block_scores: &mut [f32],
    blocks: &mut [u8],
    list: &mut [u64],
) {
    unimplemented!();
    /*
    ispc::astc_enc_context list_context;
    setup_list_context(&list_context, uint32_t(list[1] & 0xFFFFFFFF));

    assert(sizeof(ispc::rgba_surface) == sizeof(rgba_surface));
    assert(sizeof(ispc::astc_enc_settings) == sizeof(astc_enc_settings));
    ispc::astc_encode_ispc((ispc::rgba_surface*)src, block_scores, dst, list, &list_context, (ispc::astc_enc_settings*)settings);
    */
}

fn astc_rank(
    settings: &mut kernel_astc::astc_enc_settings,
    surface: &mut kernel_astc::rgba_surface,
    xx: u32,
    yy: u32,
    mode_buffer: &mut [u32],
) {
    unsafe {
        kernel_astc::astc_rank_ispc(
            surface,
            xx as i32,
            yy as i32,
            mode_buffer.as_mut_ptr(),
            settings,
        );
    }
}

#[inline(always)]
pub fn calc_output_size(width: u32, height: u32) -> usize {
    // ASTC uses a fixed block size of 16 bytes (128 bits).
    let block_count = crate::divide_up_by_multiple(width * height, 16);
    block_count as usize * 16
}

pub fn compress_blocks(settings: &EncodeSettings, surface: &RgbaSurface) -> Vec<u8> {
    let output_size = calc_output_size(surface.width, surface.height);
    let mut output = vec![0u8; output_size];
    compress_blocks_into(settings, surface, &mut output);
    output
}

pub fn compress_blocks_into(settings: &EncodeSettings, surface: &RgbaSurface, blocks: &mut [u8]) {
    assert_eq!(surface.height % settings.block_height, 0);
    assert_eq!(surface.width % settings.block_width, 0);
    assert!(settings.block_height <= 8);
    assert!(settings.block_width <= 8);
    assert_eq!(
        blocks.len(),
        calc_output_size(surface.width, surface.height)
    );

    let tex_width = surface.width / settings.block_width;
    let program_count = unsafe { kernel_astc::get_programCount() as u32 };

    let mut block_scores =
        vec![std::f32::INFINITY; (tex_width * surface.height / settings.block_height) as usize];

    let mode_list_size = 3334usize;
    let list_size = program_count as usize;

    let mut mode_lists: Vec<u64> = vec![0; (list_size * mode_list_size) as usize];
    let mut mode_buffer: Vec<u32> =
        vec![0; (program_count * settings.fast_skip_threshold) as usize];

    let mut surface = kernel_astc::rgba_surface {
        width: surface.width as i32,
        height: surface.height as i32,
        stride: surface.stride as i32,
        ptr: surface.data.as_ptr() as *mut u8,
    };

    let mut settings = kernel_astc::astc_enc_settings {
        block_width: settings.block_width as i32,
        block_height: settings.block_height as i32,
        channels: settings.channels as i32,
        fastSkipTreshold: settings.fast_skip_threshold as i32,
        refineIterations: settings.refine_iterations as i32,
    };

    for yy in 0..(surface.height / settings.block_height) as u32 {
        for xx in 0..((tex_width + program_count - 1) / program_count) as u32 {
            let xx = xx * program_count;
            astc_rank(&mut settings, &mut surface, xx, yy, &mut mode_buffer);
            for i in 0..settings.fastSkipTreshold as u32 {
                for k in 0..program_count {
                    if xx + k >= tex_width {
                        continue;
                    }
                    let offset = (yy << 16) + (xx + k);
                    let mode = u64::from(mode_buffer[(program_count * i + k) as usize]);
                    let mode_bin = mode >> 20;
                    let list_start = list_size * mode_bin as usize;
                    if mode_lists[list_start] < u64::from(program_count - 1) {
                        let index = (mode_lists[list_start] + 1) as usize;
                        mode_lists[list_start] = index as u64;
                        mode_lists[list_start + index] = (u64::from(offset) << 32) + mode;
                    } else {
                        mode_lists[list_start] = (u64::from(offset) << 32) + mode;
                        astc_encode(
                            &mut settings,
                            &mut surface,
                            &mut block_scores,
                            blocks,
                            &mut mode_lists[list_start..(list_start + list_size)],
                        );
                        for mode_list in mode_lists.iter_mut().skip(list_start).take(list_size) {
                            *mode_list = 0;
                        }
                    }
                }
            }
        }
    }

    for mode_bin in 0usize..mode_list_size {
        let list_start = list_size * mode_bin;
        if mode_lists[list_start] == 0 {
            continue;
        }
        mode_lists[list_start] = 0;
        astc_encode(
            &mut settings,
            &mut surface,
            &mut block_scores,
            blocks,
            &mut mode_lists[list_start..(list_start + list_size)],
        );
        for mode_list in mode_lists.iter_mut().skip(list_start).take(list_size) {
            *mode_list = 0;
        }
    }
}

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
*/

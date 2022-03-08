use crate::bindings::kernel;
use crate::RgbaSurface;

#[derive(Debug, Copy, Clone)]
pub struct EncodeSettings {
    pub slow_mode: bool,
    pub fast_mode: bool,
    pub refine_iterations_1p: u32,
    pub refine_iterations_2p: u32,
    pub fast_skip_threshold: u32,
}

#[inline(always)]
pub fn calc_output_size(width: u32, height: u32) -> usize {
    // BC6H uses a fixed block size of 16 bytes (128 bits) and a fixed tile size of 4x4 texels.
    let block_count = crate::divide_up_by_multiple(width * height, 16) as usize;
    block_count * 16
}

pub fn compress_blocks(settings: &EncodeSettings, surface: &RgbaSurface) -> Vec<u8> {
    let output_size = calc_output_size(surface.width, surface.height);
    let mut output = vec![0u8; output_size];
    compress_blocks_into(settings, surface, &mut output);
    output
}

pub fn compress_blocks_into(settings: &EncodeSettings, surface: &RgbaSurface, blocks: &mut [u8]) {
    assert_eq!(
        blocks.len(),
        calc_output_size(surface.width, surface.height)
    );
    let mut surface = kernel::rgba_surface {
        width: surface.width as i32,
        height: surface.height as i32,
        stride: surface.stride as i32,
        ptr: surface.data.as_ptr() as *mut u8,
    };
    let mut settings = kernel::bc6h_enc_settings {
        slow_mode: settings.slow_mode,
        fast_mode: settings.fast_mode,
        refineIterations_1p: settings.refine_iterations_1p as i32,
        refineIterations_2p: settings.refine_iterations_2p as i32,
        fastSkipTreshold: settings.fast_skip_threshold as i32,
    };

    unsafe {
        kernel::CompressBlocksBC6H_ispc(&mut surface, blocks.as_mut_ptr(), &mut settings);
    }
}

#[inline(always)]
pub fn very_fast_settings() -> EncodeSettings {
    EncodeSettings {
        slow_mode: false,
        fast_mode: true,
        fast_skip_threshold: 0,
        refine_iterations_1p: 0,
        refine_iterations_2p: 0,
    }
}

#[inline(always)]
pub fn fast_settings() -> EncodeSettings {
    EncodeSettings {
        slow_mode: false,
        fast_mode: true,
        fast_skip_threshold: 2,
        refine_iterations_1p: 0,
        refine_iterations_2p: 1,
    }
}

#[inline(always)]
pub fn basic_settings() -> EncodeSettings {
    EncodeSettings {
        slow_mode: false,
        fast_mode: false,
        fast_skip_threshold: 4,
        refine_iterations_1p: 2,
        refine_iterations_2p: 2,
    }
}

#[inline(always)]
pub fn slow_settings() -> EncodeSettings {
    EncodeSettings {
        slow_mode: true,
        fast_mode: false,
        fast_skip_threshold: 10,
        refine_iterations_1p: 2,
        refine_iterations_2p: 2,
    }
}

#[inline(always)]
pub fn very_slow_settings() -> EncodeSettings {
    EncodeSettings {
        slow_mode: true,
        fast_mode: false,
        fast_skip_threshold: 32,
        refine_iterations_1p: 2,
        refine_iterations_2p: 2,
    }
}

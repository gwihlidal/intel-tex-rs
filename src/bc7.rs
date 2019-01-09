use crate::bindings::kernel;
use crate::RgbaSurface;

#[derive(Debug, Copy, Clone)]
pub struct EncodeSettings {
    pub mode_selection: [bool; 4usize],
    pub refine_iterations: [u32; 8usize],
    pub skip_mode2: bool,
    pub fast_skip_threshold_mode1: u32,
    pub fast_skip_threshold_mode3: u32,
    pub fast_skip_threshold_mode7: u32,
    pub mode45_channel0: u32,
    pub refine_iterations_channel: u32,
    pub channels: i32,
}

#[inline(always)]
pub fn calc_output_size(width: u32, height: u32) -> usize {
    // BC7 uses a fixed block size of 16 bytes (128 bits) and a fixed tile size of 4x4 texels.
    let block_count = crate::divide_up_by_multiple(width * height, 8);
    block_count as usize * 16
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
    let mut settings = kernel::bc7_enc_settings {
        mode_selection: settings.mode_selection,
        refineIterations: [
            settings.refine_iterations[0] as i32,
            settings.refine_iterations[1] as i32,
            settings.refine_iterations[2] as i32,
            settings.refine_iterations[3] as i32,
            settings.refine_iterations[4] as i32,
            settings.refine_iterations[5] as i32,
            settings.refine_iterations[6] as i32,
            settings.refine_iterations[7] as i32,
        ],
        skip_mode2: settings.skip_mode2,
        fastSkipTreshold_mode1: settings.fast_skip_threshold_mode1 as i32,
        fastSkipTreshold_mode3: settings.fast_skip_threshold_mode3 as i32,
        fastSkipTreshold_mode7: settings.fast_skip_threshold_mode7 as i32,
        mode45_channel0: settings.mode45_channel0 as i32,
        refineIterations_channel: settings.refine_iterations_channel as i32,
        channels: settings.channels as i32,
    };

    unsafe {
        kernel::CompressBlocksBC7_ispc(&mut surface, blocks.as_mut_ptr(), &mut settings);
    }
}

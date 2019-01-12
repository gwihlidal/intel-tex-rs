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

#[inline(always)]
pub fn opaque_ultra_fast_settings() -> EncodeSettings {
    EncodeSettings {
        channels: 3,
        mode_selection: [false, false, false, true],
        fast_skip_threshold_mode1: 3,
        fast_skip_threshold_mode3: 1,
        fast_skip_threshold_mode7: 0,
        skip_mode2: true,
        mode45_channel0: 0,
        refine_iterations_channel: 0,
        refine_iterations: [2, 2, 2, 1, 2, 2, 1, 0],
    }
}

#[inline(always)]
pub fn opaque_very_fast_settings() -> EncodeSettings {
    EncodeSettings {
        channels: 3,
        mode_selection: [false, true, false, true],
        fast_skip_threshold_mode1: 3,
        fast_skip_threshold_mode3: 1,
        fast_skip_threshold_mode7: 0,
        skip_mode2: true,
        mode45_channel0: 0,
        refine_iterations_channel: 0,
        refine_iterations: [2, 2, 2, 1, 2, 2, 1, 0],
    }
}

#[inline(always)]
pub fn opaque_fast_settings() -> EncodeSettings {
    EncodeSettings {
        channels: 3,
        mode_selection: [false, true, false, true],
        fast_skip_threshold_mode1: 12,
        fast_skip_threshold_mode3: 4,
        fast_skip_threshold_mode7: 0,
        skip_mode2: true,
        mode45_channel0: 0,
        refine_iterations_channel: 0,
        refine_iterations: [2, 2, 2, 1, 2, 2, 2, 0],
    }
}

#[inline(always)]
pub fn opaque_basic_settings() -> EncodeSettings {
    EncodeSettings {
        channels: 3,
        mode_selection: [true, true, true, true],
        fast_skip_threshold_mode1: 8 + 4,
        fast_skip_threshold_mode3: 8,
        fast_skip_threshold_mode7: 0,
        skip_mode2: true,
        mode45_channel0: 0,
        refine_iterations_channel: 2,
        refine_iterations: [2, 2, 2, 2, 2, 2, 2, 0],
    }
}

#[inline(always)]
pub fn opaque_slow_settings() -> EncodeSettings {
    let more_refinement = 2;
    EncodeSettings {
        channels: 3,
        mode_selection: [true, true, true, true],
        fast_skip_threshold_mode1: 64,
        fast_skip_threshold_mode3: 64,
        fast_skip_threshold_mode7: 0,
        skip_mode2: false,
        mode45_channel0: 0,
        refine_iterations_channel: 2 + more_refinement,
        refine_iterations: [
            2 + more_refinement,
            2 + more_refinement,
            2 + more_refinement,
            2 + more_refinement,
            2 + more_refinement,
            2 + more_refinement,
            2 + more_refinement,
            0,
        ],
    }
}

#[inline(always)]
pub fn alpha_ultra_fast_settings() -> EncodeSettings {
    EncodeSettings {
        channels: 4,
        mode_selection: [false, false, true, true],
        fast_skip_threshold_mode1: 0,
        fast_skip_threshold_mode3: 0,
        fast_skip_threshold_mode7: 4,
        skip_mode2: true,
        mode45_channel0: 3,
        refine_iterations_channel: 1,
        refine_iterations: [2, 1, 2, 1, 1, 1, 2, 2],
    }
}

#[inline(always)]
pub fn alpha_very_fast_settings() -> EncodeSettings {
    EncodeSettings {
        channels: 4,
        mode_selection: [false, true, true, true],
        fast_skip_threshold_mode1: 0,
        fast_skip_threshold_mode3: 0,
        fast_skip_threshold_mode7: 4,
        skip_mode2: true,
        mode45_channel0: 3,
        refine_iterations_channel: 2,
        refine_iterations: [2, 1, 2, 1, 2, 2, 2, 2],
    }
}

#[inline(always)]
pub fn alpha_fast_settings() -> EncodeSettings {
    EncodeSettings {
        channels: 4,
        mode_selection: [false, true, true, true],
        fast_skip_threshold_mode1: 4,
        fast_skip_threshold_mode3: 4,
        fast_skip_threshold_mode7: 8,
        skip_mode2: true,
        mode45_channel0: 3,
        refine_iterations_channel: 2,
        refine_iterations: [2, 1, 2, 1, 2, 2, 2, 2],
    }
}

#[inline(always)]
pub fn alpha_basic_settings() -> EncodeSettings {
    EncodeSettings {
        channels: 4,
        mode_selection: [true, true, true, true],
        fast_skip_threshold_mode1: 8 + 4,
        fast_skip_threshold_mode3: 8,
        fast_skip_threshold_mode7: 4,
        skip_mode2: true,
        mode45_channel0: 0,
        refine_iterations_channel: 2,
        refine_iterations: [2, 2, 2, 2, 2, 2, 2, 2],
    }
}

#[inline(always)]
pub fn alpha_slow_settings() -> EncodeSettings {
    let more_refinement = 2;
    EncodeSettings {
        channels: 4,
        mode_selection: [true, true, true, true],
        fast_skip_threshold_mode1: 64,
        fast_skip_threshold_mode3: 64,
        fast_skip_threshold_mode7: 64,
        skip_mode2: false,
        mode45_channel0: 0,
        refine_iterations_channel: 2 + more_refinement,
        refine_iterations: [
            2 + more_refinement,
            2 + more_refinement,
            2 + more_refinement,
            2 + more_refinement,
            2 + more_refinement,
            2 + more_refinement,
            2 + more_refinement,
            2 + more_refinement,
        ],
    }
}

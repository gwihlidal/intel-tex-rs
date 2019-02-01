use crate::bindings::kernel;
use crate::RgbaSurface;

#[derive(Debug, Copy, Clone)]
pub struct EncodeSettings {
    pub fast_skip_threshold: u32,
}

#[inline(always)]
pub fn calc_output_size(width: u32, height: u32) -> usize {
    // ETC1 uses a fixed block size of 8 bytes (64 bits) and a fixed tile size of 4x4 texels.
    let block_count = crate::divide_up_by_multiple(width * height, 16);
    block_count as usize * 8
}

pub fn compress_blocks(settings: EncodeSettings, surface: &RgbaSurface) -> Vec<u8> {
    let output_size = calc_output_size(surface.width, surface.height);
    let mut output = vec![0u8; output_size];
    compress_blocks_into(settings, surface, &mut output);
    output
}

pub fn compress_blocks_into(settings: EncodeSettings, surface: &RgbaSurface, blocks: &mut [u8]) {
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
    let mut settings = kernel::etc_enc_settings {
        fastSkipTreshold: settings.fast_skip_threshold as i32,
    };

    unsafe {
        kernel::CompressBlocksETC1_ispc(&mut surface, blocks.as_mut_ptr(), &mut settings);
    }
}

#[inline(always)]
pub fn slow_settings() -> EncodeSettings {
    EncodeSettings {
        fast_skip_threshold: 6,
    }
}

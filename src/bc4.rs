use crate::bindings::kernel;
use crate::RgbaSurface;

#[inline(always)]
pub fn calc_output_size(width: u32, height: u32) -> usize {
    // BC4 uses 8 bytes to store each 4×4 block, giving it an average data rate of 1 byte per pixel.
    let block_count = crate::divide_up_by_multiple(width * height, 16) as usize;
    block_count * 8
}

pub fn compress_blocks(surface: &RgbaSurface) -> Vec<u8> {
    let output_size = calc_output_size(surface.width, surface.height);
    let mut output = vec![0u8; output_size];
    compress_blocks_into(surface, &mut output);
    output
}

pub fn compress_blocks_into(surface: &RgbaSurface, blocks: &mut [u8]) {
    assert_eq!(
        blocks.len(),
        calc_output_size(surface.width, surface.height)
    );

    let mut r_data = vec![0_u8; (surface.width * surface.height) as usize];
    let pitch = (surface.width * 32 + 7) / 8;
    let mut offset = 0_u32;

    for y in 0..surface.height {
        for x in 0..surface.width {
            // Copy R byte over
            r_data[offset as usize] = surface.data[(x * 4 + y * pitch) as usize];
            offset += 1;
        }
    }

    let mut surface = kernel::rgba_surface {
        width: surface.width as i32,
        height: surface.height as i32,
        stride: (surface.stride / 4) as i32,
        ptr: (&r_data).as_ptr() as *mut u8,
    };

    unsafe {
        kernel::CompressBlocksBC4_ispc(&mut surface, blocks.as_mut_ptr());
    }
}

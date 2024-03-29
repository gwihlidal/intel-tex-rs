use crate::bindings::kernel;
use crate::RgbaSurface;

#[inline(always)]
pub fn calc_output_size(width: u32, height: u32) -> usize {
    // BC5 uses 16 bytes to store each 4×4 block, giving it an average data rate of 1 byte per pixel.
    let block_count = crate::divide_up_by_multiple(width * height, 16) as usize;
    block_count * 16
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

    let mut rg_data = vec![0_u8; (surface.width * surface.height * 2) as usize];
    let pitch = (surface.width * 32 + 7) / 8;
    let mut offset = 0_u32;

    for y in 0..surface.height {
        for x in 0..surface.width {
            // Copy R and G bytes over
            rg_data[offset as usize] = surface.data[(x * 4 + y * pitch) as usize];
            rg_data[(offset + 1) as usize] = surface.data[(x * 4 + y * pitch + 1) as usize];
            offset += 2;
        }
    }

    let mut surface = kernel::rgba_surface {
        width: surface.width as i32,
        height: surface.height as i32,
        stride: (surface.stride / 2) as i32,
        ptr: (&rg_data).as_ptr() as *mut u8,
    };

    unsafe {
        kernel::CompressBlocksBC5_ispc(&mut surface, blocks.as_mut_ptr());
    }
}

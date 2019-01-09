extern crate image;
extern crate ddsfile;
extern crate intel_tex;

use std::fs::File;
use std::path::Path;
use image::Pixel;
use image::ImageBuffer;
use image::GenericImageView;
use intel_tex::bindings::kernel;

use ddsfile::{
    Dds,
    DxgiFormat,
    D3D10ResourceDimension,
    AlphaMode,
    Caps2
};

fn main() {
    let rgb_img = image::open(&Path::new("examples/lambertian.jpg")).unwrap();

    let (width, height) = rgb_img.dimensions();
    println!("Width is {}", width);
    println!("Height is {}", height);
    println!("ColorType is {:?}", rgb_img.color());

    let mut rgba_img = ImageBuffer::new(width, height);

    // Convert RGB -> RGBA
    for x in (0_u32..width).into_iter() {
        for y in (0_u32..height).into_iter() {
            let pixel = rgb_img.get_pixel(x, y);
            let pixel_rgba = pixel.to_rgba();
            rgba_img.put_pixel(x, y, pixel_rgba);
        }
    }

    let block_count = divide_up_by_multiple(width * height, 8);
    println!("Block count: {}", block_count);

    let mip_count = 1;
    let array_layers = 1;
    let caps2 = Caps2::empty();
    let is_cubemap = false;
    let resource_dimension = D3D10ResourceDimension::Texture2D;
    let alpha_mode = AlphaMode::Opaque;
    let depth = 1;

    let mut dds = Dds::new_dxgi(
        height,
        width,
        Some(depth),
        DxgiFormat::BC7_UNorm,
        Some(mip_count),
        Some(array_layers),
        Some(caps2),
        is_cubemap,
        resource_dimension,
        alpha_mode).unwrap();

    let mut surface = kernel::rgba_surface {
        width: width as i32,
        height: height as i32,
        stride: width as i32 * 4,
        ptr: rgba_img.as_mut_ptr(),
    };

    let more_refine: i32 = 2;
    let mut bc7_settings_slow = kernel::bc7_enc_settings {
        channels: 3,
        mode_selection: [true, true, true, true], // mode02, mode13, mode45, mode6
        refineIterations: [more_refine + 2, more_refine + 2, more_refine + 2, more_refine + 2, more_refine + 2, more_refine + 2, more_refine + 2, 0],
        skip_mode2: false,
        refineIterations_channel: more_refine + 2,
        mode45_channel0: 0,
        fastSkipTreshold_mode1: 64,
        fastSkipTreshold_mode3: 64,
        fastSkipTreshold_mode7: 0,
    };

    let _bc6_settings = kernel::bc6h_enc_settings {
        slow_mode: false,
        fast_mode: false,
        refineIterations_1p: 3,
        refineIterations_2p: 3,
        fastSkipTreshold: 4,
    };

    unsafe {
        kernel::CompressBlocksBC7_ispc(&mut surface, dds.get_mut_data(0 /* layer */).unwrap().as_mut_ptr(), &mut bc7_settings_slow);
    }

    let mut dds_file = File::create("examples/lambertian.dds").unwrap();
    dds.write(&mut dds_file).expect("Failed to write dds file");
}
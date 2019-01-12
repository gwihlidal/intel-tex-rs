extern crate ddsfile;
extern crate image;
extern crate intel_tex;

use image::GenericImageView;
use image::ImageBuffer;
use image::Pixel;
use intel_tex::bc7;
use std::fs::File;
use std::path::Path;

use ddsfile::{AlphaMode, Caps2, D3D10ResourceDimension, Dds, DxgiFormat};

fn main() {
    let rgb_img = image::open(&Path::new("examples/lambertian.jpg")).unwrap();

    let (width, height) = rgb_img.dimensions();
    println!("Width is {}", width);
    println!("Height is {}", height);
    println!("ColorType is {:?}", rgb_img.color());

    let mut rgba_img = ImageBuffer::new(width, height);

    println!("Converting RGB -> RGBA"); // could be optimized
    for x in (0_u32..width).into_iter() {
        for y in (0_u32..height).into_iter() {
            let pixel = rgb_img.get_pixel(x, y);
            let pixel_rgba = pixel.to_rgba();
            rgba_img.put_pixel(x, y, pixel_rgba);
        }
    }

    let block_count = intel_tex::divide_up_by_multiple(width * height, 16);
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
        alpha_mode,
    )
    .unwrap();

    let surface = intel_tex::RgbaSurface {
        width,
        height,
        stride: width * 4,
        data: &rgba_img,
    };

    println!("Compressing to BC7...");
    bc7::compress_blocks_into(
        &bc7::opaque_ultra_fast_settings(),
        &surface,
        &mut dds.get_mut_data(0 /* layer */).unwrap(),
    );
    println!("  Done!");

    println!("Saving lambertian.dds file");
    let mut dds_file = File::create("examples/lambertian.dds").unwrap();
    dds.write(&mut dds_file).expect("Failed to write dds file");
}

#[macro_use]
extern crate ispc_rt;

pub mod bindings {
    ispc_module!(kernel);
    ispc_module!(kernel_astc);
}

pub mod astc;
pub mod bc1;
pub mod bc3;
pub mod bc6h;
pub mod bc7;
pub mod etc1;

#[derive(Debug, Copy, Clone)]
pub struct RgbaSurface<'a> {
    pub data: &'a [u8],
    pub width: u32,
    pub height: u32,
    pub stride: u32,
}

#[inline(always)]
pub fn divide_up_by_multiple(val: u32, align: u32) -> u32 {
    let mask: u32 = align - 1;
    (val + mask) / align
}

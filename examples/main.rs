extern crate image;
extern crate ddsfile;

fn main() {
    println!("This is the demo");

    let bump_data = include_bytes!("./bump.png");
    let lambertian_data = include_bytes!("./lambertian.jpg");
}
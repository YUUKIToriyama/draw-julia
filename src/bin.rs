use std::path::Path;

use image::{ColorType, ImageFormat};

use draw_julia::{
    complex::Complex,
    julia::get_julia_set,
    util::{Bound, RADIUS_OF_CONVERGENCE},
};

fn main() {
    let width = 500;
    let height = 500;
    let bound = Bound {
        north: RADIUS_OF_CONVERGENCE,
        south: -RADIUS_OF_CONVERGENCE,
        west: -RADIUS_OF_CONVERGENCE,
        east: RADIUS_OF_CONVERGENCE,
    };
    let c = Complex {
        re: -0.15,
        im: 0.65,
    };
    let image_buffer = get_julia_set(width, height, bound, c);
    image::save_buffer_with_format(
        Path::new("./generated.png"),
        &image_buffer,
        width,
        height,
        ColorType::Rgba8,
        ImageFormat::Png,
    )
    .unwrap();
}

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    /// マルチスレッドを有効にする
    #[arg(long = "multi-thread")]
    enable_multi_thread: Option<bool>,
    /// 横の長さを指定する(px)
    #[arg(long)]
    width: u32,
    /// 縦の長さを指定する(px)
    #[arg(long)]
    height: u32,
    /// 出力先
    #[arg(long = "out-file")]
    file_name: String,
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use image::{ColorType, ImageFormat};
    use std::path::Path;

    use draw_julia::{
        complex::Complex,
        julia::{get_julia_set_multi_thread, get_julia_set_single_thread},
        util::{Bound, RADIUS_OF_CONVERGENCE},
    };

    let args = Cli::parse();

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
    let image_buffer = match args.enable_multi_thread.unwrap_or(false) {
        true => get_julia_set_multi_thread(args.width, args.height, bound, c),
        false => get_julia_set_single_thread(args.width, args.height, bound, c),
    };
    image::save_buffer_with_format(
        Path::new(&args.file_name),
        &image_buffer,
        args.width,
        args.height,
        ColorType::Rgba8,
        ImageFormat::Png,
    )
    .unwrap();
}

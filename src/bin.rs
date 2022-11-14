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
    save_buffer_as_png(&args.file_name, args.width, args.height, &image_buffer);
}

#[cfg(not(target_arch = "wasm32"))]
fn save_buffer_as_png(file_name: &str, width: u32, height: u32, buffer: &Vec<u8>) {
    use std::{fs::File, io::BufWriter, path::Path};
    let path = Path::new(file_name);
    let file = File::create(path).unwrap();
    let mut encoder = png::Encoder::new(BufWriter::new(file), width, height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(buffer).unwrap();
}

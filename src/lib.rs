use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn draw_julia(
    context: &web_sys::CanvasRenderingContext2d,
    width: u32,
    height: u32,
    real: f64,
    imaginary: f64,
) -> Result<(), wasm_bindgen::JsValue> {
    let c = num_complex::Complex {
        re: real,
        im: imaginary,
    };
    let mut data = get_julia_set(width, height, c);
    let image_data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
        wasm_bindgen::Clamped(&mut data),
        width,
        height,
    )
    .unwrap();
    context.put_image_data(&image_data, 0.0, 0.0)
}

pub fn get_julia_set(width: u32, height: u32, c: num_complex::Complex<f64>) -> Vec<u8> {
    let mut data = Vec::new();

    let param_i = 1.5;
    let param_r = 1.5;
    let scale = 0.005;

    for x in 0..width {
        for y in 0..height {
            let z = num_complex::Complex::new(
                (y as f64) * scale - param_r,
                (x as f64) * scale - param_i,
            );
            let iter_index = get_iter_index(z, c);
            data.push((iter_index / 4) as u8);
            data.push((iter_index / 2) as u8);
            data.push(iter_index as u8);
            data.push(255);
        }
    }
    return data;
}

fn get_iter_index(z: num_complex::Complex<f64>, c: num_complex::Complex<f64>) -> u32 {
    let mut iter_index: u32 = 0;
    let mut z = z;
    while iter_index < 900 {
        if z.norm_sqr() > 2.0 {
            break;
        }
        z = z.powu(2) + c;
        iter_index = iter_index + 1;
    }
    iter_index
}

use wasm_bindgen::prelude::wasm_bindgen;

const RADIUS_OF_CONVERGENCE: f64 = 2.0;
const LIMIT: u32 = 900;

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

pub fn get_julia_set(canvas_width: u32, canvas_height: u32, c: num_complex::Complex<f64>) -> Vec<u8> {
    let mut data = Vec::new();

    let scale = 2.0 * RADIUS_OF_CONVERGENCE / (canvas_width as f64);

    for x in 0..canvas_width {
        for y in 0..canvas_height {
            let z0 = num_complex::Complex::new(
                RADIUS_OF_CONVERGENCE - (y as f64) * scale,
                RADIUS_OF_CONVERGENCE - (x as f64) * scale,
            );
            match calculate_sequence_limit(z0, c) {
                // 収束する場合は黒(#000000)
                SequenceLimit::Convergence => {
                    data.push(0); // R
                    data.push(0); // G
                    data.push(0); // B
                    data.push(255); // A
                }
                // 発散する場合は適当な色に着色する
                SequenceLimit::Divergence(count) => {
                    data.push((255 - count / 2) as u8); // R
                    data.push((255 - count / 4) as u8); // G
                    data.push((255 - count / 6) as u8); // B
                    data.push(255); // A
                }
            }
        }
    }
    return data;
}

enum SequenceLimit {
    Convergence,     // 収束
    Divergence(u32), // 発散
}

fn calculate_sequence_limit(
    z0: num_complex::Complex<f64>,
    c: num_complex::Complex<f64>,
) -> SequenceLimit {
    let mut result: SequenceLimit = SequenceLimit::Convergence;
    let mut n: u32 = 0;
    let mut z: num_complex::Complex<f64> = z0;
    // n -> LIMITの極限をとる
    while n < LIMIT {
        // 収束半径を超えた場合
        if z.norm() > RADIUS_OF_CONVERGENCE {
            result = SequenceLimit::Divergence(n);
            break;
        }
        // 漸化式を計算
        z = z.powu(2) + c;
        // ステップ数を更新
        n = n + 1;
    }
    return result;
}

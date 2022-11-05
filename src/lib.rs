use std::ops::Add;
use std::sync::{Arc, Mutex};
use std::thread;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};

const RADIUS_OF_CONVERGENCE: f64 = 2.0;
const LIMIT: u32 = 900;

#[derive(Serialize, Deserialize)]
struct Bound {
    north: f64,
    south: f64,
    west: f64,
    east: f64,
}

#[derive(Serialize, Deserialize)]
struct Constant {
    real: f64,
    imaginary: f64,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
struct Complex {
    re: f64,
    im: f64,
}

impl Complex {
    fn square(self) -> Complex {
        let real = (self.re * self.re) - (self.im * self.im);
        let imaginary = 2.0 * self.re * self.im;
        Complex {
            re: real,
            im: imaginary,
        }
    }

    fn norm(&self) -> f64 {
        (self.re * self.re) + (self.im * self.im)
    }
}

impl Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, z: Complex) -> Complex {
        Complex {
            re: self.re + z.re,
            im: self.im + z.im,
        }
    }
}

enum SequenceLimit {
    Convergence,     // 収束
    Divergence(u32), // 発散
}

#[wasm_bindgen]
pub struct JuliaSet {
    c: Complex,
}

#[wasm_bindgen]
impl JuliaSet {
    pub fn new(c: JsValue) -> JuliaSet {
        let constant: Complex = c.into_serde().unwrap();
        JuliaSet { c: constant }
    }

    /// 指定された<canvas>要素にジュリア集合を描画する
    pub fn draw(&self, canvas: &web_sys::HtmlCanvasElement, bound: JsValue) -> Result<(), JsValue> {
        // <canvas>要素のcontextを取得する
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        // <canvas>要素の横幅、縦幅を取得する
        let canvas_width = canvas.width();
        let canvas_height = canvas.height();

        // 描画する範囲を表す変数
        let bound: Bound = match bound.into_serde() {
            Ok(v) => v,
            Err(_) => Bound {
                north: RADIUS_OF_CONVERGENCE,
                south: -RADIUS_OF_CONVERGENCE,
                west: -RADIUS_OF_CONVERGENCE,
                east: RADIUS_OF_CONVERGENCE,
            }, // 省略されている場合、うまく変換できなかった場合は既定値を設定する
        };

        // 漸化式を計算して画像を生成する
        let mut data = Self::get_julia_set(canvas_width, canvas_height, bound, self.c);

        // ImageDataを作成する
        let image_data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
            wasm_bindgen::Clamped(&mut data),
            canvas_width,
            canvas_height,
        )
        .unwrap();
        // <canvas>要素に画像を書き込む
        context.put_image_data(&image_data, 0.0, 0.0)
    }

    /// ジュリア集合の画像データを生成する
    fn get_julia_set(canvas_width: u32, canvas_height: u32, bound: Bound, c: Complex) -> Vec<u8> {
        let data_size = canvas_height * canvas_width * 4;
        let data = Arc::new(Mutex::new(vec![0; data_size as usize]));

        let scale_x = (bound.east - bound.west).abs() / (canvas_width as f64);
        let scale_y = (bound.north - bound.south).abs() / (canvas_height as f64);

        let mut handles = vec![];

        for y in 0..canvas_height {
            for x in 0..canvas_width {
                let data = Arc::clone(&data);
                let handle = thread::spawn(move || {
                    // 初期値を設定
                    let z0 = Complex {
                        re: bound.west + (x as f64) * scale_x,
                        im: bound.south + (y as f64) * scale_y,
                    };
                    // 収束・発散を計算
                    let result = Self::calculate_sequence_limit(z0, c);
                    // 画像データに書き込む
                    let mut data = data.lock().unwrap();
                    let i = (y * canvas_width + x) as usize;
                    match result {
                        // 収束する場合は黒(#000000)
                        SequenceLimit::Convergence => {
                            (*data)[4 * i] = 0; // R
                            (*data)[4 * i + 1] = 0; // G
                            (*data)[4 * i + 2] = 0; // B
                            (*data)[4 * i + 3] = 0; // A
                        }
                        // 発散する場合は適当な色に着色する
                        SequenceLimit::Divergence(count) => {
                            (*data)[4 * i] = (255 - count / 2) as u8; // R
                            (*data)[4 * i + 1] = (255 - count / 4) as u8; // G
                            (*data)[4 * i + 2] = (255 - count / 6) as u8; // B
                            (*data)[4 * i + 3] = 255; // A
                        }
                    }
                });
                handles.push(handle);
            }
        }
        for handle in handles {
            handle.join().unwrap();
        }
        return Arc::try_unwrap(data).unwrap().lock().unwrap().to_vec();
    }

    /// 与えられた初期値に対して数列が収束するか発散するかを計算する
    fn calculate_sequence_limit(z0: Complex, c: Complex) -> SequenceLimit {
        let mut result: SequenceLimit = SequenceLimit::Convergence;
        let mut n: u32 = 0;
        let mut z: Complex = z0;
        // n -> LIMITの極限をとる
        while n < LIMIT {
            // 収束半径を超えた場合
            if z.norm() > RADIUS_OF_CONVERGENCE * RADIUS_OF_CONVERGENCE {
                result = SequenceLimit::Divergence(n);
                break;
            }
            // 漸化式を計算
            z = z.square() + c;
            // ステップ数を更新
            n = n + 1;
        }
        return result;
    }
}

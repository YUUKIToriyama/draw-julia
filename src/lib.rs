pub mod complex;
pub mod julia;
pub mod util;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};

use complex::Complex;
use julia::get_julia_set_single_thread;
use util::{Bound, RADIUS_OF_CONVERGENCE};

#[derive(Serialize, Deserialize)]
struct Constant {
    real: f64,
    imaginary: f64,
}

#[wasm_bindgen]
pub struct JuliaSet {
    c: Complex,
}

#[wasm_bindgen]
impl JuliaSet {
    pub fn new(c: JsValue) -> JuliaSet {
        let constant: Complex = serde_wasm_bindgen::from_value(c).unwrap();
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
        let bound: Bound = match serde_wasm_bindgen::from_value(bound) {
            Ok(v) => v,
            Err(_) => Bound {
                north: RADIUS_OF_CONVERGENCE,
                south: -RADIUS_OF_CONVERGENCE,
                west: -RADIUS_OF_CONVERGENCE,
                east: RADIUS_OF_CONVERGENCE,
            }, // 省略されている場合、うまく変換できなかった場合は既定値を設定する
        };

        // 漸化式を計算して画像を生成する
        let mut data = get_julia_set_single_thread(canvas_width, canvas_height, bound, self.c);

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
}

use std::sync::{Arc, Mutex};

#[cfg(not(target_arch = "wasm32"))]
use std::thread;
#[cfg(target_arch = "wasm32")]
use wasm_thread as thread;

use crate::complex::Complex;
use crate::util::{Bound, LIMIT, RADIUS_OF_CONVERGENCE};

/// 数列の極限
pub enum SequenceLimit {
    Convergence,     // 収束
    Divergence(u32), // 発散
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

/// ジュリア集合の画像データを生成する
pub fn get_julia_set(canvas_width: u32, canvas_height: u32, bound: Bound, c: Complex) -> Vec<u8> {
    let data_size = canvas_height * canvas_width * 4;
    let data = Arc::new(Mutex::new(vec![0; data_size as usize]));

    let scale_x = (bound.east - bound.west).abs() / (canvas_width as f64);
    let scale_y = (bound.north - bound.south).abs() / (canvas_height as f64);

    for y in 0..canvas_height {
        let mut handles = vec![];
        for x in 0..canvas_width {
            let data = Arc::clone(&data);
            let handle = thread::spawn(move || {
                // 初期値を設定
                let z0 = Complex {
                    re: bound.west + (x as f64) * scale_x,
                    im: bound.south + (y as f64) * scale_y,
                };
                // 収束・発散を計算
                let result = calculate_sequence_limit(z0, c);
                // 画像データに書き込む
                let mut data = data.lock().expect(&format!("{},{}", x, y));
                let i = (y * canvas_width + x) as usize;
                match result {
                    // 収束する場合は黒(#000000)
                    SequenceLimit::Convergence => {
                        data[4 * i] = 0; // R
                        data[4 * i + 1] = 0; // G
                        data[4 * i + 2] = 0; // B
                        data[4 * i + 3] = 0; // A
                    }
                    // 発散する場合は適当な色に着色する
                    SequenceLimit::Divergence(count) => {
                        data[4 * i] = (255 - count / 2) as u8; // R
                        data[4 * i + 1] = (255 - count / 4) as u8; // G
                        data[4 * i + 2] = (255 - count / 6) as u8; // B
                        data[4 * i + 3] = 255; // A
                    }
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
    }
    return Arc::try_unwrap(data).unwrap().lock().unwrap().to_vec();
}

use serde::{Deserialize, Serialize};
use std::ops::Add;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub fn square(self) -> Complex {
        let real = (self.re * self.re) - (self.im * self.im);
        let imaginary = 2.0 * self.re * self.im;
        Complex {
            re: real,
            im: imaginary,
        }
    }

    pub fn norm(&self) -> f64 {
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

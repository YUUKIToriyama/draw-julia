use serde::{Deserialize, Serialize};

pub const RADIUS_OF_CONVERGENCE: f64 = 2.0;
pub const LIMIT: u32 = 900;

#[derive(Serialize, Deserialize)]
pub struct Bound {
    pub north: f64,
    pub south: f64,
    pub west: f64,
    pub east: f64,
}

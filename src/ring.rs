use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Ring {
    pub a: f64,
    pub mass: f64,
    pub composition: f64,
    pub color: f64,
    pub width: f64,
}

impl Ring {
    pub fn new() -> Self {
        Ring {
            a: 0.0,
            mass: 0.0,
            composition: 0.0,
            color: 0.0,
            width: 0.0,
        }
    }
}
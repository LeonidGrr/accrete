use crate::structs::Planetesimal;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Ring {
    pub a: f64,
    pub mass: f64,
    pub width: f64,
}

impl Ring {
    pub fn new(roche_limit: f64, moon: &Planetesimal) -> Self {
        Ring {
            a: roche_limit,
            mass: moon.mass,
            width: moon.radius * 2.0,
        }
    }
}

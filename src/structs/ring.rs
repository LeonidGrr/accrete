use crate::structs::Planetesimal;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Ring {
    pub a: f64,
    pub mass: f64,
    pub width: f64,
}

impl Ring {
    pub fn from_planet(roche_limit: f64, moon: &Planetesimal) -> Self {
        Ring {
            a: roche_limit,
            mass: moon.mass,
            width: moon.radius * 2.0,
        }
    }

    pub fn new(a: f64, mass: f64, width: f64) -> Self {
        Ring { a, mass, width }
    }
}

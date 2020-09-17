use serde::Serialize;
use rand::prelude::*;
use crate::planetismal::Planetismal;
use crate::consts::*;

#[derive(Debug, Clone, Serialize)]
pub struct Ring {
    pub a: f64,
    pub mass: f64,
    pub width: f64,
}

impl Ring {
    pub fn new(roche_limit: f64, moon: &Planetismal) -> Self {
        Ring {
            a: roche_limit,
            mass: moon.mass,
            width: moon.radius * 2.0,
        }
    }
}

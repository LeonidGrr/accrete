use crate::{structs::Planetesimal, utils::random_id};

use rand::RngCore;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ring {
    pub a: f64,
    pub mass: f64,
    pub width: f64,
    pub id: String,
}

impl Ring {
    pub fn from_planet(roche_limit: f64, moon: &Planetesimal) -> Self {
        Ring {
            a: roche_limit,
            mass: moon.mass,
            width: moon.radius * 2.0,
            id: moon.id.clone(),
        }
    }

    pub fn new(a: f64, mass: f64, width: f64, rng: &mut dyn RngCore) -> Self {
        Ring {
            a,
            mass,
            width,
            id: random_id(rng),
        }
    }
}

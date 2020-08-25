#![allow(dead_code)]
use crate::consts;

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct AsteroidBelt {
    pub axis: f64,
    pub mass: f64,
}

impl AsteroidBelt {
    pub fn new(
        axis: f64,
        mass: f64,
    ) -> Self {
        AsteroidBelt { axis, mass }
    }

    pub fn get_earth_mass(&self) -> f64 {
        self.mass * consts::SOLAR_MASS_IN_EARTH_MASS
    }
}

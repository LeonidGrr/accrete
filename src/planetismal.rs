use serde::Serialize;
use crate::consts;
use crate::dole_params;

#[derive(Serialize, Debug, PartialOrd, PartialEq, Clone)]
pub struct Planetismal {
    pub axis: f64,
    pub eccn: f64,
    pub mass: f64,
    pub gas_giant: bool,
    pub moons: Vec<Planetismal>,
}

impl Planetismal {
    pub fn new(
        a: f64,
        e: f64,
        stellar_mass: &f64,
        mass: f64,
    ) -> Self {
        let axis = a * dole_params::outermost_planet(stellar_mass) + dole_params::innermost_planet(stellar_mass);
        let eccn = dole_params::random_eccentricity(e);
        let gas_giant = false;
        let moons = Vec::new();

        Planetismal {
            axis,
            eccn,
            mass,
            gas_giant,
            moons,
        }
    }

    pub fn get_earth_mass(&self) -> f64 {
        self.mass * consts::SOLAR_MASS_IN_EARTH_MASS
    }
}

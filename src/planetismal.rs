use serde::Serialize;
use crate::astro;

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
        axis: Option<f64>,
        eccn: Option<f64>,
        mass: Option<f64>,
        gas_giant: Option<bool>,
        moons: Option<Vec<Planetismal>>,
    ) -> Self {
        let axis = axis.unwrap_or(0.0);
        let eccn = eccn.unwrap_or(0.0);
        let mass = mass.unwrap_or(astro::PROTOPLANET_MASS);
        let gas_giant = gas_giant.unwrap_or(false);
        let moons = moons.unwrap_or_default();

        Planetismal {
            axis,
            eccn,
            mass,
            gas_giant,
            moons,
        }
    }

    pub fn get_earth_mass(&self) -> f64 {
        self.mass * astro::SOLAR_MASS_IN_EARTH_MASS
    }
}

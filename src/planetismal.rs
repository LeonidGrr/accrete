use serde::Serialize;
use crate::consts;

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
        axis: f64,
        eccn: f64,
        mass: Option<f64>,
        gas_giant: Option<bool>,
        moons: Option<Vec<Planetismal>>,
    ) -> Self {
        let mass = mass.unwrap_or(consts::PROTOPLANET_MASS);
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
        self.mass * consts::SOLAR_MASS_IN_EARTH_MASS
    }
}

use crate::structs::Planetesimal;
use serde::Serialize;

// "The asteroid belt formed from the primordial solar nebula as a group of planetesimals.[8] Planetesimals are the smaller precursors of the protoplanets. Between Mars and Jupiter, however, gravitational perturbations from Jupiter imbued the protoplanets with too much orbital energy for them to accrete into a planet.[8][9] Collisions became too violent, and instead of fusing together, the planetesimals and most of the protoplanets shattered."

#[derive(Debug, Clone, Serialize)]
pub struct Asteroids {
    pub a: f64,
    pub e: f64,
    pub mass: f64,
}

impl Asteroids {
    pub fn new(planetesimal: Planetesimal) -> Self {
        let Planetesimal { a, e, mass, .. } = planetesimal;
        Asteroids {
            a,
            e,
            mass,
        }
    }
}

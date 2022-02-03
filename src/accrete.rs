use crate::consts::*;
use crate::event_store::EventSource;
use crate::structs::planetesimal::Planetesimal;
use crate::structs::system::System;
use crate::utils::*;

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// ### Configuration:
///
/// **stellar_mass** - Primary star mass in solar masses.
/// *Default: random f64 in a range of 0.6-1.3 (corresponds main sequence spectral classes of F-G-K)*
///
/// **dust_density_coeff** - "A" in Dole's paper, recommended range according to Dole's paper is 0.00125-0.0015, aslo noted that binary stars produced by increasing coeff of dust density in cloud (Formation of Planetary Systems by Aggregation: A Computer Simulation by Stephen H. Dole).
/// *Default: 0.0015*
///
/// **k** - The dust-to-gas ratio 50-100 (dust/gas = K), gas = hydrogen and helium, dust = other. Recommended range: 50.0-100.0
/// *Default: 50.0*
///
/// **cloud_eccentricity** - Initial dust cloud cloud_eccentricity. High eccentricity reduce number of planets. Recommended range: 0.15-0.25.
/// *Default: 0.20*
///
/// **b** - Crit_mass coeff is used as threshold for planet to become gas giant. Recommended range: 1.0e-5 - 1.2e-5
/// *Default: 1.2e-5*
///
/// **post_accretion_intensity** - Amount of random planetesimals that will bomb planets of created system after accretion.
/// *Default: 1000*
///
/// Parameters specific for standalone planet generation
/// **planet_a** - Planet orbital radius in AU.
/// *Default: random f64 in a range of 0.3-50.0*
///
/// **planet_e** - Planet eccentricity
/// *Default: f64 from random_eccentricity function*
///
/// **planet_mass** - Planet mass in Earth masses.
/// *Default: Random f64 in a range 3.3467202125167E-10 - 500.0*
///
/// **stellar_luminosity** - Primary star luminosity.
/// *Default: 1.0*
#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Accrete {
    pub stellar_mass: f64,
    pub dust_density_coeff: f64,
    pub k: f64,
    pub cloud_eccentricity: f64,
    pub b: f64,
    pub post_accretion_intensity: u32,
    pub planet_a: f64,
    pub planet_e: f64,
    pub planet_mass: f64,
    pub stellar_luminosity: f64,
    rng: ChaCha8Rng,
}

impl Default for Accrete {
    fn default() -> Self {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let random_stellar_mass = rng.gen_range(0.6..1.3);
        let planet_a = rng.gen_range(0.3..50.0);
        let planet_e = random_eccentricity(&mut rng);
        let planet_mass = rng.gen_range(PROTOPLANET_MASS * EARTH_MASSES_PER_SOLAR_MASS..500.0)
            / EARTH_MASSES_PER_SOLAR_MASS;

        Accrete {
            stellar_mass: random_stellar_mass,
            dust_density_coeff: DUST_DENSITY_COEFF,
            k: K,
            cloud_eccentricity: 0.2,
            b: B,
            post_accretion_intensity: 1000,
            stellar_luminosity: 1.0,
            planet_a,
            planet_e,
            planet_mass,
            rng,
        }
    }
}

impl Accrete {
    pub fn new(seed: u64) -> Self {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let random_stellar_mass = rng.gen_range(0.6..1.3);
        let planet_a = rng.gen_range(0.3..50.0);
        let planet_e = random_eccentricity(&mut rng);
        let planet_mass = rng.gen_range(PROTOPLANET_MASS * EARTH_MASSES_PER_SOLAR_MASS..500.0)
            / EARTH_MASSES_PER_SOLAR_MASS;

        let accrete = Accrete {
            stellar_mass: random_stellar_mass,
            dust_density_coeff: DUST_DENSITY_COEFF,
            k: K,
            cloud_eccentricity: 0.2,
            b: B,
            post_accretion_intensity: 1000,
            stellar_luminosity: 1.0,
            planet_a,
            planet_e,
            planet_mass,
            rng,
        };

        accrete
    }

    /// Generate planetary system.
    pub fn planetary_system(&mut self) -> System {
        let Accrete {
            stellar_mass,
            dust_density_coeff,
            k,
            cloud_eccentricity,
            b,
            post_accretion_intensity,
            rng,
            ..
        } = self;

        let mut planetary_system = System::set_initial_conditions(
            *stellar_mass,
            *dust_density_coeff,
            *k,
            *cloud_eccentricity,
            *b,
        );

        planetary_system.distribute_planetary_masses(rng);
        planetary_system.post_accretion(*post_accretion_intensity, rng);
        planetary_system.process_planets(rng);

        planetary_system.event("system_complete");
        planetary_system
    }

    /// Generate planet.
    pub fn planet(&mut self) -> Planetesimal {
        let Accrete {
            stellar_mass,
            stellar_luminosity,
            planet_a,
            planet_e,
            planet_mass,
            post_accretion_intensity,
            rng,
            ..
        } = self;

        Planetesimal::random_planet(
            *stellar_luminosity,
            *stellar_mass,
            *planet_a,
            *planet_e,
            *planet_mass,
            *post_accretion_intensity,
            rng,
        )
    }
}

mod consts;
mod enviro;
mod structs;
mod utils;

use crate::structs::planetesimal::Planetesimal;
use crate::structs::system::System;
use crate::consts::*;
use crate::utils::*;
use rand::prelude::*;
use serde_json::json;

#[derive(Debug)]
pub enum AccreteOutput {
    Planet(Planetesimal),
    System(System),
    Json(String),
}

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
/// **to_json** - Output as JSON string.
/// *Default: false*
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
#[derive(Debug)]
pub struct Accrete {
    pub stellar_mass: f64,
    pub dust_density_coeff: f64,
    pub k: f64,
    pub cloud_eccentricity: f64,
    pub b: f64,
    pub post_accretion_intensity: u32,
    pub to_json: bool,
    pub planet_a: f64,
    pub planet_e: f64,
    pub planet_mass: f64,
    pub stellar_luminosity: f64,
}

impl Default for Accrete {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let random_stellar_mass = rng.gen_range(0.6, 1.3);
        let planet_a = rng.gen_range(0.3, 50.0);
        let planet_e = random_eccentricity();
        let planet_mass = rng.gen_range(PROTOPLANET_MASS * EARTH_MASSES_PER_SOLAR_MASS, 500.0)
            / EARTH_MASSES_PER_SOLAR_MASS;

        Accrete {
            stellar_mass: random_stellar_mass,
            dust_density_coeff: DUST_DENSITY_COEFF,
            k: K,
            cloud_eccentricity: 0.2,
            b: B,
            post_accretion_intensity: 1000,
            to_json: false,
            stellar_luminosity: 1.0,
            planet_a,
            planet_e,
            planet_mass,
        }
    }
}

impl Accrete {
    pub fn new() -> Self {
        Self::default()
    }

    /// Generate planetary system.
    pub fn planetary_system(&self) -> AccreteOutput {
        let Accrete {
            stellar_mass,
            dust_density_coeff,
            k,
            cloud_eccentricity,
            b,
            post_accretion_intensity,
            to_json,
            ..
        } = *self;

        let mut planetary_system = System::set_initial_conditions(
            stellar_mass,
            dust_density_coeff,
            k,
            cloud_eccentricity,
            b,
        );

        planetary_system.distribute_planetary_masses();
        planetary_system.post_accretion(post_accretion_intensity);
        planetary_system.process_planets();

        if to_json {
            let s = json!({
                "primary_star": planetary_system.primary_star,
                "planets": planetary_system.planets,
            })
            .to_string();
            return AccreteOutput::Json(s);
        }
        AccreteOutput::System(planetary_system)
    }

    /// Generate planet.
    pub fn planet(&self) -> AccreteOutput {
        let Accrete {
            stellar_mass,
            stellar_luminosity,
            planet_a,
            planet_e,
            planet_mass,
            post_accretion_intensity,
            to_json,
            ..
        } = *self;
        
        let planet = Planetesimal::random_planet(
            stellar_luminosity,
            stellar_mass,
            planet_a,
            planet_e,
            planet_mass,
            post_accretion_intensity,
        );

        if to_json {
            let s = json!({
                "planet": planet,
            })
            .to_string();
            return AccreteOutput::Json(s);
        }

        AccreteOutput::Planet(planet)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run_with_default_config() {
        let accrete = Accrete::new();
        accrete.planetary_system();
    }

    #[test]
    fn run_to_json() {
        let mut accrete = Accrete::new();
        accrete.to_json = true;
        accrete.planetary_system();
    }

    #[test]
    fn run_with_o_spectral_class() {
        let mut accrete = Accrete::new();
        accrete.stellar_mass = 60.0;
        accrete.planetary_system();
    }

    #[test]
    fn run_with_b_spectral_class() {
        let mut accrete = Accrete::new();
        accrete.stellar_mass = 18.0;
        accrete.planetary_system();
    }

    #[test]
    fn run_with_a_spectral_class() {
        let mut accrete = Accrete::new();
        accrete.stellar_mass = 2.1;
        accrete.planetary_system();
    }

    #[test]
    fn run_with_f_spectral_class() {
        let mut accrete = Accrete::new();
        accrete.stellar_mass = 1.3;
        accrete.planetary_system();
    }

    #[test]
    fn run_with_g_spectral_class() {
        let mut accrete = Accrete::new();
        accrete.stellar_mass = 1.0;
        accrete.planetary_system();
    }

    #[test]
    fn run_with_k_spectral_class() {
        let mut accrete = Accrete::new();
        accrete.stellar_mass = 0.8;
        accrete.planetary_system();
    }

    #[test]
    fn run_with_m_spectral_class() {
        let mut accrete = Accrete::new();
        accrete.stellar_mass = 0.3;
        accrete.planetary_system();
    }

    #[test]
    fn run_with_brown_dwarf() {
        let mut accrete = Accrete::new();
        accrete.stellar_mass = 0.1;
        accrete.planetary_system();
    }

    #[test]
    fn run_with_rogue_planet() {
        let mut accrete = Accrete::new();
        accrete.stellar_mass = 0.0005;
        accrete.planetary_system();
    }

    // // "Even small increases in A result in large increases in the total mass of the systems produced; increasing A also decreases the average number of planets per system. As may be seen in Figure 17, for A = 0.003 and 0.006 the planetary system has become a binary star sys-tem, the body near 9 a.u. having grown large enough to be considered a red dwarf star. Observationally, the two stars of smallest mass now known are members of a binary system designated L726-8; each star has a mass estimated at about 0.04Ms (about 40 times the mass of Jupiter) or 13,000M e. The lower theoretical limit to the mass of a star is believed to be near 0.02Ms. It will be noticed that the binary star systems still contain numerous planetary bodies. As A is increased still more the systems become multiple-star systems and the number of planetary companions diminishes. Actually, the results at the higher values of A should be considered only suggestive of the general trend, since the total mass of the "planetary" bodies is now becoming fairly high with respect to that of the central body, so that the original simplifying assumptions, which were adequate when the total planetary mass was well below 0.01Ms, no longer apply so satisfactorily. The gravitational attractions of the several large masses for each other can no longer be considered to have negligible effects on the secular stability of the systems. This is pushing the ACRETE program somewhat beyond its original intent (to create planetary systems similar to the solar system). However, it would be readily possible to modify the program slightly to provide more rigorously for cases in which some of the planetary bodies grow to stellar mass. In any event, the general trend is clear. Simply increasing the value assigned to one parameter makes it possible to generate widely spaced binary and multiple-star systems."

    #[test]
    fn high_density_dust() {
        let mut accrete = Accrete::new();
        accrete.dust_density_coeff = 0.05;
        accrete.planetary_system();
    }

    #[test]
    fn low_density_dust() {
        let mut accrete = Accrete::new();
        accrete.dust_density_coeff = 0.00125;
        accrete.planetary_system();
    }

    #[test]
    fn high_cloud_ecentricity() {
        let mut accrete = Accrete::new();
        accrete.cloud_eccentricity = 0.5;
        accrete.planetary_system();
    }

    #[test]
    fn low_cloud_ecentricity() {
        let mut accrete = Accrete::new();
        accrete.cloud_eccentricity = 0.1;
        accrete.planetary_system();
    }

    #[test]
    fn low_cloud_ecentricity_and_dust_density() {
        let mut accrete = Accrete::new();
        accrete.cloud_eccentricity = 0.05;
        accrete.dust_density_coeff = 0.035;
        accrete.planetary_system();
    }

    #[test]
    fn random_planet_default() {
        let accrete = Accrete::new();
        accrete.planet();
    }

    #[test]
    fn random_planet_to_json() {
        let mut accrete = Accrete::new();
        accrete.to_json = true; 
        accrete.planet();
    }
}

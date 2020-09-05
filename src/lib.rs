mod consts;
mod dust;
mod enviro;
mod planetismal;
mod system;
mod utils;

use consts::*;
use planetismal::Planetismal;
use rand::prelude::*;
use serde_json::json;
use system::System;

#[derive(Debug)]
pub enum AccreteOutput {
    Struct(Vec<Planetismal>),
    Json(String),
}

/// Generate planetary system.
///
/// # Arguments
///
/// planets_limit -
/// Limit number of planets
///
/// stellar_mass -
/// Preconfigured stellar mass
///
/// dust_density_coeff -
/// "A" in Dole's paper
/// Dole's paper tests ranges between 0.00125 and 0.0015
/// Binary stars produced by increasing coeff of dust density in cloud (Formation of Planetary Systems by Aggregation: A Computer Simulation by Stephen H. Dole)
/// Range: 0.00125-0.0015
/// Default: 0.0015
///
/// k -
/// The dust-to-gas ratio 50-100 (dust/gas = K), gas = hydrogen and helium, dust = other
/// Range: 50.0-100.0
/// Default: 50.0
///
/// cloud_eccentricity -
/// Range: 0.15-0.25
/// Default: 0.20
///
/// b -
/// Crit_mass coeff
/// Range: 1.0e-5 - 1.2e-5
/// Default: 1.2e-5
///
/// with_moons -
/// Enable moon generation by accretion and collision
///
/// to_json -
/// Return json instead of structs
pub fn run(
    planets_limit: Option<usize>,
    stellar_mass: Option<f64>,
    dust_density_coeff: Option<f64>,
    k: Option<f64>,
    cloud_eccentricity: Option<f64>,
    b: Option<f64>,
    with_moons: bool,
    to_json: bool,
) -> AccreteOutput {
    let mut rng = rand::thread_rng();
    let random_stellar_mass = rng.gen_range(0.6, 1.3);
    let stellar_mass = stellar_mass.unwrap_or(random_stellar_mass);

    let dust_density_coeff = dust_density_coeff.unwrap_or(DUST_DENSITY_COEFF);
    let k = k.unwrap_or(K);
    let cloud_eccentricity = cloud_eccentricity.unwrap_or(0.2);
    let b = b.unwrap_or(B);

    let mut planetary_system = System::set_initial_conditions(
        planets_limit,
        stellar_mass,
        dust_density_coeff,
        k,
        cloud_eccentricity,
        b,
        with_moons,
    );
    planetary_system.distribute_planetary_masses();
    planetary_system.generate_planetary_environment();

    if to_json {
        let s = json!({
            // "stellar_mass": stellar_mass,
            // "stellar_luminosity": stellar_luminosity,
            "planets": planetary_system.planets,
        })
        .to_string();
        return AccreteOutput::Json(s);
    }
    println!("{:#?}", planetary_system);
    AccreteOutput::Struct(planetary_system.planets)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run_with_default_config() {
        run(None, None, None, None, None, None, false, false);
    }

    #[test]
    fn planets_limit() {
        run(Some(2), None, None, None, None, None, false, false);
    }

    #[test]
    fn low_density_dust() {
        run(None, None, Some(0.00125), None, None, None, false, false);
    }

    #[test]
    fn high_density_dust() {
        run(None, None, Some(0.05), None, None, None, false, false);
    }

    #[test]
    fn massive_star() {
        run(None, Some(1.3), None, None, None, None, false, false);
    }

    #[test]
    fn small_star() {
        run(None, Some(0.3), None, None, None, None, false, false);
    }
}

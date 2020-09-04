mod system;
mod consts;
mod dust;
mod enviro;
mod planetismal;
mod utils;

use system::System;
use planetismal::Planetismal;
use serde_json::json;
use consts::*;
use rand::prelude::*;

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
/// Return json instead of struct

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

    
    // var anum;
    // var main_seq_life;
    // var age, r_ecosphere;
    // var r_greenhouse;
    // var spin_resonance;
    // let main_seq_life = 1.0E10 * (stellar_mass_ratio / stellar_luminosity_ratio);
    // if ((main_seq_life >= 6.0E9))
    // age = random_number(1.0E9, 6.0E9);
    // else
    // age = random_number(1.0E9, main_seq_life);
    // r_ecosphere = Math.sqrt(stellar_luminosity_ratio);
    // r_greenhouse = r_ecosphere * GREENHOUSE_EFFECT_CONST;

    // while (planet != NULL) {
    // planet.orbit_zone = orbital_zone(planet.a);
    // if (planet.gas_giant) {
    //     planet.density = empirical_density(planet.mass, planet.a, planet.gas_giant);
    //     planet.radius = volume_radius(planet.mass, planet.density);
    // } else {
    //     planet.radius = kothari_radius(planet.mass, planet.a, planet.gas_giant, planet.orbit_zone);
    //     planet.density = volume_density(planet.mass, planet.radius);
    // }
    // planet.orbital_period = period(planet.a, planet.mass, stellar_mass_ratio);
    // planet.day = day_length(planet.mass, planet.radius, planet.orbital_period, planet.e, planet.gas_giant);
    // planet.resonant_period = spin_resonance;
    // planet.axial_tilt = inclination(planet.a);
    // planet.escape_velocity = escape_vel(planet.mass, planet.radius);
    // planet.surface_accel = acceleration(planet.mass, planet.radius);
    // planet.rms_velocity = rms_vel(MOLECULAR_NITROGEN, planet.a);
    // planet.molecule_weight = molecule_limit(planet.a, planet.mass, planet.radius);
    // if ((planet.gas_giant)) {
    //     planet.surface_grav = INCREDIBLY_LARGE_NUMBER;
    //     planet.greenhouse_effect = FALSE;
    //     planet.volatile_gas_inventory = INCREDIBLY_LARGE_NUMBER;
    //     planet.surface_pressure = INCREDIBLY_LARGE_NUMBER;
    //     planet.boil_point = INCREDIBLY_LARGE_NUMBER;
    //     planet.hydrosphere = INCREDIBLY_LARGE_NUMBER;
    //     planet.albedo = about(GAS_GIANT_ALBEDO, 0.1);
    //     planet.surface_temp = INCREDIBLY_LARGE_NUMBER;
    // } else {
    //     planet.surface_grav = gravity(planet.surface_accel);
    //     planet.greenhouse_effect = greenhouse(planet.orbit_zone, planet.a, r_greenhouse);
    //     planet.volatile_gas_inventory = vol_inventory(planet.mass, planet.escape_velocity, planet.rms_velocity, stellar_mass_ratio, planet.orbit_zone, planet.greenhouse_effect);
    //     planet.surface_pressure = pressure(planet.volatile_gas_inventory, planet.radius, planet.surface_grav);
    //     if ((planet.surface_pressure == 0.0))
    //     planet.boil_point = 0.0;
    //     else
    //     planet.boil_point = boiling_point(planet.surface_pressure);
    //     iterate_surface_temp(planet);
    // }
    // planet = planet.next_planet;
    // }
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

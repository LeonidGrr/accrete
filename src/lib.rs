mod consts;
mod dust;
mod enviro;
mod planetismal;
mod ring;
mod system;
mod utils;

use consts::*;
use planetismal::*;
use system::*;
use rand::prelude::*;
use serde_json::json;

#[derive(Debug)]
pub enum AccreteOutput {
    Struct(Vec<Planetismal>),
    Json(String),
}

/// ## Generate planetary system.
///
/// ### Default:
/// ```rust
/// use accrete;
///
/// fn main() {
///     let planets = accrete::run(None, None, None, None, None, None, false);
/// }
/// ```
///
/// Simple way to variate output is to change stellar mass. This accrete implementation is capable of generating planetary system for any stellar mass, but better (most realistic) results achieved for main sequence star class with primary star mass of 0.6 - 1.3 solar masses.
///
/// ### Configuration:
///
/// **planets_limit** - Limit number of planets.
/// *Default: None*
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
/// **cloud_eccentricity** - Initial dust cloud cloud_eccentricity. Recommended range: 0.15-0.25.
/// *Default: 0.20*
///
/// **b** - Crit_mass coeff is used as threshold for planet to become gas giant. Recommended range: 1.0e-5 - 1.2e-5
/// *Default: 1.2e-5*
///
/// **to_json** - Output as JSON string.
/// *Default: false*
///
pub fn run(
    planets_limit: Option<usize>,
    stellar_mass: Option<f64>,
    dust_density_coeff: Option<f64>,
    k: Option<f64>,
    cloud_eccentricity: Option<f64>,
    b: Option<f64>,
    to_json: bool,
) -> AccreteOutput {
    let mut rng = rand::thread_rng();
    let random_stellar_mass = rng.gen_range(0.6, 1.3);
    let stellar_mass = stellar_mass.unwrap_or(random_stellar_mass);

    let dust_density_coeff = dust_density_coeff.unwrap_or(DUST_DENSITY_COEFF);
    let k = k.unwrap_or(K);
    let cloud_eccentricity = cloud_eccentricity.unwrap_or(0.2);
    let b = b.unwrap_or(B);

    let mut planetary_system = PrimaryStar::set_initial_conditions(
        planets_limit,
        stellar_mass,
        dust_density_coeff,
        k,
        cloud_eccentricity,
        b,
    );
    planetary_system.distribute_planetary_masses();
    planetary_system.process_planets();

    if to_json {
        let s = json!({
            // "stellar_mass": stellar_mass,
            // "stellar_luminosity": stellar_luminosity,
            // "planets": planetary_system.planets,
        })
        .to_string();
        return AccreteOutput::Json(s);
    }
    println!("{:#?}", planetary_system.planets);
    for (i, p) in planetary_system.planets.iter().enumerate() {
        println!("Planet {}", i);
        println!("mass {}", p.mass);
        println!("a {}", p.a);        
        println!("Moons: {}", p.moons.len());
        println!("------------------");
    }
    AccreteOutput::Struct(planetary_system.planets)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_with_default_config() {
        run(None, None, None, None, None, None, false);
    }

    #[test]
    fn run_with_o_spectral_class() {
        run(None, Some(60.0), None, None, None, None, false);
    }

    #[test]
    fn run_with_b_spectral_class() {
        run(None, Some(18.0), None, None, None, None, false);
    }

    #[test]
    fn run_with_a_spectral_class() {
        run(None, Some(2.1), None, None, None, None, false);
    }

    #[test]
    fn run_with_f_spectral_class() {
        run(None, Some(1.3), None, None, None, None, false);
    }

    #[test]
    fn run_with_g_spectral_class() {
        run(None, Some(1.0), None, None, None, None, false);
    }

    #[test]
    fn run_with_k_spectral_class() {
        run(None, Some(0.8), None, None, None, None, false);
    }

    #[test]
    fn run_with_m_spectral_class() {
        run(None, Some(0.3), None, None, None, None, false);
    }

    #[test]
    fn run_with_brown_dwarf() {
        run(None, Some(0.1), None, None, None, None, false);
    }

    #[test]
    fn run_with_rogue_planet() {
        run(None, Some(0.0005), None, None, None, None, false);
    }

    #[test]
    fn planets_limit() {
        run(Some(2), None, None, None, None, None, false);
    }

    #[test]
    fn low_density_dust() {
        run(None, None, Some(0.00125), None, None, None, false);
    }

    #[test]
    fn high_density_dust() {
        run(None, None, Some(0.05), None, None, None, false);
    }

    #[test]
    fn massive_star() {
        run(None, Some(1.3), None, None, None, None, false);
    }

    #[test]
    fn small_star() {
        run(None, Some(0.3), None, None, None, None, false);
    }
}

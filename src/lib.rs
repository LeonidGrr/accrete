mod consts;
mod structs;
mod enviro;
mod utils;

use consts::*;
use rand::prelude::*;
use serde_json::json;
use structs::system::System;

#[derive(Debug)]
pub enum AccreteOutput {
    Struct(System),
    Json(String),
}

/// ## Generate planetary system.
///
/// ### Default:
/// ```rust
/// let planets = accrete::run(None, None, None, None, None, None, false);
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
/// From original Dole paper:
/// "Certain parameters must be specified to obtain quantitative results: the density distribution within the cloud; the ratio of gas to dust; a definition of critical mass, or the planetary mass above which a planet can begin to accumulate gas in addition to dust; and the orbital eccentricity of particles within the cloud. Also, a few rules for the coalescence and growth of planets must be established. When these parameters and rules have been set forth in a suitable manner, planetary systems very much like the solar system can be created. Multiple-star systems can be created by changing a single parameter, the density level within the cloud."
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

    let mut planetary_system = System::set_initial_conditions(
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
    // println!("{:#?}", planetary_system.planets);
    for (i, p) in planetary_system.planets.iter().enumerate() {
        println!("Planet {}", i);
        println!("mass EM {}", p.mass * EARTH_MASSES_PER_SOLAR_MASS);
        println!("a {}", p.a);
        println!("is giant: {}", p.gas_giant);
        println!("Moons: {}", p.moons.len());
        println!("Rings: {}", p.rings.len());
        println!("------------------");
    }
    AccreteOutput::Struct(planetary_system)
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
        run(Some(15), None, None, None, None, None, false);
    }

    #[test]
    fn massive_star() {
        run(None, Some(1.3), None, None, None, None, false);
    }

    #[test]
    fn small_star() {
        run(None, Some(0.3), None, None, None, None, false);
    }

    // "Even small increases in A result in large increases in the total mass of the systems produced; increasing A also decreases the average number of planets per system. As may be seen in Figure 17, for A = 0.003 and 0.006 the planetary system has become a binary star sys-tem, the body near 9 a.u. having grown large enough to be considered a red dwarf star. Observationally, the two stars of smallest mass now known are members of a binary system designated L726-8; each star has a mass estimated at about 0.04Ms (about 40 times the mass of Jupiter) or 13,000M e. The lower theoretical limit to the mass of a star is believed to be near 0.02Ms. It will be noticed that the binary star systems still contain numerous planetary bodies. As A is increased still more the systems become multiple-star systems and the number of planetary companions diminishes. Actually, the results at the higher values of A should be considered only suggestive of the general trend, since the total mass of the "planetary" bodies is now becoming fairly high with respect to that of the central body, so that the original simplifying assumptions, which were adequate when the total planetary mass was well below 0.01Ms, no longer apply so satisfactorily. The gravitational attractions of the several large masses for each other can no longer be considered to have negligible effects on the secular stability of the systems. This is pushing the ACRETE program somewhat beyond its original intent (to create planetary systems similar to the solar system). However, it would be readily possible to modify the program slightly to provide more rigorously for cases in which some of the planetary bodies grow to stellar mass. In any event, the general trend is clear. Simply increasing the value assigned to one parameter makes it possible to generate widely spaced binary and multiple-star systems."

    #[test]
    fn high_density_dust() {
        run(None, None, Some(0.05), None, None, None, false);
    }

    #[test]
    fn low_density_dust() {
        run(None, None, Some(0.00125), Some(25.0), None, None, false);
    }
}

mod consts;
mod enviro;
mod structs;
mod utils;

use consts::*;
use rand::prelude::*;
use serde_json::json;
use structs::planetesimal::Planetesimal;
use structs::system::System;

#[derive(Debug)]
pub enum AccreteOutput {
    Planet(Planetesimal),
    System(System),
    Json(String),
}

/// ## Generate planetary system.
///
/// ### Default:
/// ```rust
/// let system = accrete::planetary_system(None, None, None, None, None, None, None, false);
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
pub fn planetary_system(
    planets_limit: Option<usize>,
    stellar_mass: Option<f64>,
    dust_density_coeff: Option<f64>,
    k: Option<f64>,
    cloud_eccentricity: Option<f64>,
    b: Option<f64>,
    post_accretion_intensity: Option<u32>,
    to_json: bool,
) -> AccreteOutput {
    let mut rng = rand::thread_rng();
    let random_stellar_mass = rng.gen_range(0.6, 1.3);
    let stellar_mass = stellar_mass.unwrap_or(random_stellar_mass);
    let dust_density_coeff = dust_density_coeff.unwrap_or(DUST_DENSITY_COEFF);
    let k = k.unwrap_or(K);
    let cloud_eccentricity = cloud_eccentricity.unwrap_or(0.2);
    let b = b.unwrap_or(B);
    let intensity = post_accretion_intensity.unwrap_or(1000);

    let mut planetary_system = System::set_initial_conditions(
        planets_limit,
        stellar_mass,
        dust_density_coeff,
        k,
        cloud_eccentricity,
        b,
    );

    planetary_system.distribute_planetary_masses();
    planetary_system.post_accretion(intensity);
    planetary_system.process_planets();

    for (i, p) in planetary_system.planets.iter().enumerate() {
        println!("Planet {}", i);
        println!("mass EM {}", p.mass * EARTH_MASSES_PER_SOLAR_MASS);
        println!("a {}", p.a);
        println!("is giant: {}", p.is_gas_giant);
        println!("is dwarf: {}", p.is_dwarf_planet);
        println!("Moons: {}", p.moons.len());
        println!("Rings: {}", p.rings.len());
        println!("------------------");
    }

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

/// ## Generate planet.
///
/// ### Default:
/// ```rust
/// let system = accrete::planet(None, None, None, None, None, None, false);
/// ```
///
/// ### Configuration:
///
/// **stellar_luminosity** - Primary star luminosity.
/// *Default: 1.0*
///
/// **stellar_mass** - Primary star mass in solar masses.
/// *Default: 1.0*
///
/// **a** - Planet orbital radius in AU.
/// *Default: random f64 in a range of 0.3-50.0*
///
/// **e** - Planet eccentricity
/// *Default: f64 from random_eccentricity function*
///
/// **mass** - Planet mass in Earth masses.
/// *Default: Random f64 in a range 3.3467202125167E-10 - 500.0*
///
/// **post_accretion_intensity** - Amount of random planetesimals that will bomb planet after accretion.
/// *Default: 100*
///
/// **to_json** - Output as JSON string.
/// *Default: false*
///
pub fn planet(
    stellar_luminosity: Option<f64>,
    stellar_mass: Option<f64>,
    a: Option<f64>,
    e: Option<f64>,
    mass: Option<f64>,
    post_accretion_intensity: Option<u32>,
    to_json: bool,
) -> AccreteOutput {
    let planet = Planetesimal::random_planet(
        stellar_luminosity,
        stellar_mass,
        a,
        e,
        mass,
        post_accretion_intensity,
    );
    println!("{:#?}", planet);

    if to_json {
        let s = json!({
            "planet": planet,
        })
        .to_string();
        return AccreteOutput::Json(s);
    }

    AccreteOutput::Planet(planet)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run_with_default_config() {
        planetary_system(None, None, None, None, None, None, None, false);
    }

    #[test]
    fn run_to_json() {
        planetary_system(None, None, None, None, None, None, None, true);
    }

    #[test]
    fn run_with_o_spectral_class() {
        planetary_system(None, Some(60.0), None, None, None, None, None, false);
    }

    #[test]
    fn run_with_b_spectral_class() {
        planetary_system(None, Some(18.0), None, None, None, None, None, false);
    }

    #[test]
    fn run_with_a_spectral_class() {
        planetary_system(None, Some(2.1), None, None, None, None, None, false);
    }

    #[test]
    fn run_with_f_spectral_class() {
        planetary_system(None, Some(1.3), None, None, None, None, None, false);
    }

    #[test]
    fn run_with_g_spectral_class() {
        planetary_system(None, Some(1.0), None, None, None, None, None, false);
    }

    #[test]
    fn run_with_k_spectral_class() {
        planetary_system(None, Some(0.8), None, None, None, None, None, false);
    }

    #[test]
    fn run_with_m_spectral_class() {
        planetary_system(None, Some(0.3), None, None, None, None, None, false);
    }

    #[test]
    fn run_with_brown_dwarf() {
        planetary_system(None, Some(0.1), None, None, None, None, None, false);
    }

    #[test]
    fn run_with_rogue_planet() {
        planetary_system(None, Some(0.0005), None, None, None, None, None, false);
    }

    #[test]
    fn planets_limit() {
        planetary_system(Some(5), None, None, None, None, None, None, false);
    }

    #[test]
    fn massive_star() {
        planetary_system(None, Some(1.3), None, None, None, None, None, false);
    }

    #[test]
    fn small_star() {
        planetary_system(None, Some(0.3), None, None, None, None, None, false);
    }

    // "Even small increases in A result in large increases in the total mass of the systems produced; increasing A also decreases the average number of planets per system. As may be seen in Figure 17, for A = 0.003 and 0.006 the planetary system has become a binary star sys-tem, the body near 9 a.u. having grown large enough to be considered a red dwarf star. Observationally, the two stars of smallest mass now known are members of a binary system designated L726-8; each star has a mass estimated at about 0.04Ms (about 40 times the mass of Jupiter) or 13,000M e. The lower theoretical limit to the mass of a star is believed to be near 0.02Ms. It will be noticed that the binary star systems still contain numerous planetary bodies. As A is increased still more the systems become multiple-star systems and the number of planetary companions diminishes. Actually, the results at the higher values of A should be considered only suggestive of the general trend, since the total mass of the "planetary" bodies is now becoming fairly high with respect to that of the central body, so that the original simplifying assumptions, which were adequate when the total planetary mass was well below 0.01Ms, no longer apply so satisfactorily. The gravitational attractions of the several large masses for each other can no longer be considered to have negligible effects on the secular stability of the systems. This is pushing the ACRETE program somewhat beyond its original intent (to create planetary systems similar to the solar system). However, it would be readily possible to modify the program slightly to provide more rigorously for cases in which some of the planetary bodies grow to stellar mass. In any event, the general trend is clear. Simply increasing the value assigned to one parameter makes it possible to generate widely spaced binary and multiple-star systems."

    #[test]
    fn high_density_dust() {
        planetary_system(None, None, Some(0.05), None, None, None, None, false);
    }

    #[test]
    fn low_density_dust() {
        planetary_system(
            None,
            None,
            Some(0.00125),
            Some(25.0),
            None,
            None,
            None,
            false,
        );
    }

    #[test]
    fn high_cloud_ecentricity() {
        planetary_system(None, None, None, None, Some(0.5), None, None, false);
    }

    #[test]
    fn low_cloud_ecentricity() {
        planetary_system(None, None, None, None, Some(0.1), None, None, false);
    }

    #[test]
    fn low_cloud_ecentricity_and_dust_density() {
        planetary_system(None, None, Some(0.035), None, Some(0.05), None, None, false);
    }

    #[test]
    fn random_planet_default() {
        planet(None, None, None, None, None, None, false);
    }

    #[test]
    fn random_planet_to_json() {
        planet(None, None, None, None, None, None, true);
    }
}

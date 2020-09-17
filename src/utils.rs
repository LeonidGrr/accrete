use crate::consts::*;
use rand::prelude::*;

pub fn about(value: f64, variation: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(value - variation, value + variation)
}

pub fn innermost_planet(stellar_mass_ratio: &f64) -> f64 {
    0.3 * stellar_mass_ratio.powf(0.33)
}

pub fn outermost_planet(stellar_mass_ratio: &f64) -> f64 {
    50.0 * stellar_mass_ratio.powf(0.33)
}

pub fn inner_effect_limit(a: &f64, e: &f64, mass: &f64, cloud_eccentricity: &f64) -> f64 {
    a * (1.0 - e) * (1.0 - mass) / (1.0 + cloud_eccentricity)
}

pub fn outer_effect_limit(a: &f64, e: &f64, mass: &f64, cloud_eccentricity: &f64) -> f64 {
    a * (1.0 + e) * (1.0 + mass) / (1.0 - cloud_eccentricity)
}

/// The distance between the orbiting body and the sun at it's closest approach.
pub fn _perihelion_distance(radius: &f64, eccentricity: &f64) -> f64 {
    radius * (1.0 - eccentricity)
}

// The distance between the orbiting body and the sun at it's furthest approach.
pub fn _aphelion_distance(radius: &f64, eccentricity: &f64) -> f64 {
    radius * (1.0 + eccentricity)
}

pub fn random_eccentricity(random: f64, cloud_eccentricity: &f64) -> f64 {
    1.0 - random.powf(*cloud_eccentricity)
}

pub fn roche_limit_au(planet_mass: &f64, moon_mass: &f64, moon_radius: &f64) -> f64 {
    moon_radius / KM_PER_AU * (2.0 * (planet_mass / moon_mass)).powf(1.0 / 3.0)
}

pub fn hill_sphere_au(
    planet_axis: &f64,
    planet_eccn: &f64,
    planet_mass: &f64,
    moon_mass: &f64,
) -> f64 {
    planet_axis * (1.0 - planet_eccn) * (moon_mass / (3.0 * planet_mass)).powf(1.0 / 3.0)
}

/// Orbital radius is in AU, eccentricity is unitless, and the stellar luminosity ratio is with respect to the sun.
/// The value returned is the mass at which the planet begins to accrete gas as well as dust, and is in units of solar masses.
pub fn critical_limit(
    b: &f64,
    orbital_radius: &f64,
    eccentricity: &f64,
    stellar_luminosity_ratio: &f64,
) -> f64 {
    let perihelion_dist = orbital_radius - orbital_radius * eccentricity;
    let temp = perihelion_dist * stellar_luminosity_ratio.sqrt();
    b * temp.powf(-0.75)
}

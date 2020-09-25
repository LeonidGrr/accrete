use crate::consts::*;
use rand::prelude::*;

pub fn about(value: f64, variation: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(value - variation, value + variation)
}

pub fn reduced_mass(mass: &f64) -> f64 {
    (mass / (1.0 + mass)).powf(1.0 / 4.0)
}

pub fn inner_effect_limit(a: &f64, e: &f64, mass: &f64) -> f64 {
    let mass = reduced_mass(mass);
    perihelion_distance(a, e) * (1.0 - mass)
}

pub fn outer_effect_limit(a: &f64, e: &f64, mass: &f64) -> f64 {
    let mass = reduced_mass(mass);
    aphelion_distance(a, e) * (1.0 + mass)
}

pub fn inner_swept_limit(a: &f64, e: &f64, mass: &f64, cloud_eccentricity: &f64) -> f64 {
    inner_effect_limit(a, e, mass) / (1.0 + cloud_eccentricity)
}

pub fn outer_swept_limit(a: &f64, e: &f64, mass: &f64, cloud_eccentricity: &f64) -> f64 {
    outer_effect_limit(a, e, mass) / (1.0 - cloud_eccentricity)
}

/// The distance between the orbiting body and the sun at it's closest approach.
pub fn perihelion_distance(radius: &f64, eccentricity: &f64) -> f64 {
    radius * (1.0 - eccentricity)
}

// The distance between the orbiting body and the sun at it's furthest approach.
pub fn aphelion_distance(radius: &f64, eccentricity: &f64) -> f64 {
    radius * (1.0 + eccentricity)
}

pub fn random_eccentricity() -> f64 {
    let mut rng = rand::thread_rng();
    let random = rng.gen_range(0.0, 1.0);
    1.0 - (1.0 - random as f64).powf(ECCENTRICITY_COEFF)
}

/// Roche limit for planet / moon system in AU. Moon radius passes in AU, masses in solar mass.
pub fn roche_limit_au(planet_mass: &f64, moon_mass: &f64, moon_radius: &f64) -> f64 {
    moon_radius / KM_PER_AU * (2.0 * (planet_mass / moon_mass)).powf(1.0 / 3.0)
}

/// Hill sphere radius for planet / moon system in AU.
pub fn hill_sphere_au(
    planet_axis: &f64,
    planet_eccn: &f64,
    planet_mass: &f64,
    stellar_mass: &f64,
) -> f64 {
    planet_axis * (1.0 - planet_eccn) * (planet_mass / (3.0 * stellar_mass)).powf(1.0 / 3.0)
}

/// Clearing neightbourhood around planets orbit Margot's П discriminant (masses in solar mass, axis untiless)
pub fn clearing_neightbourhood(planet_mass: &f64, planet_axis: &f64, stellar_mass: &f64) -> f64 {
    let planet_mass_earth_mass = planet_mass * EARTH_MASSES_PER_SOLAR_MASS;
    // Coeff depends on extent of clearing required and parents star lifetime (~10 billion years for main-seq)
    let k = 807.0;
    (planet_mass_earth_mass / (stellar_mass.powf(5.0 / 2.0) * planet_axis.powf(9.0 / 8.0))) * k
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_earth_moon_roche_limit() {
        let earth_mass = 2.988000001494E-6;
        let moon_mass = earth_mass * 0.012;
        let moon_radius = 1737.5;
        let earth_moon_roche_limit =
            roche_limit_au(&earth_mass, &moon_mass, &moon_radius) * KM_PER_AU;
        assert!(earth_moon_roche_limit > 9400.0 && earth_moon_roche_limit < 9600.0);
    }
}

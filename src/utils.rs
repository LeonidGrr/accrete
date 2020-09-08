use rand::prelude::*;

pub fn about(value: f64, variation: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(value - variation, value + variation)
}

pub fn innermost_planet(stellar_mass_ratio: f64) -> f64 {
    0.3 * stellar_mass_ratio.powf(0.33)
}

pub fn outermost_planet(stellar_mass_ratio: f64) -> f64 {
    50.0 * stellar_mass_ratio.powf(0.33)
}

pub fn _innermost_moon(planetary_mass: &f64) -> f64 {
    0.001 * planetary_mass.powf(0.33)
}

pub fn _outermost_moon(planetary_mass: &f64) -> f64 {
    4.0 * planetary_mass.powf(0.33)
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

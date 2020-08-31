use crate::consts::*;

pub fn stellar_dust_limit(stellar_mass_ratio: &f64) -> f64 {
    200.0 * stellar_mass_ratio.powf(0.33)
}

pub fn innermost_planet(stellar_mass_ratio: &f64) -> f64 {
    0.3 * stellar_mass_ratio.powf(0.33)
}

pub fn outermost_planet(stellar_mass_ratio: &f64) -> f64 {
    50.0 * stellar_mass_ratio.powf(0.33)
}

pub fn innermost_moon(planetary_mass: &f64) -> f64 {
    0.001 * planetary_mass.powf(0.33)
}

pub fn outermost_moon(planetary_mass: &f64) -> f64 {
    4.0 * planetary_mass.powf(0.33)
}
pub fn inner_effect_limit(a: &f64, e: &f64, mass: &f64, cloud_eccentricity: &f64) -> f64 {
    a * (1.0 - e) * (1.0 - mass) / (1.0 + cloud_eccentricity)
}

pub fn outer_effect_limit(a: &f64, e: &f64, mass: &f64, cloud_eccentricity: &f64) -> f64 {
    a * (1.0 + e) * (1.0 + mass) / (1.0 - cloud_eccentricity)
}

pub fn mass_density(dust_density: &f64, critical_mass: &f64, mass: &f64) -> f64 {
    K * dust_density / (1.0 + (critical_mass / mass).sqrt() * (K - 1.0))
}

pub fn dust_density(stellar_mass: &f64, oribital_radius: &f64) -> f64 {
    DUST_DENSITY_COEFF * stellar_mass.sqrt() * (-ALPHA * oribital_radius.powf(1.0 / N)).exp()
}

/// Orbital radius is in AU, eccentricity is unitless, and the stellar luminosity ratio is with respect to the sun.
/// The value returned is the mass at which the planet begins to accrete gas as well as dust, and is in units of solar masses.
pub fn critical_limit(
    orbital_radius: &f64,
    eccentricity: &f64,
    stellar_luminosity_ratio: &f64,
) -> f64 {
    let perihelion_dist = orbital_radius - orbital_radius * eccentricity;
    let temp = perihelion_dist * stellar_luminosity_ratio.sqrt();
    B * temp.powf(-0.75)
}

/// The distance between the orbiting body and the sun at it's closest approach.
pub fn perihelion_distance(radius: &f64, eccentricity: &f64) -> f64 {
    radius * (1.0 - eccentricity)
}

// The distance between the orbiting body and the sun at it's furthest approach.
pub fn aphelion_distance(radius: &f64, eccentricity: &f64) -> f64 {
    radius * (1.0 + eccentricity)
}

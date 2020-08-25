use crate::consts::*;

pub fn critical_mass(radius: &f64, eccentricity: &f64, luminosity: &f64) -> f64 {
    B * (perihelion_distance(radius, eccentricity) * luminosity.sqrt()).powf(-0.75)
}

/// The distance between the orbiting body and the sun at it's closest approach.
pub fn perihelion_distance(radius: &f64, eccentricity: &f64) -> f64 {
    radius * (1.0 - eccentricity)
}

// The distance between the orbiting body and the sun at it's furthest approach.
pub fn aphelion_distance(radius: &f64, eccentricity: &f64) -> f64 {
    radius * (1.0 + eccentricity)
}

pub fn reduced_mass(mass: &f64) -> f64 {
    mass / (1.0 + mass)
}

pub fn reduced_margin(mass: &f64) -> f64 {
    reduced_mass(mass).powf(0.25)
}

pub fn low_bound(inner: &f64) -> f64 {
    inner / (1.0 + CLOUD_ECCENTRICITY)
}

pub fn high_bound(outer: &f64) -> f64 {
    outer / (1.0 - CLOUD_ECCENTRICITY)
}

pub fn inner_effect_limit(mass: &f64, axis: &f64, eccn: &f64) -> f64 {
    let mass = reduced_margin(mass);
    perihelion_distance(axis, eccn) * (1.0 - mass)
}

pub fn outer_effect_limit(mass: &f64, axis: &f64, eccn: &f64) -> f64 {
    let mass = reduced_margin(mass);
    aphelion_distance(axis, eccn) * (1.0 + mass)
}

// TODO: Not sure quite yet if we're interacting with this in a way where we can't call inner_effect_limit here...
pub fn inner_swept_limit(mass: &f64, axis: &f64, eccn: &f64) -> f64 {
    low_bound(&inner_effect_limit(mass, axis, eccn))
}

pub fn outer_swept_limit(mass: &f64, axis: &f64, eccn: &f64) -> f64 {
    high_bound(&outer_effect_limit(mass, axis, eccn))
}

pub fn dust_density(stellar_mass: &f64, oribital_radius: &f64) -> f64 {
    DUST_DENSITY_COEFF * stellar_mass.sqrt() * (-ALPHA * oribital_radius.powf(1.0 / N)).exp()
}

pub fn mass_density(dust_density: &f64, critical_mass: &f64, mass: &f64) -> f64 {
    K * dust_density / (1.0 + (critical_mass / mass).sqrt() * (K - 1.0))
}

pub fn scale_cube_root_mass(scale: &f64, mass: &f64) -> f64 {
    scale * mass.powf(0.33)
}

pub fn inner_dust_limit(_stellar_mass: &f64) -> f64 {
    0.0
}

pub fn outer_dust_limit(stellar_mass: &f64) -> f64 {
    scale_cube_root_mass(&200.0, stellar_mass)
}

pub fn innermost_planet(stellar_mass: &f64) -> f64 {
    scale_cube_root_mass(&0.3, stellar_mass)
}

pub fn outermost_planet(mass: &f64) -> f64 {
    scale_cube_root_mass(&50.0, mass)
}

pub fn random_eccentricity(random: f64) -> f64 {
    1.0 - random.powf(ECCENTRICITY_COEFF)
}

pub fn planet_outer_swept_limit(planetary_mass: &f64) -> f64 {
    0.01 * planetary_mass.powf(0.33)
}

pub fn planet_outer_dust_limit(planetary_mass: &f64) -> f64 {
    scale_cube_root_mass(&4.0, planetary_mass)
}

pub fn innermost_moon(planetary_mass: &f64) -> f64 {
    scale_cube_root_mass(&0.001, planetary_mass)
}

pub fn outermost_moon(planetary_mass: &f64) -> f64 {
    scale_cube_root_mass(&4.0, planetary_mass)
}

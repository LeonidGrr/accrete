use rand::prelude::*;
use crate::consts::*;

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

pub fn random_eccentricity(random: f64, cloud_eccentricity: &f64) -> f64 {
    1.0 - random.powf(*cloud_eccentricity)
}

pub fn roche_limit_au(planet_mass: &f64, moon_mass: &f64, moon_radius: &f64) -> f64 {
    moon_radius / KM_PER_AU * (2.0 * (planet_mass / moon_mass)).powf(1.0 / 3.0)
}

pub fn hill_sphere_au(planet_axis: &f64, planet_eccn: &f64, planet_mass: &f64, moon_mass: &f64) -> f64 {
    planet_axis * (1.0 - planet_eccn) * (moon_mass / (3.0 * planet_mass)).powf(1.0 / 3.0)
}

// 					if ((roche_limit * 3.0) < hill_sphere)
// 					{
// 						ptr->moon_a = random_number(roche_limit * 1.5, hill_sphere / 2.0) / KM_PER_AU;
// 						ptr->moon_e = random_eccentricity ();
// 					}
// 					else
// 					{
// 						ptr->moon_a = 0;
// 						ptr->moon_e = 0;
// 					}

// 					if (flag_verbose & 0x40000)
// 					{
// 						fprintf (stderr,
// 									"   Roche limit: R = %4.2Lg, rM = %4.2Lg, rm = %4.2Lg -> %.0Lf km\n"
// 									"   Hill Sphere: a = %4.2Lg, m = %4.2Lg, M = %4.2Lg -> %.0Lf km\n"
// 									"%s Moon orbit: a = %.0Lf km, e = %.0Lg\n",
// 									planet->radius, planet->density, ptr->density,
// 									roche_limit,
// 									planet->a * KM_PER_AU, planet->mass * SOLAR_MASS_IN_KILOGRAMS, sun->mass * SOLAR_MASS_IN_KILOGRAMS,
// 									hill_sphere,
// 									moon_id,
// 									ptr->moon_a * KM_PER_AU, ptr->moon_e
// 								);
// 					}

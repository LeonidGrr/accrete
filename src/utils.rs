use crate::consts::*;
use rand::prelude::*;

pub fn about(value: f64, variation: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(value - variation, value + variation)
}
pub fn stellar_dust_limit(stellar_mass_ratio: &f64) -> f64 {
    200.0 * stellar_mass_ratio.powf(0.33)
}

pub fn innermost_planet(stellar_mass_ratio: &f64) -> f64 {
    0.3 * stellar_mass_ratio.powf(0.33)
}

pub fn outermost_planet(stellar_mass_ratio: &f64) -> f64 {
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

pub fn mass_density(k: &f64, dust_density: &f64, critical_mass: &f64, mass: &f64) -> f64 {
    k * dust_density / (1.0 + (critical_mass / mass).sqrt() * (k - 1.0))
}

pub fn dust_density(dust_density_coeff: &f64, stellar_mass: &f64, oribital_radius: &f64) -> f64 {
    dust_density_coeff * stellar_mass.sqrt() * (-ALPHA * oribital_radius.powf(1.0 / N)).exp()
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

/// The distance between the orbiting body and the sun at it's closest approach.
pub fn _perihelion_distance(radius: &f64, eccentricity: &f64) -> f64 {
    radius * (1.0 - eccentricity)
}

// The distance between the orbiting body and the sun at it's furthest approach.
pub fn _aphelion_distance(radius: &f64, eccentricity: &f64) -> f64 {
    radius * (1.0 + eccentricity)
}

pub fn smallest_molecular_weight(m: f64) -> String 
{
    let mut s = String::new();
    
    if m < MOLECULAR_HYDROGEN {
        s = "H2".to_owned();
    } else if m < HELIUM {
        s = "He".to_owned();
    } else if m < METHANE {
        s = "CH4".to_owned();
    } else if m < AMMONIA {
        s = "NH3".to_owned();
    } else if m < WATER_VAPOR {
        s = "H2O".to_owned();
    } else if m < NEON {
        s = "Ne".to_owned();
    } else if m < MOLECULAR_NITROGEN {
        s = "N2".to_owned();
    } else if m < CARBON_MONOXIDE {
        s = "CO".to_owned();
    } else if m < NITRIC_OXIDE {
        s = "NO".to_owned();
    } else if m < MOLECULAR_OXYGEN {
        s = "O2".to_owned();
    } else if m < HYDROGEN_SULPHIDE {
        s = "H2S".to_owned();
    } else if m < ARGON {
        s = "Ar".to_owned();
    } else if m < CARBON_DIOXIDE {
        s = "CO2".to_owned();
    } else if m < NITROUS_OXIDE {
        s = "N2O".to_owned();
    } else if m < NITROGEN_DIOXIDE {
        s = "NO2".to_owned();
    } else if m < OZONE {
        s = "O3".to_owned();
    } else if m < SULPHUR_DIOXIDE {
        s = "SO2".to_owned();
    } else if m < SULPHUR_TRIOXIDE {
        s = "SO3".to_owned();
    } else if m < KRYPTON {
        s = "Kr".to_owned();
    } else if m < XENON {
        s = "Xe".to_owned();
    } else {
        s = "OTHER".to_owned();
    }

    s
}
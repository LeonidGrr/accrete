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

/// hasNebulae
/// Stars greater than 8 solar masses (M⊙) will likely end their lives in dramatic supernovae explosions, while planetary nebulae seemingly only occur at the end of the lives of intermediate and low mass stars between 0.8 M⊙ to 8.0 M⊙.[26] Progenitor stars that form planetary nebulae will spend most of their lifetimes converting their hydrogen into helium in the star's core by nuclear fusion at about 15 million K. This generated energy creates outward pressure from fusion reactions in the core, balancing the crushing inward pressures of the star's gravity.[27] This state of equilibrium is known as the main sequence, which can last for tens of millions to billions of years, depending on the mass.

// When the hydrogen source in the core starts to diminish, gravity starts compressing the core, causing a rise in temperature to about 100 million K.[28] Such higher core temperatures then make the star's cooler outer layers expand to create much larger red giant stars. This end phase causes a dramatic rise in stellar luminosity, where the released energy is distributed over a much larger surface area, which in fact causes the average surface temperature to be lower. In stellar evolution terms, stars undergoing such increases in luminosity are known as asymptotic giant branch stars (AGB).[28] During this phase, the star can lose 50 to 70% of its total mass from its stellar wind.[29]

// For the more massive asymptotic giant branch stars that form planetary nebulae, whose progenitors exceed about 3M⊙, their cores will continue to contract. When temperatures reach about 100 million K, the available helium nuclei fuse into carbon and oxygen, so that the star again resumes radiating energy, temporarily stopping the core's contraction. This new helium burning phase (fusion of helium nuclei) forms a growing inner core of inert carbon and oxygen. Above it is a thin helium-burning shell, surrounded in turn by a hydrogen-burning shell. However, this new phase lasts only 20,000 years or so, a very short period compared to the entire lifetime of the star.

// The venting of atmosphere continues unabated into interstellar space, but when the outer surface of the exposed core reaches temperatures exceeding about 30,000 K, there are enough emitted ultraviolet photons to ionize the ejected atmosphere, causing the gas to shine as a planetary nebula.[28]

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

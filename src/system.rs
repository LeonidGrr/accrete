use crate::consts::PROTOPLANET_MASS;
use crate::consts::*;
use crate::dust::*;
use crate::enviro::*;
use crate::planetismal::*;
use crate::utils::*;
use rand::prelude::*;

#[derive(Debug, Clone)]
pub struct System {
    pub with_moons: bool,
    pub with_rings: bool,
    pub stellar_mass: f64,
    pub stellar_luminosity: f64,
    pub main_seq_life: f64,
    pub age: f64,
    pub ecosphere: (f64, f64),
    pub planets: Vec<Planetismal>,
    pub cloud_eccentricity: f64,
    pub planetismal_inner_bound: f64,
    pub planetismal_outer_bound: f64,
    pub dust_density_coeff: f64,
    pub planets_limit: Option<usize>,
    pub k: f64,
    pub b: f64,
    pub stellar_surface_temp: f64,
    pub stellar_radius: f64,
    // pub spectral_class: SpectralClass,
    // pub color: String,
    // pub surface_temp: f64,
    // pub absolute_magnitude: f64,
}

impl System {
    pub fn set_initial_conditions(
        planets_limit: Option<usize>,
        stellar_mass: f64,
        dust_density_coeff: f64,
        k: f64,
        cloud_eccentricity: f64,
        b: f64,
        _with_moons: bool,
    ) -> Self {
        let stellar_luminosity = luminosity(stellar_mass);
        let planetismal_inner_bound = innermost_planet(stellar_mass);
        let planetismal_outer_bound = outermost_planet(stellar_mass);

        let main_seq_life = main_sequence_age(stellar_mass, stellar_luminosity);

        let mut rng = rand::thread_rng();
        let age = match main_seq_life >= 6.0E9 {
            true => rng.gen_range(1.0E9, 6.0E9),
            false => rng.gen_range(1.0E9, main_seq_life),
        };
        let ecosphere = ecosphere(stellar_luminosity);
        let stellar_radius = stellar_radius(stellar_mass);
        let stellar_surface_temp = stellar_surface_temp(stellar_mass, stellar_radius, stellar_luminosity);

        Self {
            stellar_mass,
            stellar_luminosity,
            main_seq_life,
            age,
            ecosphere,
            with_moons: false,
            with_rings: false,
            planets: Vec::new(),
            k,
            b,
            dust_density_coeff,
            planets_limit,
            cloud_eccentricity,
            planetismal_inner_bound,
            planetismal_outer_bound,
            stellar_surface_temp,
            stellar_radius,
        }
    }

    pub fn distribute_planetary_masses(&mut self) {
        let Self {
            stellar_mass,
            stellar_luminosity,
            planets,
            k,
            b,
            dust_density_coeff,
            planets_limit,
            cloud_eccentricity,
            planetismal_inner_bound,
            planetismal_outer_bound,
            ..
        } = self;
        let inner_dust = 0.0;
        let outer_dust = stellar_dust_limit(&stellar_mass);
        let dust_band = DustBand::new(outer_dust, inner_dust, true, true);
        let mut dust_bands = Vec::new();
        dust_bands.push(dust_band);
        let mut dust_left = true;

        while dust_left {
            let mut p = Planetismal::new(
                &planetismal_inner_bound,
                &planetismal_outer_bound,
                &cloud_eccentricity,
            );

            let inside_range = inner_effect_limit(&p.a, &p.e, &p.mass, &cloud_eccentricity);
            let outside_range = outer_effect_limit(&p.a, &p.e, &p.mass, &cloud_eccentricity);

            if dust_availible(&dust_bands, &inside_range, &outside_range) {
                let dust_density = dust_density(&dust_density_coeff, &stellar_mass, &p.a);
                let crit_mass = critical_limit(&b, &p.a, &p.e, &stellar_luminosity);
                accrete_dust(
                    &mut p,
                    &mut dust_bands,
                    &crit_mass,
                    &dust_density,
                    &cloud_eccentricity,
                    &k,
                );

                let min = inner_effect_limit(&p.a, &p.e, &p.mass, &cloud_eccentricity);
                let max = outer_effect_limit(&p.a, &p.e, &p.mass, &cloud_eccentricity);
                update_dust_lanes(&mut dust_bands, min, max, &p.mass, &crit_mass);
                compress_dust_lanes(&mut dust_bands);

                if p.mass != 0.0 && p.mass != PROTOPLANET_MASS {
                    if p.mass > crit_mass {
                        p.gas_giant = true;
                    }
                    planets.push(p);
                    planets.sort_by(|p1, p2| p1.a.partial_cmp(&p2.a).unwrap());
                    coalesce_planetismals(planets, &cloud_eccentricity);
                } else {
                    // belt?
                    // console.debug(sprintf(".. failed due to large neighbor.\n"));
                }
            }

            let dust_still_left = dust_availible(
                &dust_bands,
                &planetismal_inner_bound,
                &planetismal_outer_bound,
            );

            dust_left = match planets_limit {
                Some(limit) => planets.len() < *limit && dust_still_left,
                None => dust_still_left,
            };
        }
    }

    pub fn generate_planetary_environment(&mut self) {
        for planet in self.planets.iter_mut() {
            planet.orbit_zone = orbital_zone(&self.stellar_luminosity, &planet.a);
            if planet.gas_giant {
                planet.density = empirical_density(
                    &planet.mass,
                    &planet.a,
                    &self.ecosphere.1,
                    &planet.gas_giant,
                );
                planet.radius = volume_radius(&planet.mass, &planet.density);
            } else {
                planet.radius = kothari_radius(&planet.mass, &planet.gas_giant, &planet.orbit_zone);
                planet.density = volume_density(&planet.mass, &planet.radius);
            }

            planet.orbital_period = period(&planet.a, &planet.mass, &self.stellar_mass);
            planet.day = day_length(planet, &self.stellar_mass, &self.main_seq_life);
            planet.axial_tilt = inclination(&planet.a);
            planet.escape_velocity = escape_vel(&planet.mass, &planet.radius);
            planet.surface_accel = acceleration(&planet.mass, &planet.radius);
            planet.rms_velocity = rms_vel(&MOLECULAR_NITROGEN, &planet.a);
            planet.molecule_weight = molecule_limit(&planet.mass, &planet.radius);

            if planet.gas_giant {
                planet.surface_grav = INCREDIBLY_LARGE_NUMBER;
                planet.greenhouse_effect = false;
                planet.volatile_gas_inventory = INCREDIBLY_LARGE_NUMBER;
                planet.surface_pressure = INCREDIBLY_LARGE_NUMBER;
                planet.boil_point = INCREDIBLY_LARGE_NUMBER;
                planet.hydrosphere = INCREDIBLY_LARGE_NUMBER;
                planet.albedo = about(GAS_GIANT_ALBEDO, 0.1);
                planet.surface_temp = INCREDIBLY_LARGE_NUMBER;
            } else {
                planet.surface_grav = gravity(&planet.surface_accel);
                planet.greenhouse_effect = greenhouse(
                    &planet.a,
                    &planet.orbit_zone,
                    &planet.surface_pressure,
                    &self.ecosphere.1,
                );
                planet.volatile_gas_inventory = vol_inventory(
                    &planet.mass,
                    &planet.escape_velocity,
                    &planet.rms_velocity,
                    &self.stellar_mass,
                    &planet.orbit_zone,
                    &planet.greenhouse_effect,
                );
                planet.surface_pressure = pressure(
                    &planet.volatile_gas_inventory,
                    &planet.radius,
                    &planet.surface_grav,
                );
                if planet.surface_pressure == 0.0 {
                    planet.boil_point = 0.0;
                } else {
                    planet.boil_point = boiling_point(&planet.surface_pressure);
                    iterate_surface_temp(planet, &self.ecosphere.1);
                }
            }

            planet.earth_mass = get_earth_mass(planet.mass);
            planet.smallest_molecular_weight = get_smallest_molecular_weight(planet.molecule_weight);
            planet.boiling_point_celsium = planet.boil_point - KELVIN_CELCIUS_DIFFERENCE;
            planet.surface_pressure_bar = planet.surface_pressure / 1000.0;
            planet.surface_temp_celsium = planet.surface_temp - KELVIN_CELCIUS_DIFFERENCE;
            planet.hydrosphere_percentage =  planet.hydrosphere * 100.0;
            planet.cloud_cover_percentage =  planet.cloud_cover * 100.0;
            planet.ice_cover_percentage =  planet.ice_cover * 100.0;
            planet.length_of_year =  planet.orbital_period / 365.25;
            planet.escape_velocity_km_per_sec = planet.escape_velocity / CM_PER_KM;
        }
    }
}

/// Star classifier
/// https://en.wikipedia.org/wiki/Stellar_classification
/// https://www.enchantedlearning.com/subjects/astronomy/stars/startypes.shtml
// [StarType.O, '#9bb0ff'],
//   [StarType.B, '#aabfff'],
//   [StarType.A, '#cad7ff'],
//   [StarType.F, '#f8f7ff'],
//   [StarType.G, '#fff4ea'],
//   [StarType.K, '#ffd2a1'],
//   [StarType.M, '#ffcc6f'],
#[derive(Debug, Clone)]
pub enum SpectralClass {
    // Planets in HZ will be tidally locked very quickly, but about half of all M dwarfs will have a planet in the HZ
    M,
    // Starting to look good. Kepler searches star types K-F.
    K,
    // the sun is a G2
    G, 
    F,
    // Stars age too quickly - only support life for about 2 billion years. Life maybe microbial, but likely no trees.
    A,
    B,
    // Planetary dust disks located within 1.6 light-years of O-type stars are likely to be "boiled off" by superhot radiation and winds(therefore O-type stars likely won't have planets)
    O  
}

/// Star luminosity from mass
pub fn luminosity(mass: f64) -> f64 {
    let n = match mass < 1.0 {
        true => 1.75 * (mass - 0.1) + 3.325,
        false => 0.5 * (2.0 - mass) + 4.4,
    };
    mass.powf(n)
}

/// Star min-max ecosphere
// // "normalized solar flux factor"
// // http://www.solstation.com/habitable.htm
// const SeffInner = new Map<StarType, number>([
//     [StarType.M, 1.05],
//     [StarType.K, 1.05],
//     [StarType.G, 1.41],
//     [StarType.F, 1.9],
//     [StarType.A, 0],
//     [StarType.B, 0],
//     [StarType.O, 0],
//   ]);
  
//   const SeffOuter = new Map<StarType, number>([
//     [StarType.M, 0.27],
//     [StarType.K, 0.27],
//     [StarType.G, 0.36],
//     [StarType.F, 0.46],
//     [StarType.A, 0],
//     [StarType.B, 0],
//     [StarType.O, 0],
//   ]);
  
//   function computeHZBoundary(luminosity: number, seff: number): number {
//     return 1 * Math.pow(luminosity / seff, 0.5);
//   }
  
//   export function computeHabitableZone(t: StarType, luminosity: number): [number, number] {
//     return [
//         computeHZBoundary(luminosity, SeffInner.get(t)!),
//         computeHZBoundary(luminosity, SeffOuter.get(t)!)]
//   }
pub fn ecosphere(luminosity: f64) -> (f64, f64) {
    let min_ecosphere_radius = (luminosity / 1.51).sqrt();
    let max_ecosphere_radius = (luminosity / 0.48).sqrt();
    (min_ecosphere_radius, max_ecosphere_radius)
}

/// Main sequence star age
pub fn main_sequence_age(stellar_mass: f64, stellar_luminosity: f64) -> f64 {
    1.0e10 * stellar_mass / stellar_luminosity
}

/// Empirical star radius from mass (for main sequence only)
pub fn stellar_radius(mass: f64) -> f64 {
    if mass <= 1.66 {
        return 1.06 * mass.powf(0.945) * SOLAR_RADIUS;
    }
    1.33 * mass.powf(0.555) * SOLAR_RADIUS
}

/// Star surface temperature in Kelvin, derived from [Stefan–Boltzmann law](https://en.wikipedia.org/wiki/Stefan%E2%80%93Boltzmann_law)
pub fn stellar_surface_temp(mass: f64, radius: f64, luminosity: f64) -> f64 {
    let luminosity_watt = luminosity * WATT_PER_SOLAR_LUMINOSITY;
    let radius_meters = radius * M_PER_AU;
    let mass_kg = mass * SOLAR_MASS_IN_GRAMS * 1000.0;
    (luminosity_watt / (4.0 * PI * radius_meters.powf(2.0) * SIGMA)).powf(0.25)
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

// /*
//   https://arxiv.org/pdf/1511.07438.pdf
  
//   According to this paper, metallicity distribution is best represented
//   by a combination of two Gaussians.
//   Units are in [Fe/H], which you should google. It's a measure of the
//   presence of iron vs the solar system on a logarithmic scale.
// */
// export function computeMetallicityValue(aRandomNumber: number, n2: number): number {
//     const dist1 = gaussian(0.3, 0.1);
//     const dist2 = gaussian(-0.45, 0.1);
//     const val1 = dist1.ppf(aRandomNumber);
//     const val2 = dist2.ppf(aRandomNumber);
//     // According to stats.stackexchange.com there's a super mathy way to
//     // combine two Gaussian distributions, but using a weighted choice
//     // seems to produce similar results, so whatever.
//     return weightedChoice([[val1, 1.5], [val2, 0.5]], n2);
//   }
/*
        http://iopscience.iop.org/article/10.1086/428383/pdf
        https://arxiv.org/pdf/1511.07438.pdf
        "One-quarter of the FGK-type stars with [Fe/H] > 0.3 dex harbor
        Jupiter-like planets with orbital periods shorter than 4 yr. In
        contrast, gas giant planets are detected around fewer than 3% of
        the stars with subsolar metallicity. "
        So if stars have a 70% chance of having any planets, and a 25%
        chance of specifically having a gas giant, we want about a 35%
        chance of a planet being a gas giant.
    */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_sun_temperature() {
        let sun_radius = stellar_radius(1.0);
        let sun_temp = stellar_surface_temp(1.0, sun_radius, 1.0);
        assert_eq!(5606, sun_temp as usize);
    }
}
use crate::consts::PROTOPLANET_MASS;
use crate::consts::*;
use crate::dust::*;
use crate::enviro::*;
use crate::planetismal::*;
use crate::utils::*;
use rand::prelude::*;

/// [Star classifier by Harvard system](https://en.wikipedia.org/wiki/Stellar_classification)
/// [Additional info](https://www.enchantedlearning.com/subjects/astronomy/stars/startypes.shtml)
#[derive(Debug, Clone)]
pub enum SpectralClass {
    ROGUE,
    Y,
    T,
    L,
    M,
    K,
    G,
    F,
    A,
    B,
    O,
}

#[derive(Debug, Clone)]
pub struct PrimaryStar {
    pub stellar_mass: f64,
    pub stellar_luminosity: f64,
    pub stellar_surface_temp: f64,
    pub stellar_radius_au: f64,
    pub spectral_class: SpectralClass,
    pub bv_color_index: f64,
    pub color: String,
    pub main_seq_life: f64,
    // pub age: f64,
    pub ecosphere: (f64, f64),
    pub planets: Vec<Planetismal>,
    pub cloud_eccentricity: f64,
    pub planetismal_inner_bound: f64,
    pub planetismal_outer_bound: f64,
    pub dust_density_coeff: f64,
    pub planets_limit: Option<usize>,
    pub k: f64,
    pub b: f64,
    has_nebulae: bool,
}

impl PrimaryStar {
    pub fn set_initial_conditions(
        planets_limit: Option<usize>,
        stellar_mass: f64,
        dust_density_coeff: f64,
        k: f64,
        cloud_eccentricity: f64,
        b: f64,
    ) -> Self {
        let stellar_luminosity = luminosity(stellar_mass);
        let planetismal_inner_bound = innermost_planet(stellar_mass);
        let planetismal_outer_bound = outermost_planet(stellar_mass);
        let main_seq_life = main_sequence_age(stellar_mass, stellar_luminosity);

        let stellar_radius_au = stellar_radius_au(stellar_mass);
        let stellar_surface_temp = stellar_surface_temp(stellar_radius_au, stellar_luminosity);
        let spectral_class = spectral_class(&stellar_surface_temp);

        let bv_color_index = bv_color_index(stellar_surface_temp);
        let color = bv_to_rgb(bv_color_index);

        let ecosphere = ecosphere(&stellar_luminosity, &spectral_class);

        Self {
            stellar_mass,
            stellar_luminosity,
            main_seq_life,
            // age,
            ecosphere,
            planets: Vec::new(),
            k,
            b,
            dust_density_coeff,
            planets_limit,
            cloud_eccentricity,
            planetismal_inner_bound,
            planetismal_outer_bound,
            stellar_surface_temp,
            stellar_radius_au,
            spectral_class,
            bv_color_index,
            color,
            has_nebulae: false,
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

            if dust_availible(
                &dust_bands,
                &inside_range,
                &outside_range,
            ) {
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
                    p.mass_with_moons = p.mass;
                    planets.push(p);
                    planets.sort_by(|p1, p2| p1.a.partial_cmp(&p2.a).unwrap());
                    coalesce_planetismals(stellar_luminosity, planets, &cloud_eccentricity);
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

    pub fn process_all_planets(&mut self) {
        let PrimaryStar {
            stellar_luminosity,
            stellar_mass,
            main_seq_life,
            ecosphere,
            planets,
            ..
        } = self;
        for planet in planets.iter_mut() {
            planet.derive_planetary_environment(
                stellar_luminosity,
                stellar_mass,
                main_seq_life,
                ecosphere,
            );
        }
    }
}

/// Approximated luminosity from mass
fn luminosity(mass: f64) -> f64 {
    mass.powf(3.5)
}

/// Star min-max habitable zone
/// [Normalized solar flux factor](http://www.solstation.com/habitable.html)
/// [Red dwarfs habitable zone 1](https://en.wikipedia.org/wiki/Habitability_of_red_dwarf_systems)
/// [Planetary_habitability](https://en.wikipedia.org/wiki/Planetary_habitability)
fn ecosphere(luminosity: &f64, spectral_class: &SpectralClass) -> (f64, f64) {
    let (outer_normalized_flux_factor, inner_normalized_flux_factor) = match spectral_class {
        // BrownDwarfs. For Y, L , T approzimation is used
        SpectralClass::ROGUE => (0.0, 0.0),
        SpectralClass::Y => (0.0, 0.0),
        SpectralClass::T => (0.05, 0.2),
        SpectralClass::L => (0.16, 0.7),
        // Main seq sun-like classes
        SpectralClass::M => (0.27, 1.05),
        SpectralClass::K => (0.27, 1.05),
        // Original values
        // SpectralClass::G => (0.48, 1.51),
        SpectralClass::G => (0.36, 1.41),
        SpectralClass::F => (0.46, 1.9),
        SpectralClass::A => (0.0, 0.0),
        SpectralClass::B => (0.0, 0.0),
        SpectralClass::O => (0.0, 0.0),
    };
    let min_ecosphere_radius = (luminosity / inner_normalized_flux_factor).sqrt();
    let max_ecosphere_radius = (luminosity / outer_normalized_flux_factor).sqrt();
    (min_ecosphere_radius, max_ecosphere_radius)
}

/// Main sequence star age
fn main_sequence_age(stellar_mass: f64, stellar_luminosity: f64) -> f64 {
    1.0e10 * stellar_mass / stellar_luminosity
}

/// Empirical star radius from mass (for main sequence only)
fn stellar_radius_au(mass: f64) -> f64 {
    if mass <= 1.66 {
        return 1.06 * mass.powf(0.945) * SOLAR_RADIUS;
    }
    1.33 * mass.powf(0.555) * SOLAR_RADIUS
}

/// Star surface temperature in Kelvin, derived from [Stefan–Boltzmann law](https://en.wikipedia.org/wiki/Stefan%E2%80%93Boltzmann_law)
fn stellar_surface_temp(radius: f64, luminosity: f64) -> f64 {
    let luminosity_watt = luminosity * WATT_PER_SOLAR_LUMINOSITY;
    let radius_meters = radius * M_PER_AU;
    (luminosity_watt / (4.0 * PI * radius_meters.powf(2.0) * SIGMA)).powf(0.25)
}

/// Star B-V color index
fn bv_color_index(stellar_surface_temp: f64) -> f64 {
    (5601.0 / stellar_surface_temp).powf(1.5) - 0.4
}

/// Star RGB color from color index
/// [Reference table](http://www.vendian.org/mncharity/dir3/starcolor/details.html)
/// [StackOverflow](https://stackoverflow.com/questions/21977786/star-b-v-color-index-to-apparent-rgb-color)
fn bv_to_rgb(bv: f64) -> String {
    let mut r = 0.0;
    let mut g = 0.0;
    let mut b = 0.0;
    let mut bv = bv;
    let mut t;

    if bv < -0.4 {
        bv = -0.4;
    }

    if bv > 2.0 {
        bv = 2.0;
    }

    if bv >= -0.40 && bv < 0.00 {
        t = (bv + 0.40) / 0.40;
        r = 0.61 + (0.11 * t) + (0.1 * t * t);
    } else if bv >= 0.00 && bv < 0.40 {
        t = bv / 0.40;
        r = 0.83 + (0.17 * t);
    } else if bv >= 0.40 && bv < 2.10 {
        r = 1.00;
    }

    if bv >= -0.40 && bv < 0.00 {
        t = (bv + 0.40) / 0.40;
        g = 0.70 + (0.07 * t) + (0.1 * t * t);
    } else if bv >= 0.00 && bv < 0.40 {
        t = bv / 0.40;
        g = 0.87 + (0.11 * t);
    } else if bv >= 0.40 && bv < 1.60 {
        t = (bv - 0.40) / (1.60 - 0.40);
        g = 0.98 - (0.16 * t);
    } else if bv >= 1.60 && bv < 2.00 {
        t = (bv - 1.60) / (2.00 - 1.60);
        g = 0.82 - (0.5 * t * t);
    }

    if bv >= -0.40 && bv < 0.40 {
        b = 1.00;
    } else if bv >= 0.40 && bv < 1.50 {
        t = (bv - 0.40) / (1.50 - 0.40);
        b = 1.00 - (0.47 * t) + (0.1 * t * t);
    } else if bv >= 1.50 && bv < 1.94 {
        t = (bv - 1.50) / (1.94 - 1.50);
        b = 0.63 - (0.6 * t * t);
    }

    let mut hex = vec![
        format!("{:x}", (r * 255.0) as u16),
        format!("{:x}", (g * 255.0) as u16),
        format!("{:x}", (b * 255.0) as u16),
    ];

    for h in hex.iter_mut() {
        if h.len() < 2 {
            h.insert(0, '0');
        }
    }

    format!("#{}{}{}", hex[0], hex[1], hex[2])
}

/// Spectral class from temperature
fn spectral_class(stellar_surface_temp: &f64) -> SpectralClass {
    match *stellar_surface_temp {
        t if t >= 30000.0 => SpectralClass::O,
        t if t >= 10000.0 && t < 30000.0 => SpectralClass::B,
        t if t >= 7500.0 && t < 10000.0 => SpectralClass::A,
        t if t >= 6000.0 && t < 7500.0 => SpectralClass::F,
        t if t >= 5200.0 && t < 6000.0 => SpectralClass::G,
        t if t >= 3700.0 && t < 5200.0 => SpectralClass::K,
        t if t >= 2400.0 && t < 3700.0 => SpectralClass::M,
        t if t >= 1300.0 && t < 2400.0 => SpectralClass::L,
        t if t >= 550.0 && t < 1300.0 => SpectralClass::T,
        t if t >= 273.15 && t < 550.0 => SpectralClass::Y,
        _ => SpectralClass::ROGUE,
    }
}

/// Orbital radius is in AU, eccentricity is unitless, and the stellar luminosity ratio is with respect to the sun.
/// The value returned is the mass at which the planet begins to accrete gas as well as dust, and is in units of solar masses.
fn critical_limit(
    b: &f64,
    orbital_radius: &f64,
    eccentricity: &f64,
    stellar_luminosity_ratio: &f64,
) -> f64 {
    let perihelion_dist = orbital_radius - orbital_radius * eccentricity;
    let temp = perihelion_dist * stellar_luminosity_ratio.sqrt();
    b * temp.powf(-0.75)
}

/// Check planetismal coalescence
pub fn coalesce_planetismals(primary_star_luminosity: &f64, planets: &mut Vec<Planetismal>, cloud_eccentricity: &f64) {
    let mut next_planets = Vec::new();
    for (i, p) in planets.iter().enumerate() {
        if i == 0 {
            next_planets.push(p.clone());
        } else {
            if let Some(prev_p) = next_planets.last_mut() {
                let dist = prev_p.a - p.a;
                let (dist1, dist2) = match dist > 0.0 {
                    true => {
                        let dist1 =
                            outer_effect_limit(&p.a, &p.e, &p.mass_with_moons, cloud_eccentricity) - p.a;
                        let dist2 = prev_p.a
                            - inner_effect_limit(
                                &prev_p.a,
                                &prev_p.e,
                                &prev_p.mass_with_moons,
                                cloud_eccentricity,
                            );
                        (dist1, dist2)
                    }
                    false => {
                        let dist1 =
                            p.a - inner_effect_limit(&p.a, &p.e, &p.mass_with_moons, cloud_eccentricity);
                        let dist2 = outer_effect_limit(
                            &prev_p.a,
                            &prev_p.e,
                            &prev_p.mass_with_moons,
                            cloud_eccentricity,
                        ) - prev_p.a;
                        (dist1, dist2)
                    }
                };

                // Check for larger/smaller planetismal
                let (mut larger, mut smaller) = match p.mass >= prev_p.mass {
                    true => (p.clone(), prev_p.clone()),
                    false => (prev_p.clone(), p.clone()),
                };
                
                // Recalculate current radius of
                larger.orbit_zone = orbital_zone(primary_star_luminosity, larger.distance_to_primary_star);
                larger.radius = kothari_radius(&larger.mass, &larger.gas_giant, &larger.orbit_zone);
                smaller.orbit_zone = orbital_zone(primary_star_luminosity, smaller.distance_to_primary_star);
                smaller.radius = kothari_radius(&smaller.mass, &smaller.gas_giant, &prev_p.orbit_zone);

                // Check if planetismals whithin effective zone of each other
                if dist.abs() < dist1.abs() || dist.abs() < dist2.abs() {
                    if dist.abs() < (larger.radius + smaller.radius) / KM_PER_AU {
                        *prev_p = coalesce_two_planets(&prev_p, &p);
                    } else {
                        *prev_p = capture_moon(&larger, &smaller);
                    }
                } else {
                    next_planets.push(p.clone());
                }
            }
        }
    }
    *planets = next_planets;
}

/// Two planetismals collide and form one planet
pub fn coalesce_two_planets(a: &Planetismal, b: &Planetismal) -> Planetismal {
    let new_mass = a.mass + b.mass;
    let new_axis = new_mass / (a.mass / a.a + b.mass / b.a);
    let term1 = a.mass * (a.a * (1.0 - a.e.powf(2.0))).sqrt();
    let term2 = b.mass * (b.a * (1.0 - b.e.powf(2.0))).sqrt();
    let term3 = (term1 + term2) / (new_mass * new_axis.sqrt());
    let term4 = 1.0 - term3.powf(2.0);
    let new_eccn = term4.abs().sqrt();
    let mut coalesced = a.clone();
    coalesced.mass = new_mass;
    coalesced.a = new_axis;
    coalesced.e = new_eccn;
    coalesced.gas_giant = a.gas_giant || b.gas_giant;
    coalesced
}

/// Larger planetsimal capture smaller as moon
pub fn capture_moon(larger: &Planetismal, smaller: &Planetismal) -> Planetismal {
    let mut planet = larger.clone();
    let mut moon = smaller.clone();
    moon.is_moon = true;

    // Recalcualte combined mass of planet-moons system and planetary axis
    let new_axis = planet.mass_with_moons / (planet.mass / planet.a + moon.mass / moon.a);
    let term1 = planet.mass * (planet.a * (1.0 - planet.e.powf(2.0))).sqrt();
    let term2 = moon.mass * (moon.a * (1.0 - moon.e.powf(2.0))).sqrt();
    let term3 = (term1 + term2) / (planet.mass_with_moons * new_axis.sqrt());
    let term4 = 1.0 - term3.powf(2.0);
    let new_eccn = term4.abs().sqrt();
    planet.a = new_axis;
    planet.e = new_eccn;
    planet.distance_to_primary_star = new_axis;

    // Add moon to planetary moons, recalculate disturbed orbits of moons
    let mut rng = rand::thread_rng();
    planet.moons.append(&mut moon.moons);
    planet.moons.push(moon);
    planet.mass_with_moons = planet.mass;

    for m in planet.moons.iter_mut() {
        let hill_sphere = hill_sphere_au(
            &planet.a,
            &planet.e,
            &planet.mass,
            &m.mass,
        );
        
        // Check roche limit - if moon turns into ring

        m.a = rng.gen_range(0.0, hill_sphere);
        m.distance_to_primary_star = planet.a;
        planet.mass_with_moons += m.mass;
    }

    // Check collisions between moons

    planet
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

// rom observation," Braben said, "we know the temperature of some stars. We know the size of that star, and we know something called its metallicity," or the types of periodic elements that make up the star’s composition. "Some of the older stars actually have very low metallicity, and we factor that into the elements that are there in that specific star system."

/// hasNebulae
/// Stars greater than 8 solar masses (M⊙) will likely end their lives in dramatic supernovae explosions, while planetary nebulae seemingly only occur at the end of the lives of intermediate and low mass stars between 0.8 M⊙ to 8.0 M⊙.[26] Progenitor stars that form planetary nebulae will spend most of their lifetimes converting their hydrogen into helium in the star's core by nuclear fusion at about 15 million K. This generated energy creates outward pressure from fusion reactions in the core, balancing the crushing inward pressures of the star's gravity.[27] This state of equilibrium is known as the main sequence, which can last for tens of millions to billions of years, depending on the mass.

// When the hydrogen source in the core starts to diminish, gravity starts compressing the core, causing a rise in temperature to about 100 million K.[28] Such higher core temperatures then make the star's cooler outer layers expand to create much larger red giant stars. This end phase causes a dramatic rise in stellar luminosity, where the released energy is distributed over a much larger surface area, which in fact causes the average surface temperature to be lower. In stellar evolution terms, stars undergoing such increases in luminosity are known as asymptotic giant branch stars (AGB).[28] During this phase, the star can lose 50 to 70% of its total mass from its stellar wind.[29]

// For the more massive asymptotic giant branch stars that form planetary nebulae, whose progenitors exceed about 3M⊙, their cores will continue to contract. When temperatures reach about 100 million K, the available helium nuclei fuse into carbon and oxygen, so that the star again resumes radiating energy, temporarily stopping the core's contraction. This new helium burning phase (fusion of helium nuclei) forms a growing inner core of inert carbon and oxygen. Above it is a thin helium-burning shell, surrounded in turn by a hydrogen-burning shell. However, this new phase lasts only 20,000 years or so, a very short period compared to the entire lifetime of the star.

// The venting of atmosphere continues unabated into interstellar space, but when the outer surface of the exposed core reaches temperatures exceeding about 30,000 K, there are enough emitted ultraviolet photons to ionize the ejected atmosphere, causing the gas to shine as a planetary nebula.[28]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_sun_temperature() {
        let sun_radius = stellar_radius_au(1.0);
        let sun_temp = stellar_surface_temp(sun_radius, 1.0);
        assert_eq!(5606, sun_temp as usize);
    }

    #[test]
    fn check_sun_bv() {
        let bv_sun = bv_color_index(5601.0);
        assert_eq!(0.6, bv_sun);
    }

    #[test]
    fn check_sun_color() {
        let color_sun = bv_to_rgb(0.6);
        assert_eq!("#fff3ea", color_sun);
    }
}

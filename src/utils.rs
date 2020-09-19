use crate::consts::*;
use rand::prelude::*;

pub fn about(value: f64, variation: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(value - variation, value + variation)
}

/// "...the semimajor axes of planetary nuclei can never be greater than 50 distance units, which effectively sets an outer boundary to the problem. An inner boundary was also established, arbitrarily at 0.3 distance unit. (More than 92 percent of the total cloud mass lies between these bounds.)"
pub fn outermost_planet(stellar_mass_ratio: &f64) -> f64 {
    50.0 * stellar_mass_ratio.powf(0.33)
}

pub fn innermost_planet(stellar_mass_ratio: &f64) -> f64 {
    0.3 * stellar_mass_ratio.powf(0.33)
}

pub fn inner_effect_limit(a: &f64, e: &f64, mass: &f64, cloud_eccentricity: &f64) -> f64 {
    perihelion_distance(a, e) * (1.0 - mass) / (1.0 + cloud_eccentricity)
}

pub fn outer_effect_limit(a: &f64, e: &f64, mass: &f64, cloud_eccentricity: &f64) -> f64 {
    aphelion_distance(a, e) * (1.0 + mass) / (1.0 - cloud_eccentricity)
}

/// The distance between the orbiting body and the sun at it's closest approach.
pub fn perihelion_distance(radius: &f64, eccentricity: &f64) -> f64 {
    radius * (1.0 - eccentricity)
}

// The distance between the orbiting body and the sun at it's furthest approach.
pub fn aphelion_distance(radius: &f64, eccentricity: &f64) -> f64 {
    radius * (1.0 + eccentricity)
}

pub fn random_eccentricity(random: f64, cloud_eccentricity: &f64) -> f64 {
    1.0 - random.powf(*cloud_eccentricity)
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

/// Approximated luminosity from mass
pub fn luminosity(mass: f64) -> f64 {
    mass.powf(3.5)
}

/// Star min-max habitable zone
/// [Normalized solar flux factor](http://www.solstation.com/habitable.html)
/// [Red dwarfs habitable zone 1](https://en.wikipedia.org/wiki/Habitability_of_red_dwarf_systems)
/// [Planetary_habitability](https://en.wikipedia.org/wiki/Planetary_habitability)
pub fn ecosphere(luminosity: &f64, spectral_class: &SpectralClass) -> (f64, f64) {
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
pub fn main_sequence_age(stellar_mass: f64, stellar_luminosity: f64) -> f64 {
    1.0e10 * stellar_mass / stellar_luminosity
}

/// Empirical star radius from mass (for main sequence only)
pub fn stellar_radius_au(mass: f64) -> f64 {
    if mass <= 1.66 {
        return 1.06 * mass.powf(0.945) * SOLAR_RADIUS;
    }
    1.33 * mass.powf(0.555) * SOLAR_RADIUS
}

/// Star surface temperature in Kelvin, derived from [Stefan–Boltzmann law](https://en.wikipedia.org/wiki/Stefan%E2%80%93Boltzmann_law)
pub fn stellar_surface_temp(radius: f64, luminosity: f64) -> f64 {
    let luminosity_watt = luminosity * WATT_PER_SOLAR_LUMINOSITY;
    let radius_meters = radius * M_PER_AU;
    (luminosity_watt / (4.0 * PI * radius_meters.powf(2.0) * SIGMA)).powf(0.25)
}

/// Star B-V color index
pub fn bv_color_index(stellar_surface_temp: f64) -> f64 {
    (5601.0 / stellar_surface_temp).powf(1.5) - 0.4
}

/// Star RGB color from color index
/// [Reference table](http://www.vendian.org/mncharity/dir3/starcolor/details.html)
/// [StackOverflow](https://stackoverflow.com/questions/21977786/star-b-v-color-index-to-apparent-rgb-color)
pub fn bv_to_rgb(bv: f64) -> String {
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
pub fn spectral_class(stellar_surface_temp: &f64) -> SpectralClass {
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

    #[test]
    fn check_earth_moon_roche_limit() {
        let earth_mass = 2.988000001494E-6;
        let moon_mass = earth_mass * 0.012;
        let moon_radius =  1737.5;
        let earth_moon_roche_limit = roche_limit_au(&earth_mass, &moon_mass, &moon_radius) * KM_PER_AU;
        assert!(earth_moon_roche_limit > 9400.0 && earth_moon_roche_limit < 9600.0);
    }
}

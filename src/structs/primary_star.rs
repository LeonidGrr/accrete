use crate::consts::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PrimaryStar {
    pub stellar_mass: f64,
    pub stellar_luminosity: f64,
    pub stellar_surface_temp: f64,
    pub stellar_radius_au: f64,
    pub spectral_class: SpectralClass,
    pub bv_color_index: f64,
    pub color: [f64; 3],
    pub main_seq_age: f64,
    pub ecosphere: (f64, f64),
}

impl PrimaryStar {
    pub fn new(stellar_mass: f64) -> Self {
        // Warning - stallar mass higher than 10 solar masses probably refer to non-regular and supermassive stars (short lived giants, black holes, wolf-rayet stars, etc..) and require other calculation methods
        // Possibly can be useful https://www.fabiopacucci.com/resources/black-hole-calculator/formulas-black-hole-calculator/
        let stellar_luminosity = luminosity(stellar_mass);
        let main_seq_age = main_sequence_age(stellar_mass, stellar_luminosity);
        let stellar_radius_au = stellar_radius_au(stellar_mass);
        let stellar_surface_temp = stellar_surface_temp(stellar_radius_au, stellar_luminosity);
        let spectral_class = spectral_class(&stellar_surface_temp);
        let bv_color_index = bv_color_index(stellar_surface_temp);
        let color = bv_to_rgb(bv_color_index);
        let ecosphere = ecosphere(&stellar_luminosity, &spectral_class);

        Self {
            stellar_mass,
            stellar_luminosity,
            main_seq_age,
            ecosphere,
            stellar_surface_temp,
            stellar_radius_au,
            spectral_class,
            bv_color_index,
            color,
        }
    }
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
        SpectralClass::Rogue => (0.0, 0.0),
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
pub fn bv_to_rgb(bv: f64) -> [f64; 3] {
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

    if (-0.40..0.00).contains(&bv) {
        t = (bv + 0.40) / 0.40;
        r = 0.61 + (0.11 * t) + (0.1 * t * t);
    } else if (0.00..0.40).contains(&bv) {
        t = bv / 0.40;
        r = 0.83 + (0.17 * t);
    } else if (0.40..2.10).contains(&bv) {
        r = 1.00;
    }

    if (-0.40..0.00).contains(&bv) {
        t = (bv + 0.40) / 0.40;
        g = 0.70 + (0.07 * t) + (0.1 * t * t);
    } else if (0.00..0.40).contains(&bv) {
        t = bv / 0.40;
        g = 0.87 + (0.11 * t);
    } else if (0.40..1.60).contains(&bv) {
        t = (bv - 0.40) / (1.60 - 0.40);
        g = 0.98 - (0.16 * t);
    } else if (1.60..2.00).contains(&bv) {
        t = (bv - 1.60) / (2.00 - 1.60);
        g = 0.82 - (0.5 * t * t);
    }

    if (-0.40..0.40).contains(&bv) {
        b = 1.00;
    } else if (0.40..1.50).contains(&bv) {
        t = (bv - 0.40) / (1.50 - 0.40);
        b = 1.00 - (0.47 * t) + (0.1 * t * t);
    } else if (1.50..1.94).contains(&bv) {
        t = (bv - 1.50) / (1.94 - 1.50);
        b = 0.63 - (0.6 * t * t);
    }

    [r, g, b]
}

/// Spectral class from temperature
pub fn spectral_class(stellar_surface_temp: &f64) -> SpectralClass {
    match *stellar_surface_temp {
        t if t >= 30000.0 => SpectralClass::O,
        t if (10000.0..30000.0).contains(&t) => SpectralClass::B,
        t if (7500.0..10000.0).contains(&t) => SpectralClass::A,
        t if (6000.0..7500.0).contains(&t) => SpectralClass::F,
        t if (5200.0..6000.0).contains(&t) => SpectralClass::G,
        t if (3700.0..5200.0).contains(&t) => SpectralClass::K,
        t if (2400.0..3700.0).contains(&t) => SpectralClass::M,
        t if (1300.0..2400.0).contains(&t) => SpectralClass::L,
        t if (550.0..1300.0).contains(&t) => SpectralClass::T,
        t if (273.15..550.0).contains(&t) => SpectralClass::Y,
        _ => SpectralClass::Rogue,
    }
}

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

    // #[test]
    // fn check_sun_color() {
    //     let color_sun = bv_to_rgb(0.6);
    //     assert_eq!("#fff3ea", color_sun);
    // }
}

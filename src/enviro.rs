use crate::consts::*;
use crate::utils::*;
use crate::planetismal::Planetismal;

pub fn luminosity(mass: &f64) -> f64 {
    let n = match *mass < 1.0 {
        true => 1.75 * (mass - 0.1) + 3.325,
        false => 0.5 * (2.0 - mass) + 4.4,
    };
    mass.powf(n)
}

pub fn ecosphere(luminosity: &f64) -> (f64, f64) {
    let min_ecosphere_radius = (luminosity / 1.51).sqrt();
    let max_ecosphere_radius = (luminosity / 0.48).sqrt();
    (min_ecosphere_radius, max_ecosphere_radius)
}

pub fn main_sequence_age(stellar_mass: &f64, stellar_luminosity: &f64) -> f64 {
    1.0e10 * stellar_mass / stellar_luminosity
}

/// This function, given the orbital radius of a planet in AU, returns the orbital 'zone' of the particle.
pub fn orbital_zone(luminosity: &f64, orb_radius: &f64) -> i32 {
    if orb_radius < &(4.0 * luminosity.sqrt()) {
        return 1;
    } else if orb_radius < &(15.0 * luminosity.sqrt()) {
        return 2;
    }
    3
}

/// The mass is in units of solar masses, and the density is in units of grams/cc. The radius returned is in units of km.
pub fn volume_radius(mass: &f64, density: &f64) -> f64 {
    let mut volume = 0.0;
    volume = mass * SOLAR_MASS_IN_GRAMS / density;
    ((3.0 * volume) / (4.0 * PI)).powf(0.33) / CM_PER_KM
}

/// Returns the radius of the planet in kilometers.
/// The mass passed in is in units of solar masses, the orbital radius in A.U.
/// This formula is listed as eq.9 in Fogg's article, although some typos crop up in that eq. See "The Internal Constitution of Planets", by Dr. D. S. Kothari, Mon. Not. of the Royal Astronomical Society, vol 96 pp.833-843, 1936 for the derivation. Specifically, this is Kothari's eq.23, which appears on page 840.
pub fn kothari_radius(mass: &f64, giant: bool, zone: i32) -> f64 {
    let mut atomic_weight = 0.0;
    let mut atomic_num = 0.0;
    let mut temp = 0.0;
    let mut temp1 = 0.0;
    let mut temp2 = 0.0;

    match (zone, giant) {
        (1, true) => {
            atomic_weight = 9.5;
            atomic_num = 4.5;
        }
        (1, false) => {
            atomic_weight = 15.0;
            atomic_num = 8.0;
        }
        (2, true) => {
            atomic_weight = 2.47;
            atomic_num = 2.0;
        }
        (2, false) => {
            atomic_weight = 10.0;
            atomic_num = 5.0;
        }
        (3, true) => {
            atomic_weight = 7.0;
            atomic_num = 4.0;
        }
        (3, false) => {
            atomic_weight = 10.0;
            atomic_num = 5.0;
        }
        (_, _) => (),
    }

    temp = atomic_weight * atomic_num;
    temp = 2.0 * BETA_20 * SOLAR_MASS_IN_GRAMS.powf(0.3) / A1_20 * (temp as f64).powf(0.3);

    temp2 = A2_20 * atomic_weight.powf(1.3) * SOLAR_MASS_IN_GRAMS.powf(0.6);
    temp2 = temp2 * mass.powf(0.6);
    temp2 = temp2 / (A1_20 * atomic_num.powf(2.0));
    temp2 += 1.0;

    temp = temp / temp2;
    temp = temp * mass.powf(0.3) / CM_PER_KM;
    temp /= JIMS_FUDGE;

    temp
}

/// The mass passed in is in units of solar masses, and the orbital radius is in units of AU. The density is returned in units of grams/cc.
pub fn empirical_density(
    mass: &f64,
    orb_radius: &f64,
    ecosphere_radius: &f64,
    gas_giant: bool,
) -> f64 {
    let mut density = (mass * EARTH_MASSES_PER_SOLAR_MASS).powf(1.0 / 8.0);
    density = density * (ecosphere_radius / orb_radius).powf(0.25);

    match gas_giant {
        true => density * 1.2,
        false => density * 5.5,
    }
}

/// The mass passed in is in units of solar masses, and the equatorial radius is in km. The density is returned in units of grams/cc.
pub fn volume_density(mass: &f64, equat_radius: &mut f64) -> f64 {
    *equat_radius *= CM_PER_KM;
    let volume = (4.0 * PI * equat_radius.powf(3.0)) / 3.0;
    mass * SOLAR_MASS_IN_GRAMS / volume
}

/// Separation - Units of AU between the masses returns the period of an entire xorbit in Earth days.
pub fn period(separation: &f64, small_mass: &f64, large_mass: &f64) -> f64 {
    let period_in_years = (separation.powf(3.0) / (small_mass + large_mass)).sqrt();
    // period_in_years * planet.days_in_year
    period_in_years * DAYS_IN_A_YEAR
}

// /*--------------------------------------------------------------------------*/
// /* Fogg's information for this routine came from Dole "Habitable Planets */
// /* for Man", Blaisdell Publishing Company, NY, 1964. From this, he came */
// /* up with his eq.12, which is the equation for the base_angular_velocity */
// /* below. Going a bit further, he found an equation for the change in */
// /* angular velocity per time (dw/dt) from P. Goldreich and S. Soter's paper */
// /* "Q in the Solar System" in Icarus, vol 5, pp.375-389 (1966). Comparing */
// /* to the change in angular velocity for the Earth, we can come up with an */
// /* approximation for our new planet (his eq.13) and take that into account. */
// /*--------------------------------------------------------------------------*/
// // double
// function day_length(mass, radius, orbital_period, eccentricity, giant) {
//     var base_angular_velocity, planetary_mass_in_grams, k2, temp, equatorial_radius_in_cm, change_in_angular_velocity, spin_resonance_period;

//     spin_resonance = FALSE;
//     if (giant)
// 	k2 = 0.24;
//     else
// 	k2 = 0.33;
//     planetary_mass_in_grams = mass * SOLAR_MASS_IN_GRAMS;
//     equatorial_radius_in_cm = radius * CM_PER_KM;
//     base_angular_velocity = Math.sqrt(2.0 * J * (planetary_mass_in_grams) / (k2 * Math.pow(equatorial_radius_in_cm, 2.0)));
//     /* This next term describes how much a planet's rotation is slowed by */
//     /* it's moons. Find out what dw/dt is after figuring out Goldreich and */
//     /* Soter's Q'. */
//     change_in_angular_velocity = 0.0;
//     temp = base_angular_velocity + (change_in_angular_velocity * age);
//     /* 'temp' is now the angular velocity. Now we change from rad/sec to */
//     /* hours/rotation. */
//     temp = 1.0 / ((temp / radians_per_rotation) * SECONDS_PER_HOUR);
//     if (temp >= orbital_period) {
// 	spin_resonance_period = ((1.0 - eccentricity) / (1.0 + eccentricity)) * orbital_period;
// 	printf("...maybe: %f\n", spin_resonance_period);

// 	if (eccentricity > 0.01) {
// 	    printf("...resonance...\n");
// 	    temp = spin_resonance_period;
// 	    spin_resonance = TRUE;
// 	} else
// 	    temp = orbital_period;
//     }
//     return (temp);
// }
/// Fogg's information for this routine came from Dole "Habitable Planets for Man", Blaisdell Publishing Company, NY, 1964. From this, he came up with his eq.12, which is the equation for the base_angular_velocity below.
/// Going a bit further, he found an equation for the change in angular velocity per time (dw/dt) from P. Goldreich and S. Soter's paper "Q in the Solar System" in Icarus, vol 5, pp.375-389 (1966).
/// Comparing to the change in angular velocity for the Earth, we can come up with an approximation for our new planet (his eq.13) and take that into account.
// pub fn day_length(planet: &mut Planetismal, stellar_mass: &f64, main_sequence_age: &f64) -> f64 {
//     let planet_mass_in_grams = planet.mass * self.solar_mass_in_grams,
//     equatorial_radius_in_cm = planet.radius * CM_PER_KM,
//     year_in_hours = planet.orbPeriod || self.period(planet.axis, planet.mass, 1),
//     let giant = planet.giant || false;
//     let k2 = 0.0;
//     base_angular_velocity = 0.0;
//     change_in_angular_velocity = 0.0;
//     ang_velocity = 0.0;
//     spin_resonance_factor = 0.0;
//     day_in_hours = 0.0;
//     stopped = false;

//     planet.resonant_period = false;

//     if (giant) {
//         k2 = 0.24;
//     }
//     else {
//         k2 = 0.33;
//     }

//     base_angular_velocity = Math.sqrt(2 * J * planet_mass_in_grams) /
//     (k2 * Math.pow(equatorial_radius_in_cm, 2));

//     change_in_angular_velocity = self.change_in_earth_ang_vel *
//         (planet.density / EARTH_DENSITY) *
//         (equatorial_radius_in_cm / EARTH_RADIUS_IN_CM) *
//         (EARTH_MASS_IN_GRAMS / planet_mass_in_grams) *
//         Math.pow(stellar_masss, 2) *
//         (1 / Math.pow(planet.axis, 6));

//     ang_velocity = base_angular_velocity + (change_in_angular_velocity * main_sequence_age);

//     if (ang_velocity <= 0.0) {
//         stopped = true;
//         day_in_hours = self.very_large_number;
//     }
//     else {
//         day_in_hours = self.radians_per_rotation / (seconds_per_hour * ang_velocity);
//     }

//     if (day_in_hours >= year_in_hours || stopped) {
//         if (planet.eccn > 0.1) {
//         spin_resonance_factor = (1 - planet.eccn) / (1 + planet.eccn);
//         planet.resonant_period = true;

//         return spin_resonance_factor * year_in_hours;
//     } else {
//         return year_in_hours;
//     }
//     }
//     return day_in_hours;
// }

/// The orbital radius is expected in units of Astronomical Units (AU).
/// Inclination is returned in units of degrees.
pub fn inclination(orbital_radius: &f64) -> f64 {
    let inclination = orbital_radius.powf(0.2) * about(EARTH_AXIAL_TILT, 0.4);
    inclination % 360.0
}

/// This function implements the escape velocity calculation. Note that it appears that Fogg's eq.15 is i/ncorrect.
/// The mass is in units of solar mass, the radius in kilometers, and the velocity returned is in cm/sec.
pub fn escape_vel(mass: &f64, radius: &f64) -> f64 {
    let mass_in_grams = mass * SOLAR_MASS_IN_GRAMS;
    let radius_in_cm = radius * CM_PER_KM;
    (2.0 * GRAV_CONSTANT * mass_in_grams / radius_in_cm).sqrt()
}

/// This is Fogg's eq.16. The molecular weight (usually assumed to be N2) is used as the basis of the Root Mean Square velocity of the molecule or atom. The velocity returned is in cm/sec.
pub fn rms_vel(molecular_weight: &f64, orbital_radius: &f64) -> f64 {
    let exospheric_temp = EARTH_EXOSPHERE_TEMP / orbital_radius.powf(2.0);
    ((3.0 * MOLAR_GAS_CONST * exospheric_temp) / molecular_weight).sqrt() * CM_PER_METER
}

/// This function returns the smallest molecular weight retained by the body, which is useful for determining the atmosphere composition. Orbital radius is in A.U.(ie: in units of the earth's orbital radius), mass is in units of solar masses, and equatorial radius is in units of kilometers.
pub fn molecule_limit(mass: &f64, equatorial_radius: &f64) -> f64 {
    let escape_velocity = escape_vel(mass, equatorial_radius);
    3.0 * (GAS_RETENTION_THRESHOLD * CM_PER_METER).powf(2.0)
        * MOLAR_GAS_CONST
        * EARTH_EXOSPHERE_TEMP
        / escape_velocity.powf(2.0)
}

/// This function calculates the surface acceleration of a planet. The mass is in units of solar masses, the radius in terms of km, and the acceleration is returned in units of cm/sec2.
pub fn acceleration(mass: &f64, radius: &f64) -> f64 {
    GRAV_CONSTANT * mass * SOLAR_MASS_IN_GRAMS / (radius * CM_PER_KM).powf(2.0)
}

/// This function calculates the surface gravity of a planet. The acceleration is in units of cm/sec2, and the gravity is returned in units of Earth gravities.
pub fn gravity(acceleration: &f64) -> f64 {
    acceleration / EARTH_ACCELERATION
}

/// Note that if the orbital radius of the planet is greater than or equal to R_inner, 99% of it's volatiles are assumed to have been deposited in surface reservoirs (otherwise, it suffers from the greenhouse effect).
pub fn greenhouse(planet: &Planetismal, zone: i32, orbital_radius: f64, ecosphere_radius: f64) -> bool {
    let greenhouse_radius = ecosphere_radius * GREENHOUSE_EFFECT_CONST;

    orbital_radius < greenhouse_radius && zone == 1 && planet.surface_pressure > 0.0
}

/// This implements Fogg's eq.17. The 'inventory' returned is unitless.
pub fn vol_inventory(
    mass: f64,
    escape_vel: f64,
    rms_vel: f64,
    stellar_mass: f64,
    zone: i32,
    greenhouse_effect: bool,
) -> f64 {
    let velocity_ratio = escape_vel / rms_vel;

    if velocity_ratio < GAS_RETENTION_THRESHOLD {
        return 0.0;
    }

    let proportion_const = match zone {
        1 => 100000.0,
        2 => 75000.0,
        3 => 250.0,
        _ => 10.0,
    };

    let mass_in_earth_units = mass * EARTH_MASSES_PER_SOLAR_MASS;
    let temp1 = proportion_const * mass_in_earth_units / stellar_mass;
    let temp2 = about(temp1, 0.2);

    if greenhouse_effect {
        return temp2;
    }

    temp2 / 100.0
}

/// This implements Fogg's eq.18. The pressure returned is in units of millibars (mb). The gravity is in units of Earth gravities, the radius in units of kilometers.
pub fn pressure(volatile_gas_inventory: &f64, equatorial_radius: &f64, gravity: &f64) -> f64 {
    let equatorial_radius = EARTH_RADIUS_IN_KM / equatorial_radius;
    volatile_gas_inventory * gravity / equatorial_radius.powf(2.0)
}

/// This function returns the boiling point of water in an atmosphere of pressure 'surface_pressure', given in millibars. The boiling point is returned in units of Kelvin. This is Fogg's eq.21.
pub fn boiling_point(surface_pressure: &f64) -> f64 {
    let surface_pressure_in_bars = surface_pressure / MILLIBARS_PER_BAR;

    1.0 / (surface_pressure_in_bars.log(std::f64::consts::E) / -5050.5 + 1.0 / 373.0)
}

/// This function is Fogg's eq.22. Given the volatile gas inventory and planetary radius of a planet (in Km), this function returns the fraction of the planet covered with water.
pub fn hydrosphere_fraction(volatile_gas_inventory: &f64, planetary_radius: &f64) -> f64 {
    let hydrosphere_fraction =
        0.75 * volatile_gas_inventory / 1000.0 * (EARTH_RADIUS_IN_KM / planetary_radius).powf(2.0);

    match hydrosphere_fraction >= 1.0 {
        true => 1.0,
        false => hydrosphere_fraction,
    }
}

/// Given the surface temperature of a planet (in Kelvin), this function  returns the fraction of cloud cover available. This is Fogg's eq.23.
/// See Hart in "Icarus" (vol 33, pp23 - 39, 1978) for an explanation.
/// This equation is Hart's eq.3.
/// It was modified slightly by using constants and relationships from
/// Glass's book "Introduction to Planetary Geology", p.46.
/// The 'CLOUD_COVERAGE_FACTOR' is the amount of surface area on Earth covered by one Kg. of cloud.
pub fn cloud_fraction(
    surface_temp: f64,
    smallest_mw_retained: f64,
    equatorial_radius: f64,
    hydrosphere_fraction: f64,
) -> f64 {
    if smallest_mw_retained > WATER_VAPOR {
        return 0.0;
    }

    let surface_area = 4.0 * PI * equatorial_radius.powf(2.0);
    let hydrosphere_mass = hydrosphere_fraction * surface_area * EARTH_WATER_MASS_PER_AREA;
    let water_vapor_in_kg =
        (0.00000001 * hydrosphere_mass) * (Q2_36 * (surface_temp - 288.0)).exp();
    let mut fraction = CLOUD_COVERAGE_FACTOR * water_vapor_in_kg / surface_area;

    if fraction >= 1.0 {
        fraction = 1.0;
    }

    fraction
}

/// Given the surface temperature of a planet (in Kelvin), this function returns the fraction of the planet's surface covered by ice. This is Fogg's eq.24. See Hart[24] in Icarus vol.33, p.28 for an explanation.
pub fn ice_fraction(hydrosphere_fraction: &f64, surface_temp: &mut f64) -> f64 {
    if *surface_temp > 328.0 {
        *surface_temp = 328.0;
    }

    let mut temp = ((328.0 - *surface_temp) / 70.0).powf(5.0);

    if temp > 1.5 * hydrosphere_fraction {
        temp = 1.5 * hydrosphere_fraction;
    }

    if temp >= 1.0 {
        return 1.0;
    }

    temp
}

/// This is Fogg's eq.19. The ecosphere radius is given in AU, the orbital radius in AU, and the temperature returned is in Kelvin.
pub fn eff_temp(ecosphere_radius: &f64, orbital_radius: &f64, albedo: &f64) -> f64 {
    (ecosphere_radius / orbital_radius).sqrt()
        * ((1.0 - albedo) / 0.7).powf(0.25)
        * EARTH_EFFECTIVE_TEMP
}

/// This is Fogg's eq.20, and is also Hart's eq.20 in his "Evolution of Earth's Atmosphere" article. The effective temperature given is in units of Kelvin, as is the rise in temperature produced by the greenhouse effect, which is returned.
pub fn green_rise(optical_depth: f64, effective_temp: f64, surface_pressure: f64) -> f64 {
    let convection_factor =
        EARTH_CONVECTION_FACTOR * (surface_pressure / EARTH_SURF_PRES_IN_MILLIBARS).powf(0.25);

    ((1.0 + 0.75 * optical_depth).powf(0.25) - 1.0) * effective_temp * convection_factor
}



//
//     if (rock_fraction >= cloud_adjustment)
// 	rock_fraction = rock_fraction - cloud_adjustment;
//     else
// 	rock_fraction = 0.0;
//     if (water_fraction > cloud_adjustment)
// 	water_fraction = water_fraction - cloud_adjustment;
//     else
// 	water_fraction = 0.0;
//     if (ice_fraction > cloud_adjustment)
// 	ice_fraction = ice_fraction - cloud_adjustment;
//     else
// 	ice_fraction = 0.0;
//     cloud_contribution = cloud_fraction * about(CLOUD_ALBEDO, 0.2);
//     if (surface_pressure == 0.0)
// 	rock_contribution = rock_fraction * about(AIRLESS_ROCKY_ALBEDO, 0.3);
//     else
// 	rock_contribution = rock_fraction * about(ROCKY_ALBEDO, 0.1);
//     water_contribution = water_fraction * about(WATER_ALBEDO, 0.2);
//     if (surface_pressure == 0.0)
// 	ice_contribution = ice_fraction * about(AIRLESS_ICE_ALBEDO, 0.4);
//     else
// 	ice_contribution = ice_fraction * about(ICE_ALBEDO, 0.1);
//     return (cloud_contribution + rock_contribution + water_contribution + ice_contribution);
// }

/// The surface temperature passed in is in units of Kelvin.
/// The cloud adjustment is the fraction of cloud cover obscuring each of the three major components of albedo that lie below the clouds.
pub fn planet_albedo(
    water_fraction: &mut f64,
    cloud_fraction: &mut f64,
    ice_fraction: &mut f64,
    surface_pressure: &mut f64,
) -> f64 {
    let mut rock_fraction = 1.0 - *water_fraction - *ice_fraction;
    let mut components = 0.0;

    if *water_fraction > 0.0 {
        components = components + 1.0;
    }
    if *ice_fraction > 0.0 {
        components = components + 1.0;
    }
    if rock_fraction > 0.0 {
        components = components + 1.0;
    }

    let cloud_adjustment = *cloud_fraction / components as f64;

    if rock_fraction >= cloud_adjustment {
        rock_fraction = rock_fraction - cloud_adjustment;
    } else {
        rock_fraction = 0.0;
    }

    if *water_fraction > cloud_adjustment {
        *water_fraction -= cloud_adjustment;
    } else {
        *water_fraction = 0.0;
    }

    if *ice_fraction > cloud_adjustment {
        *ice_fraction -= cloud_adjustment;
    } else {
        *ice_fraction = 0.0;
    }

    let cloud_contribution = *cloud_fraction * about(CLOUD_ALBEDO, 0.2);
    let water_contribution = *water_fraction * about(WATER_ALBEDO, 0.2);
    let (rock_contribution, ice_contribution) = match *surface_pressure == 0.0 {
        true => (rock_fraction * about(AIRLESS_ROCKY_ALBEDO, 0.3), *ice_fraction * about(AIRLESS_ICE_ALBEDO, 0.4)),
        false => (rock_fraction * about(ROCKY_ALBEDO, 0.1), *ice_fraction * about(ICE_ALBEDO, 0.1)),
    };


    cloud_contribution + rock_contribution + water_contribution + ice_contribution
}

/// This function returns the dimensionless quantity of optical depth, which is useful in determining the amount of greenhouse effect on a planet.
pub fn opacity(molecular_weight: f64, surface_pressure: f64) -> f64 {
    let mut optical_depth = 0.0;

    if molecular_weight >= 0.0 && molecular_weight < 10.0 {
        optical_depth += 3.0;
    }
    if molecular_weight >= 10.0 && molecular_weight < 20.0 {
        optical_depth += 2.34;
    }
    if molecular_weight >= 20.0 && molecular_weight < 30.0 {
        optical_depth += 1.0;
    }
    if molecular_weight >= 30.0 && molecular_weight < 45.0 {
        optical_depth += 0.15;
    }
    if molecular_weight >= 45.0 && molecular_weight < 100.0 {
        optical_depth += 0.05;
    }

    if surface_pressure >= 70.0 * EARTH_SURF_PRES_IN_MILLIBARS {
        optical_depth = optical_depth * 8.333;
    } else if surface_pressure >= 50.0 * EARTH_SURF_PRES_IN_MILLIBARS {
        optical_depth = optical_depth * 6.666;
    } else if surface_pressure >= 30.0 * EARTH_SURF_PRES_IN_MILLIBARS {
        optical_depth = optical_depth * 3.333;
    } else if surface_pressure >= 10.0 * EARTH_SURF_PRES_IN_MILLIBARS {
        optical_depth = optical_depth * 2.0;
    } else if surface_pressure >= 5.0 * EARTH_SURF_PRES_IN_MILLIBARS {
        optical_depth = optical_depth * 1.5;
    }

    optical_depth
}



// /*--------------------------------------------------------------------------*/
// /* The temperature calculated is in degrees Kelvin. */
// /* Quantities already known which are used in these calculations: */
// /* planet->molecule_weight */
// /* planet->surface_pressure */
// /* R_ecosphere */
// /* planet->a */
// /* planet->volatile_gas_inventory */
// /* planet->radius */
// /* planet->boil_point */
// /*--------------------------------------------------------------------------*/
// function iterate_surface_temp(planet) {

//     var surface_temp, effective_temp, greenhouse_rise, previous_temp, optical_depth, albedo = 0.0, water = 0.0, clouds = 0.0, ice = 0.0;

//     optical_depth = opacity(planet.molecule_weight, planet.surface_pressure);
//     effective_temp = eff_temp(r_ecosphere, planet.a, EARTH_ALBEDO);
//     greenhouse_rise = green_rise(optical_depth, effective_temp, planet.surface_pressure);
//     surface_temp = effective_temp + greenhouse_rise;
//     previous_temp = surface_temp - 5.0;
//     while ((Math.abs(surface_temp - previous_temp) > 1.0)) {
// 	previous_temp = surface_temp;
// 	water = hydrosphere_fraction(planet.volatile_gas_inventory, planet.radius);
// 	clouds = cloud_fraction(surface_temp, planet.molecule_weight, planet.radius, water);
// 	ice = ice_fraction(water, surface_temp);
// 	if ((surface_temp >= planet.boil_point) || (surface_temp <= FREEZING_POINT_OF_WATER)){
// 	    water = 0.0;
// 	}
// 	albedo = planet_albedo(water, clouds, ice, planet.surface_pressure);
// 	optical_depth = opacity(planet.molecule_weight, planet.surface_pressure);
// 	effective_temp = eff_temp(r_ecosphere, planet.a, albedo);
// 	greenhouse_rise = green_rise(optical_depth, effective_temp, planet.surface_pressure);
// 	surface_temp = effective_temp + greenhouse_rise;
//     }
//     planet.hydrosphere = water;
//     planet.cloud_cover = clouds;
//     planet.ice_cover = ice;
//     planet.albedo = albedo;
//     planet.surface_temp = surface_temp;
// };
// The temperature calculated is in degrees Kelvin.
// Quantities already known which are used in these calculations:
// planet->molecule_weight
// planet->surface_pressure
// R_ecosphere
// planet->a
// planet->volatile_gas_inventory
// planet->radius
// planet->
// pub fn iterate_surface_temp(planet: &Planet, ecosphere_radius: f64) -> f64 {
//     let albedo = 0.0;
//     let water = 0.0;
//     let clouds = 0.0;
//     let ice = 0.0;

//     let optical_depth = planet.opacity(planet.molecule_weight, planet.surface_pressure);
//     let effective_temp = planet.eff_temp(ecosphere_radius, planet.a, EARTH_ALBEDO);
//     let greenhouse_rise = planet.green_rise(optical_depth, effective_temp, planet.surface_pressure);
//     let surface_temp = effective_temp + greenhouse_rise;
//     let previous_temp = surface_temp - 5.0;

//     while (surface_temp - previous_temp).abs() > 1.0 {
//         previous_temp = surface_temp;
//         water = planet.hydrosphere_fraction(planet.volatile_gas_inventory, planet.radius);
//         clouds = planet.cloud_fraction(surface_temp, planet.molecule_weight, planet.radius, water);
//         ice = planet.ice_fraction(water, surface_temp);

//         if surface_temp >= planet.boilPoint || surface_temp <= FREEZING_POINT_OF_WATER {
//             water = 0.0;
//         }
//         albedo = planet.planet_albedo(water, clouds, ice, planet.surface_pressure);
//         optical_depth = planet.opacity(planet.molecule_weight, planet.surface_pressure);
//         effective_temp = planet.eff_temp(ecosphere_radius, planet.a, albedo);
//         greenhouse_rise = planet.green_rise(optical_depth, effective_temp, planet.surface_pressure);
//         surface_temp = effective_temp + greenhouse_rise;
//     }
//     // planet.hydrosphere = water;
//     // planet.cloud_cover = clouds;
//     // planet.ice_cover = ice;
//     // planet.albedo = albedo;
//     // planet.surface_temp = surface_temp;
//     surface_temp
// }
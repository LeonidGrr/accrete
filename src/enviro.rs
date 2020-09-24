use crate::consts::*;
use crate::structs::planetesimal::*;
use crate::utils::*;

/// This function, given the orbital radius of a planet in AU, returns the orbital 'zone' of the particle.
pub fn orbital_zone(luminosity: &f64, distance_to_primary_star: f64) -> i32 {
    if distance_to_primary_star < (4.0 * luminosity.sqrt()) {
        return 1;
    } else if distance_to_primary_star < (15.0 * luminosity.sqrt()) {
        return 2;
    }
    3
}

/// The mass is in units of solar masses, and the density is in units of grams/cc. The radius returned is in units of km.
pub fn volume_radius(mass: &f64, density: &f64) -> f64 {
    let volume = mass * SOLAR_MASS_IN_GRAMS / density;
    ((3.0 * volume) / (4.0 * PI)).powf(0.33) / CM_PER_KM
}

/// Returns the radius of the planet in kilometers.
/// The mass passed in is in units of solar masses, the orbital radius in A.U.
/// This formula is listed as eq.9 in Fogg's article, although some typos crop up in that eq. See "The Internal Constitution of Planets", by Dr. D. S. Kothari, Mon. Not. of the Royal Astronomical Society, vol 96 pp.833-843, 1936 for the derivation. Specifically, this is Kothari's eq.23, which appears on page 840.
pub fn kothari_radius(mass: &f64, giant: &bool, zone: &i32) -> f64 {
    let mut atomic_weight = 0.0;
    let mut atomic_num = 0.0;

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

    let mut temp = atomic_weight * atomic_num;
    temp = 2.0 * BETA_20 * SOLAR_MASS_IN_GRAMS.powf(1.0 / 3.0)
        / (A1_20 * (temp as f64).powf(1.0 / 3.0));

    let mut temp2 = A2_20 * atomic_weight.powf(4.0 / 3.0) * SOLAR_MASS_IN_GRAMS.powf(2.0 / 3.0);
    temp2 *= mass.powf(2.0 / 3.0);
    temp2 /= A1_20 * atomic_num.powf(2.0);
    temp2 += 1.0;

    temp /= temp2;
    temp = (temp * mass.powf(1.0 / 3.0)) / CM_PER_KM;
    temp /= JIMS_FUDGE;

    temp
}

/// The mass passed in is in units of solar masses, and the orbital radius is in units of AU. The density is returned in units of grams/cc.
pub fn empirical_density(
    mass: &f64,
    distance_to_primary_star: &f64,
    ecosphere_radius: &f64,
    is_gas_giant: &bool,
) -> f64 {
    let mut density = (mass * EARTH_MASSES_PER_SOLAR_MASS).powf(1.0 / 8.0);
    density *= (ecosphere_radius / distance_to_primary_star).powf(0.25);

    match is_gas_giant {
        true => density * 1.2,
        false => density * 5.5,
    }
}

/// The mass passed in is in units of solar masses, and the equatorial radius is in km. The density is returned in units of grams/cc.
pub fn volume_density(mass: &f64, equat_radius: &f64) -> f64 {
    let equat_radius = equat_radius * CM_PER_KM;
    let volume = (4.0 * PI * equat_radius.powf(3.0)) / 3.0;
    mass * SOLAR_MASS_IN_GRAMS / volume
}

/// Separation - Units of AU between the masses returns the period of an entire xorbit in Earth days.
pub fn period(separation: &f64, small_mass: &f64, large_mass: &f64) -> f64 {
    let period_in_years = (separation.powf(3.0) / (small_mass + large_mass)).sqrt();
    // period_in_years * planet.days_in_year
    period_in_years * DAYS_IN_A_YEAR
}

/// Fogg's information for this routine came from Dole "Habitable Planets for Man", Blaisdell Publishing Company, NY, 1964. From this, he came up with his eq.12, which is the equation for the base_angular_velocity below.
/// Going a bit further, he found an equation for the change in angular velocity per time (dw/dt) from P. Goldreich and S. Soter's paper "Q in the Solar System" in Icarus, vol 5, pp.375-389 (1966).
/// Comparing to the change in angular velocity for the Earth, we can come up with an approximation for our new planet (his eq.13) and take that into account.
pub fn day_length(planet: &mut Planetesimal, stellar_mass: &f64, main_sequence_age: &f64) -> f64 {
    let planet_mass_in_grams = planet.mass * SOLAR_MASS_IN_GRAMS;
    let equatorial_radius_in_cm = planet.radius * CM_PER_KM;
    let year_in_hours = planet.orbital_period_days;

    let k2 = match planet.is_gas_giant {
        true => 0.24,
        false => 0.33,
    };

    let base_angular_velocity =
        (2.0 * J * planet_mass_in_grams).sqrt() / (k2 * equatorial_radius_in_cm.powf(2.0));

    let change_in_angular_velocity = CHANGE_IN_EARTH_ANG_VEL
        * (planet.density / EARTH_DENSITY)
        * (equatorial_radius_in_cm / EARTH_RADIUS_IN_CM)
        * (EARTH_MASS_IN_GRAMS / planet_mass_in_grams)
        * stellar_mass.powf(2.0)
        * (1.0 / planet.a.powf(6.0));

    let ang_velocity = base_angular_velocity + (change_in_angular_velocity * main_sequence_age);

    let (stopped, day_in_hours) = match ang_velocity <= 0.0 {
        true => (true, INCREDIBLY_LARGE_NUMBER),
        false => (
            false,
            RADIANS_PER_ROTATION / (SECONDS_PER_HOUR * ang_velocity),
        ),
    };

    if day_in_hours >= year_in_hours || stopped {
        if planet.e > 0.1 {
            let spin_resonance_factor = (1.0 - planet.e) / (1.0 + planet.e);
            planet.resonant_period = true;
            return spin_resonance_factor * year_in_hours;
        } else {
            return year_in_hours;
        }
    }
    day_in_hours
}

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
pub fn greenhouse(
    distance_to_primary_star: &f64,
    orbit_zone: &i32,
    surface_pressure_bar: &f64,
    ecosphere_radius: &f64,
) -> bool {
    let greenhouse_radius = ecosphere_radius * GREENHOUSE_EFFECT_CONST;
    *distance_to_primary_star < greenhouse_radius && *orbit_zone == 1 && *surface_pressure_bar > 0.0
}

/// This implements Fogg's eq.17. The 'inventory' returned is unitless.
pub fn vol_inventory(
    mass: &f64,
    escape_vel: &f64,
    rms_vel: &f64,
    stellar_mass: &f64,
    zone: &i32,
    greenhouse_effect: &bool,
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

    if *greenhouse_effect {
        return temp2;
    }

    temp2 / 100.0
}

/// This implements Fogg's eq.18. The pressure returned is in units of bars. The gravity is in units of Earth gravities, the radius in units of kilometers.
pub fn pressure(volatile_gas_inventory: &f64, equatorial_radius: &f64, gravity: &f64) -> f64 {
    let equatorial_radius = EARTH_RADIUS_IN_KM / equatorial_radius;
    (volatile_gas_inventory * gravity / equatorial_radius.powf(2.0)) / EARTH_SURF_PRES_IN_MILLIBARS
}

/// This function returns the boiling point of water in an atmosphere of pressure 'surface_pressure_bar', given in bars. The boiling point is returned in units of Kelvin. This is Fogg's eq.21.
pub fn boiling_point_kelvin(surface_pressure_bar: &f64) -> f64 {
    1.0 / (surface_pressure_bar.log(std::f64::consts::E) / -5050.5 + 1.0 / 373.0)
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
    surface_temp_kelvin: f64,
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
        (0.00000001 * hydrosphere_mass) * (Q2_36 * (surface_temp_kelvin - 288.0)).exp();
    let mut fraction = CLOUD_COVERAGE_FACTOR * water_vapor_in_kg / surface_area;

    if fraction >= 1.0 {
        fraction = 1.0;
    }

    fraction
}

/// Given the surface temperature of a planet (in Kelvin), this function returns the fraction of the planet's surface covered by ice. This is Fogg's eq.24. See Hart[24] in Icarus vol.33, p.28 for an explanation.
pub fn ice_fraction(hydrosphere_fraction: &f64, surface_temp_kelvin: &f64) -> f64 {
    let surface_temp_kelvin = match *surface_temp_kelvin > 328.0 {
        true => 328.0,
        false => *surface_temp_kelvin,
    };
    let mut temp = ((328.0 - surface_temp_kelvin) / 70.0).powf(5.0);

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
pub fn green_rise(optical_depth: f64, effective_temp: f64, surface_pressure_bar: f64) -> f64 {
    let convection_factor = EARTH_CONVECTION_FACTOR * surface_pressure_bar.powf(0.25);
    ((1.0 + 0.75 * optical_depth).powf(0.25) - 1.0) * effective_temp * convection_factor
}

/// The surface temperature passed in is in units of Kelvin.
/// The cloud adjustment is the fraction of cloud cover obscuring each of the three major components of albedo that lie below the clouds.
pub fn planet_albedo(
    water_fraction: &f64,
    cloud_fraction: &f64,
    ice_fraction: &f64,
    surface_pressure_bar: &f64,
) -> f64 {
    let mut rock_fraction = 1.0 - *water_fraction - *ice_fraction;
    let mut components = 0.0;

    if *water_fraction > 0.0 {
        components += 1.0;
    }
    if *ice_fraction > 0.0 {
        components += 1.0;
    }
    if rock_fraction > 0.0 {
        components += 1.0;
    }

    let cloud_adjustment = *cloud_fraction / components as f64;

    if rock_fraction >= cloud_adjustment {
        rock_fraction -= cloud_adjustment;
    } else {
        rock_fraction = 0.0;
    }

    let water_fraction = match *water_fraction > cloud_adjustment {
        true => *water_fraction - cloud_adjustment,
        false => 0.0,
    };

    let ice_fraction = match *ice_fraction > cloud_adjustment {
        true => *ice_fraction - cloud_adjustment,
        false => 0.0,
    };

    let cloud_contribution = *cloud_fraction * about(CLOUD_ALBEDO, 0.2);
    let water_contribution = water_fraction * about(WATER_ALBEDO, 0.2);
    let (rock_contribution, ice_contribution) = match *surface_pressure_bar == 0.0 {
        true => (
            rock_fraction * about(AIRLESS_ROCKY_ALBEDO, 0.3),
            ice_fraction * about(AIRLESS_ICE_ALBEDO, 0.4),
        ),
        false => (
            rock_fraction * about(ROCKY_ALBEDO, 0.1),
            ice_fraction * about(ICE_ALBEDO, 0.1),
        ),
    };

    cloud_contribution + rock_contribution + water_contribution + ice_contribution
}

/// This function returns the dimensionless quantity of optical depth, which is useful in determining the amount of greenhouse effect on a planet.
pub fn opacity(molecular_weight: f64, surface_pressure_bar: f64) -> f64 {
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

    if surface_pressure_bar >= 0.07 {
        optical_depth *= 8.333;
    } else if surface_pressure_bar >= 0.05 {
        optical_depth *= 6.666;
    } else if surface_pressure_bar >= 0.03 {
        optical_depth *= 3.333;
    } else if surface_pressure_bar >= 0.01 {
        optical_depth *= 2.0;
    } else if surface_pressure_bar >= 0.005 {
        optical_depth *= 1.5;
    }

    optical_depth
}

/// Convert solar mass to Earth mass
pub fn get_earth_mass(mass: f64) -> f64 {
    mass * EARTH_MASSES_PER_SOLAR_MASS
}

/// The temperature calculated is in degrees Kelvin.
pub fn iterate_surface_temp(planet: &mut Planetesimal, ecosphere_radius: &f64) -> f64 {
    let mut albedo = 0.0;
    let mut water = 0.0;
    let mut clouds = 0.0;
    let mut ice = 0.0;

    let mut optical_depth = opacity(planet.molecule_weight, planet.surface_pressure_bar);
    let mut effective_temp = eff_temp(ecosphere_radius, &planet.a, &EARTH_ALBEDO);
    let mut greenhouse_rise =
        green_rise(optical_depth, effective_temp, planet.surface_pressure_bar);
    let mut surface_temp_kelvin = effective_temp + greenhouse_rise;
    let mut previous_temp = surface_temp_kelvin - 5.0;

    while (surface_temp_kelvin - previous_temp).abs() > 1.0 {
        previous_temp = surface_temp_kelvin;
        water = hydrosphere_fraction(&planet.volatile_gas_inventory, &planet.radius);
        clouds = cloud_fraction(
            surface_temp_kelvin,
            planet.molecule_weight,
            planet.radius,
            water,
        );
        ice = ice_fraction(&water, &surface_temp_kelvin);

        if surface_temp_kelvin >= planet.boiling_point_kelvin
            || surface_temp_kelvin <= FREEZING_POINT_OF_WATER
        {
            water = 0.0;
        }
        albedo = planet_albedo(&water, &clouds, &ice, &planet.surface_pressure_bar);
        optical_depth = opacity(planet.molecule_weight, planet.surface_pressure_bar);
        effective_temp = eff_temp(ecosphere_radius, &planet.a, &albedo);
        greenhouse_rise = green_rise(optical_depth, effective_temp, planet.surface_pressure_bar);
        surface_temp_kelvin = effective_temp + greenhouse_rise;
    }
    planet.hydrosphere = water;
    planet.cloud_cover = clouds;
    planet.ice_cover = ice;
    planet.albedo = albedo;
    planet.surface_temp_kelvin = surface_temp_kelvin;
    surface_temp_kelvin
}

pub fn check_tidal_lock(day_length: f64, orbital_period: f64) -> bool {
    let error_margin = f64::EPSILON;
    (day_length - orbital_period * day_length).abs() < error_margin
}

// Habitable moons:
// Based on tidal heating models, scientists have defined zones in satellite systems similarly to those of planetary systems. One such zone is the circumplanetary habitable zone (or "habitable edge"). According to this theory, moons closer to their planet than the habitable edge cannot support liquid water at their surface. When effects of eclipses as well as constraints from a satellite's orbital stability are included into this concept, one finds that — depending on a moon's orbital eccentricity — there is a minimum mass of roughly 0.2 solar masses for stars to host habitable moons within the stellar HZ.[48]
// The magnetic environment of exomoons, which is critically triggered by the intrinsic magnetic field of the host planet, has been identified as another effect on exomoon habitability.[49] Most notably, it was found that moons at distances between about 5 and 20 planetary radii from a giant planet can be habitable from an illumination and tidal heating point of view, but still the planetary magnetosphere would critically influence their habitability.

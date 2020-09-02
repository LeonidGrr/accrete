/// Fogg's information for this routine came from Dole "Habitable Planets for Man", Blaisdell Publishing Company, NY, 1964. From this, he came up with his eq.12, which is the equation for the base_angular_velocity below. Going a bit further, he found an equation for the change in angular velocity per time (dw/dt) from P. Goldreich and S. Soter's paper "Q in the Solar System" in Icarus, vol 5, pp.375-389 (1966). Comparing to the change in angular velocity for the Earth, we can come up with an approximation for our new planet (his eq.13) and take that into account.
//     pub fn day_length(&self, planet, stellar_mass, main_sequence_age) -> f64 {
//         let planet_mass_in_grams = planet.mass * self.solar_mass_in_grams,
//         equatorial_radius_in_cm = planet.radius * CM_PER_KM,
//         year_in_hours = planet.orbPeriod || self.period(planet.axis, planet.mass, 1),
//         giant = planet.giant || false,
//         k2 = 0,
//         base_angular_velocity = 0,
//         change_in_angular_velocity = 0,
//         ang_velocity = 0,
//         spin_resonance_factor = 0,
//         day_in_hours = 0,
//         stopped = false;

//     planet.resonant_period = false;

//     if (giant) {
//       k2 = 0.24;
//     }
//     else {
//       k2 = 0.33;
//     }

//     base_angular_velocity = Math.sqrt(2 * J * planet_mass_in_grams) /
//       (k2 * Math.pow(equatorial_radius_in_cm, 2));

//     change_in_angular_velocity = self.change_in_earth_ang_vel *
//       (planet.density / EARTH_DENSITY) *
//       (equatorial_radius_in_cm / EARTH_RADIUS_IN_CM) *
//       (EARTH_MASS_IN_GRAMS / planet_mass_in_grams) *
//       Math.pow(stellar_masss, 2) *
//       (1 / Math.pow(planet.axis, 6));

//     ang_velocity = base_angular_velocity + (change_in_angular_velocity * main_sequence_age);

//     if (ang_velocity <= 0.0) {
//       stopped = true;
//       day_in_hours = self.very_large_number;
//     }
//     else {
//       day_in_hours = self.radians_per_rotation / (seconds_per_hour * ang_velocity);
//     }

//     if (day_in_hours >= year_in_hours || stopped) {
//       if (planet.eccn > 0.1) {
//         spin_resonance_factor = (1 - planet.eccn) / (1 + planet.eccn);
//         planet.resonant_period = true;

//         return spin_resonance_factor * year_in_hours;
//       }
//       else {
//         return year_in_hours;
//       }
//     }

//     return day_in_hours;
//   },

/// This function implements the escape velocity calculation. Note that it appears that Fogg's eq.15 is i/ncorrect.
/// The mass is in units of solar mass, the radius in kilometers, and the velocity returned is in cm/sec.
pub fn escape_vel(mass: f64, radius: f64) -> f64 {
    let mass_in_grams = mass * SOLAR_MASS_IN_GRAMS;
    let radius_in_cm = radius * CM_PER_KM;
    (2.0 * GRAV_CONSTANT * mass_in_grams / radius_in_cm).sqrt()
}

/// The orbital radius is expected in units of Astronomical Units (AU).
/// Inclination is returned in units of degrees.
// pub fn inclination(orbital_radius: f64) -> f64 {
//     let inclination = orbital_radius.powf(0.2) * utils.about(EARTH_AXIAL_TILT, 0.4);
//     inclination % 360.0
// }







/// This implements Fogg's eq.18. The pressure returned is in units of millibars (mb). The gravity is in units of Earth gravities, the radius in units of kilometers.
// pub fn pressure(volatile_gas_inventory: f64, equatorial_radius: f64, gravity: f64) -> f64 {
//     equatorial_radius = EARTH_RADIUS_IN_KM / equatorial_radius;
//     volatile_gas_inventory * gravity / equatorial_radius.powf(2.0)
// }

/// Note that if the orbital radius of the planet is greater than or equal to R_inner, 99% of it's volatiles are assumed to have been deposited in surface reservoirs (otherwise, it suffers from the greenhouse effect).
// pub fn greenhouse(planet: &Planet, zone: i32, orbital_radius: f64, ecosphere_radius: f64) -> bool {
//     let greenhouse_radius = ecosphere_radius * GREENHOUSE_EFFECT_CONST;

//     orbital_radius < greenhouse_radius && zone == 1 && planet.pressure > 0
// }

/// This implements Fogg's eq.17. The 'inventory' returned is unitless.
// pub fn vol_inventory(
//     mass: f64,
//     escape_vel: f64,
//     rms_vel: f64,
//     stellar_mass: f64,
//     zone: i32,
//     greenhouse_effect: bool,
// ) -> f64 {
//     let velocity_ratio = escape_vel / rms_vel;

//     if velocity_ratio < GAS_RETENTION_THRESHOLD {
//         return 0.0;
//     }

//     let proportion_const = match zone {
//         1 => 100000.0,
//         2 => 75000.0,
//         3 => 250.0,
//         _ => 10.0,
//     };

// let mass_in_earth_units = mass * EARTH_MASSES_PER_SOLAR_MASS;
// let temp1 = proportion_const * mass_in_earth_units / stellar_mass;
// let temp2 = utils.about(temp1, 0.2);

// if greenhouse_effect {
//     return temp2;
// }

// temp2 / 100.0
// }

/// This function returns the boiling point of water in an atmosphere of pressure 'surface_pressure', given in millibars. The boiling point is returned in units of Kelvin. This is Fogg's eq.21.
pub fn boiling_point(surface_pressure: f64) -> f64 {
    let surface_pressure_in_bars = surface_pressure / MILLIBARS_PER_BAR;

    1.0 / (surface_pressure_in_bars.log(std::f64::consts::E) / -5050.5 + 1.0 / 373.0)
}

/// This function is Fogg's eq.22. Given the volatile gas inventory and planetary radius of a planet (in Km), this function returns the fraction of the planet covered with water.
pub fn hydrosphere_fraction(volatile_gas_inventory: f64, planetary_radius: f64) -> f64 {
    let hydrosphere_fraction =
        0.75 * volatile_gas_inventory / 1000.0 * (EARTH_RADIUS_IN_KM / planetary_radius).powf(2.0);

    match hydrosphere_fraction >= 1.0 {
        true => 1.0,
        false => hydrosphere_fraction,
    }
}

/// The temperature calculated is in degrees Kelvin.
/// Quantities already known which are used in these calculations:
/// planet->molecule_weight
/// planet->surface_pressure
/// R_ecosphere
/// planet->a
/// planet->volatile_gas_inventory
/// planet->radius
/// planet->
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

/// This is Fogg's eq.20, and is also Hart's eq.20 in his "Evolution of Earth's Atmosphere" article. The effective temperature given is in units of Kelvin, as is the rise in temperature produced by the greenhouse effect, which is returned.
pub fn green_rise(optical_depth: f64, effective_temp: f64, surface_pressure: f64) -> f64 {
    let convection_factor =
        EARTH_CONVECTION_FACTOR * (surface_pressure / EARTH_SURF_PRES_IN_MILLIBARS).powf(0.25);

    ((1.0 + 0.75 * optical_depth).powf(0.25) - 1.0) * effective_temp * convection_factor
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

/// The surface temperature passed in is in units of Kelvin.
/// The cloud adjustment is the fraction of cloud cover obscuring each of the three major components of albedo that lie below the clouds.
// pub fn planet_albedo(
//     water_fraction: f64,
//     cloud_fraction: f64,
//     ice_fraction: f64,
//     surface_pressure: f64,
// ) -> f64 {
//     let rock_contribution = 0.0;
//     let ice_contribution = 0.0;
//     let rock_fraction = 1.0 - water_fraction - ice_fraction;
//     let components = 0.0;

//     if water_fraction > 0.0 {
//         components = components + 1.0;
//     }
//     if ice_fraction > 0.0 {
//         components = components + 1.0;
//     }
//     if rock_fraction > 0.0 {
//         components = components + 1.0;
//     }

//     let cloud_adjustment = cloud_fraction / components;

//     if rock_fraction >= cloud_adjustment {
//         rock_fraction = rock_fraction - cloud_adjustment;
//     } else {
//         rock_fraction = 0.0;
//     }

//     if water_fraction > cloud_adjustment {
//         water_fraction = water_fraction - cloud_adjustment;
//     } else {
//         water_fraction = 0.0;
//     }

//     if ice_fraction > cloud_adjustment {
//         ice_fraction = ice_fraction - cloud_adjustment;
//     } else {
//         ice_fraction = 0.0;
//     }

//     let cloud_contribution = cloud_fraction * utils.about(CLOUD_ALBEDO, 0.2);

//     if surface_pressure == 0.0 {
//         rock_contribution = rock_fraction * utils.about(AIRLESS_ROCKY_ALBEDO, 0.3);
//     } else {
//         rock_contribution = rock_fraction * utils.about(ROCKY_ALBEDO, 0.1);
//     }

//     let water_contribution = water_fraction * utils.about(WATER_ALBEDO, 0.2);

//     if surface_pressure == 0.0 {
//         ice_contribution = ice_fraction * utils.about(AIRLESS_ICE_ALBEDO, 0.4);
//     } else {
//         ice_contribution = ice_fraction * utils.about(ICE_ALBEDO, 0.1);
//     }

//     cloud_contribution + rock_contribution + water_contribution + ice_contribution
// }

/// This is Fogg's eq.19. The ecosphere radius is given in AU, the orbital radius in AU, and the temperature returned is in Kelvin.
pub fn eff_temp(ecosphere_radius: f64, orbital_radius: f64, albedo: f64) -> f64 {
    (ecosphere_radius / orbital_radius).sqrt()
        * ((1.0 - albedo) / 0.7).powf(0.25)
        * EARTH_EFFECTIVE_TEMP
}

/// Given the surface temperature of a planet (in Kelvin), this function returns the fraction of the planet's surface covered by ice. This is Fogg's eq.24. See Hart[24] in Icarus vol.33, p.28 for an explanation.
pub fn ice_fraction(hydrosphere_fraction: f64, surface_temp: &mut f64) -> f64 {
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

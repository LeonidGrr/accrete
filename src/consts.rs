#![allow(dead_code)]

pub const PI: f64 = std::f64::consts::PI;
pub const ECCENTRICITY_COEFF: f64 = 0.077; /* Dole's was 0.077 */
pub const PROTOPLANET_MASS: f64 = 1.0E-15; /* Units of solar masses */
pub const SOLAR_MASS_IN_GRAMS: f64 = 1.989E33; /* Units of grams */
pub const EARTH_MASS_IN_GRAMS: f64 = 5.977E27; /* Units of grams */
pub const EARTH_RADIUS: f64 = 6.378E6; /* Units of cm */
pub const EARTH_RADIUS_IN_KM: f64 = 6378.0; /* Units of km */
pub const EARTH_ACCELERATION: f64 = 981.0; /* Units of cm/sec2 */
pub const EARTH_AXIAL_TILT: f64 = 23.4; /* Units of degrees */
pub const EARTH_EXOSPHERE_TEMP: f64 = 1273.0; /* Units of degrees Kelvin */
pub const EARTH_MASSES_PER_SOLAR_MASS: f64 = 332775.64;
pub const EARTH_EFFECTIVE_TEMP: f64 = 255.0; /* Units of degrees Kelvin */
pub const EARTH_ALBEDO: f64 = 0.39;
pub const CLOUD_COVERAGE_FACTOR: f64 = 1.839E-8; /* Km2/kg */
pub const EARTH_WATER_MASS_PER_AREA: f64 = 3.83E15; /* grams per square km */
pub const EARTH_SURF_PRES_IN_MILLIBARS: f64 = 1000.0;
pub const EARTH_CONVECTION_FACTOR: f64 = 0.43; /* from Hart, eq.20 */
pub const FREEZING_POINT_OF_WATER: f64 = 273.0; /* Units of degrees Kelvin */
pub const DAYS_IN_A_YEAR: f64 = 365.256; /* Earth days per Earth year */

/* gas_retention_threshold: f64 = 6.; *//* ratio of esc vel to RMS vel */

pub const GAS_RETENTION_THRESHOLD: f64 = 5.0; /* ratio of esc vel to RMS vel */
pub const GAS_GIANT_ALBEDO: f64 = 0.5; /* albedo of a gas giant */
pub const CLOUD_ALBEDO: f64 = 0.52;
pub const AIRLESS_ROCKY_ALBEDO: f64 = 0.07;
pub const ROCKY_ALBEDO: f64 = 0.15;
pub const WATER_ALBEDO: f64 = 0.04;
pub const AIRLESS_ICE_ALBEDO: f64 = 0.5;
pub const ICE_ALBEDO: f64 = 0.7;
pub const SECONDS_PER_HOUR: f64 = 3600.0;
pub const CM_PER_AU: f64 = 1.495978707E13; /* number of cm in an AU */
pub const CM_PER_KM: f64 = 1.0E5; /* number of cm in a km */
pub const CM_PER_METER: f64 = 100.0;
pub const MILLIBARS_PER_BAR: f64 = 1000.0;
pub const KELVIN_CELCIUS_DIFFERENCE: f64 = 273.0;
pub const GRAV_CONSTANT: f64 = 6.672E-8; /* units of dyne cm2/gram2 */
pub const GREENHOUSE_EFFECT_CONST: f64 = 0.93; /* affects inner radius.. */
pub const MOLAR_GAS_CONST: f64 = 8314.41; /* units: g*m2/=(sec2*K*mol; */
pub const K: f64 = 50.0; /* K: f64 = gas/dust ratio */
pub const B: f64 = 1.2E-5; /* Used in Crit_mass calc */
pub const DUST_DENSITY_COEFF: f64 = 2.0E-3; /* A in Dole's paper */
pub const ALPHA: f64 = 5.0; /* Used in density calcs */
pub const N: f64 = 3.0; /* Used in density calcs */
pub const J: f64 = 1.46E-19; /* Used in day-length calcs: f64 =(cm2/sec2 g; */
pub const INCREDIBLY_LARGE_NUMBER: f64 = 9.9999E37;

/* Now for a few molecular weights: f64 =(used for RMS velocity calcs;: */
/* This table is from Dole's book "Habitable Planets for Man", p. 38 */
pub const ATOMIC_HYDROGEN: f64 = 1.0; /* H */
pub const MOLECULAR_HYDROGEN: f64 = 2.0; /* H2 */
pub const HELIUM: f64 = 4.0; /* He */
pub const ATOMIC_NITROGEN: f64 = 14.0; /* N */
pub const ATOMIC_OXYGEN: f64 = 16.0; /* O */
pub const METHANE: f64 = 16.0; /* CH4 */
pub const AMMONIA: f64 = 17.0; /* NH3 */
pub const WATER_VAPOR: f64 = 18.0; /* H2O */
pub const NEON: f64 = 20.2; /* Ne */
pub const MOLECULAR_NITROGEN: f64 = 28.0; /* N2 */
pub const CARBON_MONOXIDE: f64 = 28.0; /* CO */
pub const NITRIC_OXIDE: f64 = 30.0; /* NO */
pub const MOLECULAR_OXYGEN: f64 = 32.0; /* O2 */
pub const HYDROGEN_SULPHIDE: f64 = 34.1; /* H2S */
pub const ARGON: f64 = 39.9; /* Ar */
pub const CARBON_DIOXIDE: f64 = 44.0; /* CO2 */
pub const NITROUS_OXIDE: f64 = 44.0; /* N2O */
pub const NITROGEN_DIOXIDE: f64 = 46.0; /* NO2 */
pub const OZONE: f64 = 48.0; /* O3 */
pub const SULPHUR_DIOXIDE: f64 = 64.1; /* SO2 */
pub const SULPHUR_TRIOXIDE: f64 = 80.1; /* SO3 */
pub const KRYPTON: f64 = 83.8; /* Kr */
pub const XENON: f64 = 131.3; /* Xe */

/* The following defines are used in the kothari_radius function in */
/* file enviro.c. */
pub const A1_20: f64 = 6.485E12; /* All units are in cgs system. */
pub const A2_20: f64 = 4.0032E-8; /* ie: cm, g, dynes, etc. */
pub const BETA_20: f64 = 5.71E12;

/* The following defines are used in determining the fraction of a planet */
/* covered with clouds in function cloud_fraction in file enviro.c. */
pub const Q1_36: f64 = 1.258E19; /* grams */
pub const Q2_36: f64 = 0.0698; /* 1/Kelvin */

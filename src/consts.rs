#![allow(dead_code)]
use std::f64::consts::PI;

// Currently using this to limit extreme precision when creating star systems.
pub const MAX_SYSTEM_ITERATIONS: i32 = 100000;

/// Sources:
/// "Formation of Planetary Systems by Aggregation: A Computer Simulation" by Stephen H. Dole


/// Optimal perameters as specified in Dole's paper: A = 0.0015, K =  50, ⍺ = 5, n = 3
/// Initial mass-of-matter in solar masses per cubic A.U. (`A` in Dole's paper)
/// Dole's paper tests ranges between 0.00125 and 0.0015
pub const A: f64 = 0.0015; 

/// The dust-to-gas ratio 50-100 (dust/gas = K), gas = hydrogen and helium, dust = other
pub const K: f64 = 50.0; 

/// Eccentricity of dust cloud 0.15-0.25
pub const W: f64 = 0.20;

/// Negative exponential coefficient used in calculating dust density - alpha in Dole's paper
pub const ALPHA: f64 = 5.0; 

/// Used in calculating dust density (as the nth root of the radius, r)
pub const N: f64 = 3.0;

/// Used to calculate the eccentricity of planetary nuclei (Dole states this conforms to an empirical probability function for distribution of orbital eccentricities)
pub const Q: f64 = 0.077;

/// For critical mass, 1e-5 to 1.2e-5
pub const B: f64 = 1.2e-5; 

/// Maximum angular inclination of dust cloud (Dole specifies as ~90 degrees)
pub const OMEGA: f64 = PI / 2.01;

/// STAR GEN PARAMS

/// Dole's was 0.077
pub const ECCENTRICITY_COEFF: f64 = 0.077;

/// Units of solar masses
pub const PROTOPLANET_MASS: f64 = 1.0e-15; // Units of solar masses
pub const PROTOMOON_MASS: f64 = 1.0e-15; // Units of solar masses

/// Units of radians/sec/year
pub const CHANGE_IN_EARTH_ANG_VEL: f64 = -1.3e-15;

/// Units of grams
pub const SOLAR_MASS_IN_GRAMS: f64 = 1.989e33;

/// Units of kg 
pub const SOLAR_MASS_IN_KILOGRAMS: f64 = 1.989e30;  
pub const SOLAR_MASS_IN_EARTH_MASS: f64 = 332775.64;

/// Units of grams
pub const EARTH_MASS_IN_GRAMS: f64 = 5.977e27;

pub const EARTH_RADIUS: f64 = 6.378e8; 
pub const EARTH_RADIUS_IN_CM: f64 = 6.378e6;
pub const EARTH_RADIUS_IN_KM: f64 = 6378.0;

/// Units of g/cc 
pub const EARTH_DENSITY: f64 = 5.52;

/// Units of km 
pub const KM_EARTH_RADIUS: f64 = 6378.0;  

/// Units of cm/sec2
pub const EARTH_ACCELERATION: f64 = 980.7;

/// Units of degrees
pub const EARTH_AXIAL_TILT: f64 = 23.4; 

/// Units of degrees Kelvin
pub const EARTH_EXOSPHERE_TEMP: f64 = 1273.0;

/// Units of Earth Masses
pub const ASTEROID_MASS_LIMIT: f64 = 0.001;

/// Units of degrees Kelvin (was 255)
pub const EARTH_EFFECTIVE_TEMP: f64 = 250.0; 

/// Km2/kg 
pub const CLOUD_COVERAGE_FACTOR: f64 = 1.839e-8;

/// grams per square km  
pub const EARTH_WATER_MASS_PER_AREA: f64 = 3.83e15;
pub const EARTH_SURF_PRES_IN_MILLIBARS: f64 = 1013.25;

/// Dole p. 15 
pub const EARTH_SURF_PRES_IN_MMHG: f64 = 760.0;

/// Pounds per square inch
pub const EARTH_SURF_PRES_IN_PSI: f64 = 14.696;

/// EARTH_SURF_PRES_IN_MMHG;
pub const MMHG_TO_MILLIBARS: f64 = EARTH_SURF_PRES_IN_MILLIBARS;

/// EARTH_SURF_PRES_IN_PSI;
pub const PSI_TO_MILLIBARS: f64 = EARTH_SURF_PRES_IN_MILLIBARS;

/// Dole p. 15 
pub const H20_ASSUMED_PRESSURE: f64 = 47.0 * MMHG_TO_MILLIBARS;
pub const MIN_O2_IPP: f64 = 72.0 * MMHG_TO_MILLIBARS;   
pub const MAX_O2_IPP: f64 = 400.0 * MMHG_TO_MILLIBARS;   

/// Dole, p. 16
pub const MAX_HE_IPP: f64 = 61000.0 * MMHG_TO_MILLIBARS; 
pub const MAX_NE_IPP: f64 = 3900.0 * MMHG_TO_MILLIBARS;   
pub const MAX_N2_IPP: f64 = 2330.0 * MMHG_TO_MILLIBARS;   
pub const MAX_AR_IPP: f64 = 1220.0 * MMHG_TO_MILLIBARS;   
pub const MAX_KR_IPP: f64 = 350.0 * MMHG_TO_MILLIBARS;   
pub const MAX_XE_IPP: f64 = 160.0 * MMHG_TO_MILLIBARS;   
pub const MAX_CO2_IPP: f64 = 7.0 * MMHG_TO_MILLIBARS;   
pub const MAX_HABITABLE_PRESSURE: f64 = 118.0 * PSI_TO_MILLIBARS;   

/// The next gases are listed as poisonous in parts per million by volume at 1 atm:
pub const PPM_PRSSURE: f64 = EARTH_SURF_PRES_IN_MILLIBARS / 1000000.0;

/// Dole, p. 18
pub const MAX_F_IPP: f64 = 0.1 * PPM_PRSSURE;   
pub const MAX_CL_IPP: f64 = 1.0 * PPM_PRSSURE;  
pub const MAX_NH3_IPP: f64 = 100.0 * PPM_PRSSURE;  
pub const MAX_O3_IPP: f64 = 0.1 * PPM_PRSSURE;  
pub const MAX_CH4_IPP: f64 = 50000.0 * PPM_PRSSURE;

/// from Hart, eq.20
pub const EARTH_CONVECTION_FACTOR: f64 = 0.43;

/// Units of degrees Kelvin 
pub const FREEZING_POINT_OF_WATER: f64 = 273.15;

/// Average Earth Temperature
pub const EARTH_AVERAGE_CELSIUS: f64 = 14.0;
pub const EARTH_AVERAGE_KELVIN: f64 = EARTH_AVERAGE_CELSIUS + FREEZING_POINT_OF_WATER;

/// Earth days per Earth year
pub const DAYS_IN_A_YEAR: f64 = 365.256;

/// ratio of esc vel to RMS vel
pub const GAS_RETENTION_THRESHOLD: f64 = 6.0;

pub const ICE_ALBEDO: f64 = 0.7;
pub const CLOUD_ALBEDO: f64 = 0.52;

/// albedo of a gas giant 
pub const GAS_GIANT_ALBEDO: f64 = 0.5;
pub const AIRLESS_ICE_ALBEDO: f64 = 0.5;
pub const GREENHOUSE_TRIGGER_ALBEDO: f64 = 0.2;
pub const ROCKY_ALBEDO: f64 = 0.15;
pub const ROCKY_AIRLESS_ALBEDO: f64 = 0.07;
pub const WATER_ALBEDO: f64 = 0.04;

/// was .33 for a while
pub const EARTH_ALBEDO: f64 = 0.3;

pub const SECONDS_PER_HOUR: f64 = 3600.0;

/// number of cm in an AU 
pub const CM_PER_AU: f64 = 1.495978707e13;

/// number of cm in a km
pub const CM_PER_KM: f64 = 1.0e5;  
pub const KM_PER_AU: f64 = CM_PER_AU / CM_PER_KM;
pub const CM_PER_METER: f64 = 100.0;
pub const MILLIBARS_PER_BAR: f64 = 1000.0;

/// units of dyne cm2/gram2
pub const GRAV_CONSTANT: f64 = 6.672e-8;

/// units of g*m2/(sec2*K*mol)
pub const MOLAR_GAS_CONST: f64 = 8314.41;

/// Used in day-length calcs (cm2/sec2 g)
pub const J: f64 = 1.46e-19;
pub const INCREDIBLY_LARGE_NUMBER: f64 = 9.9999e37;

/// Binary stars produced by increasing coeff of dust density in cloud (Formation of Planetary Systems by Aggregation: A Computer Simulation by Stephen H. Dole)
pub const DUST_DENSITY_COEFF: f64 = 2.0E-3; 
pub const CLOUD_ECCENTRICITY: f64 = 0.25;
pub const EARTH_MASSES_PER_SOLAR_MASS: f64 = 332775.64;
pub const AIRLESS_ROCKY_ALBEDO: f64 = 0.07;
pub const KELVIN_CELCIUS_DIFFERENCE: f64 = 273.0;
pub const GREENHOUSE_EFFECT_CONST: f64 = 0.93;/// affects inner radius..
pub const RADIANS_PER_ROTATION: f64 = 2.0 * PI;

///  Now for a few molecular weights (used for RMS velocity calcs):
///  This table is from Dole's book "Habitable Planets for Man", p. 38 
/// H  
pub const ATOMIC_HYDROGEN: f64 = 1.0; 

/// H2 
pub const MOL_HYDROGEN: f64 = 2.0; 

/// He 
pub const HELIUM: f64 = 4.0; 

/// N  
pub const ATOMIC_NITROGEN: f64 = 14.0; 

/// O  
pub const ATOMIC_OXYGEN: f64 = 16.0; 

/// CH4
pub const METHANE: f64 = 16.0; 

/// NH3
pub const AMMONIA: f64 = 17.0; 

/// H2O
pub const WATER_VAPOR: f64 = 18.0; 

/// Ne 
pub const NEON: f64 = 20.2; 

/// N2 
pub const MOL_NITROGEN: f64 = 28.0; 

/// CO 
pub const CARBON_MONOXIDE: f64 = 28.0; 

/// NO 
pub const NITRIC_OXIDE: f64 = 30.0; 

/// O2 
pub const MOL_OXYGEN: f64 = 32.0; 

/// H2S
pub const HYDROGEN_SULPHIDE: f64 = 34.1; 

/// Ar 
pub const ARGON: f64 = 39.9; 

/// CO2
pub const CARBON_DIOXIDE: f64 = 44.0; 

/// N2O
pub const NITROUS_OXIDE: f64 = 44.0; 

/// NO2
pub const NITROGEN_DIOXIDE: f64 = 46.0; 

/// O3 
pub const OZONE: f64 = 48.0; 

/// SO2
pub const SULPH_DIOXIDE: f64 = 64.1; 

/// SO3
pub const SULPH_TRIOXIDE: f64 = 80.1; 

/// Kr 
pub const KRYPTON: f64 = 83.8; 

/// Xe 
pub const XENON: f64 = 131.3; 

/// H2
pub const MOLECULAR_HYDROGEN: f64 = 2.0; 

/// N2
pub const MOLECULAR_NITROGEN: f64 = 28.0; 

/// O2
pub const MOLECULAR_OXYGEN: f64 = 32.0; 

///  And atomic numbers, for use in ChemTable indexes
pub const AN_H: f64 = 1.0;
pub const AN_HE: f64 = 2.0;
pub const AN_N: f64 = 7.0;
pub const AN_O: f64 = 8.0;
pub const AN_F: f64 = 9.0;
pub const AN_NE: f64 = 10.0;
pub const AN_P: f64 = 15.0;
pub const AN_CL: f64 = 17.0;
pub const AN_AR: f64 = 18.0;
pub const AN_BR: f64 = 35.0;
pub const AN_KR: f64 = 36.0;
pub const AN_I: f64 = 53.0;
pub const AN_XE: f64 = 54.0;
pub const AN_HG: f64 = 80.0;
pub const AN_AT: f64 = 85.0;
pub const AN_RN: f64 = 86.0;
pub const AN_FR: f64 = 87.0;
pub const AN_NH3: f64 = 900.0;
pub const AN_H2O: f64 = 901.0;
pub const AN_CO2: f64 = 902.0;
pub const AN_O3: f64 = 903.0;
pub const AN_CH4: f64 = 904.0;
pub const AN_CH3CH2OH: f64 = 905.0;

/// The following defines are used in the kothari_radius function
/// All units are in cgs system ie: cm, g, dynes, etc.
pub const A1_20: f64 = 6.485e12; 
pub const A2_20: f64 = 4.0032e-8;
pub const BETA_20: f64 = 5.71e12;

pub const JIMS_FUDGE: f64 = 1.004;

/// The following defines are used in determining the fraction of a planet covered with clouds in function cloud_fraction in file
/// grams
pub const Q1_36: f64 = 1.258e19; 
/// 1/Kelvin
pub const Q2_36: f64 = 0.0698;
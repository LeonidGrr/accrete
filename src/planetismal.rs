use crate::consts::PROTOPLANET_MASS;
use crate::utils::*;
use rand::prelude::*;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Planetismal {
    // Axis in AU
    pub a: f64,
    // eccentricity of the orbit
    pub e: f64,
    // mass (in solar masses)
    pub mass: f64,
    // if the planet is a gas giant
    pub gas_giant: bool,
    // the 'zone' of the planet
    pub orbit_zone: i32,
    // equatorial radius (in km)
    pub radius: f64,
    // density (in g/cc)
    pub density: f64,
    // length of the local year (days)
    pub orbital_period: f64,
    // length of the local day (hours)
    pub day: f64,
    // TRUE if in resonant rotation
    pub resonant_period: bool,
    // units of degrees
    pub axial_tilt: f64,
    // units of cm/sec
    pub escape_velocity: f64,
    // units of cm/sec2
    pub surface_accel: f64,
    // units of Earth gravities
    pub surface_grav: f64,
    // units of cm/sec
    pub rms_velocity: f64,
    // smallest molecular weight retained
    pub molecule_weight: f64,
    pub volatile_gas_inventory: f64,
    // units of millibars (mb)
    pub surface_pressure: f64,
    // runaway greenhouse effect?
    pub greenhouse_effect: bool,
    // the boiling point of water (Kelvin)
    pub boil_point: f64,
    // albedo of the planet
    pub albedo: f64,
    // surface temperature in Kelvin
    pub surface_temp: f64,
    // fraction of surface covered
    pub hydrosphere: f64,
    // fraction of surface covered
    pub cloud_cover: f64,
    // fraction of surface covered
    pub ice_cover: f64,
    pub moons: Vec<Planetismal>,
    /// Display info
    pub earth_mass: f64,
    pub smallest_molecular_weight: String,
    pub boiling_point_celsium: f64,
    pub surface_pressure_bar: f64,
    pub surface_temp_celsium: f64,
    pub hydrosphere_percentage: f64,
    pub cloud_cover_percentage: f64,
    pub ice_cover_percentage: f64,
    pub length_of_year: f64,
    pub escape_velocity_km_per_sec: f64
}

impl Planetismal {
    pub fn new(
        planetesimal_inner_bound: &f64,
        planetesimal_outer_bound: &f64,
        cloud_eccentricity: &f64,
    ) -> Self {
        let mut rng = rand::thread_rng();
        let gas_giant = false;
        let a = rng.gen_range(planetesimal_inner_bound, planetesimal_outer_bound);
        let e = random_eccentricity(rng.gen_range(0.0, 1.0), cloud_eccentricity);

        Planetismal {
            a,
            e,
            mass: PROTOPLANET_MASS,
            gas_giant,
            orbit_zone: 0,
            radius: 0.0,
            density: 0.0,
            orbital_period: 0.0,
            day: 0.0,
            resonant_period: false,
            axial_tilt: 0.0,
            escape_velocity: 0.0,
            surface_accel: 0.0,
            surface_grav: 0.0,
            rms_velocity: 0.0,
            molecule_weight: 0.0,
            volatile_gas_inventory: 0.0,
            surface_pressure: 0.0,
            greenhouse_effect: false,
            boil_point: 0.0,
            albedo: 0.0,
            surface_temp: 0.0,
            hydrosphere: 0.0,
            cloud_cover: 0.0,
            ice_cover: 0.0,
            moons: Vec::new(),
            earth_mass: 0.0,
            smallest_molecular_weight: String::new(),
            boiling_point_celsium: 0.0,
            surface_pressure_bar: 0.0,
            surface_temp_celsium: 0.0,
            hydrosphere_percentage: 0.0,
            cloud_cover_percentage: 0.0,
            ice_cover_percentage: 0.0,
            length_of_year: 0.0,
            escape_velocity_km_per_sec: 0.0,
        }
    }
}

fn random_eccentricity(random: f64, cloud_eccentricity: &f64) -> f64 {
    1.0 - random.powf(*cloud_eccentricity)
}

pub fn coalesce_planetismals(planets: &mut Vec<Planetismal>, cloud_eccentricity: &f64) {
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
                            outer_effect_limit(&p.a, &p.e, &p.mass, cloud_eccentricity) - p.a;
                        let dist2 = prev_p.a
                            - inner_effect_limit(
                                &prev_p.a,
                                &prev_p.e,
                                &prev_p.mass,
                                cloud_eccentricity,
                            );
                        (dist1, dist2)
                    }
                    false => {
                        let dist1 =
                            p.a - inner_effect_limit(&p.a, &p.e, &p.mass, cloud_eccentricity);
                        let dist2 = outer_effect_limit(
                            &prev_p.a,
                            &prev_p.e,
                            &prev_p.mass,
                            cloud_eccentricity,
                        ) - prev_p.a;
                        (dist1, dist2)
                    }
                };

                if dist.abs() < dist1.abs() || dist.abs() < dist2.abs() {
                    *prev_p = coalesce_two_planets(&prev_p, &p);
                } else {
                    next_planets.push(p.clone());
                }
            }
        }
    }
    *planets = next_planets;
}

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


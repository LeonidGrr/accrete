use crate::consts::*;
use crate::utils::*;
use rand::prelude::*;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Planetismal {
    pub a: f64,    /* semi-major axis of the orbit (in AU)*/
    pub e: f64,    /* eccentricity of the orbit	     */
    pub mass: f64, /* mass (in solar masses)	     */
    pub gas_giant: bool, /* if the planet is a gas giant */
                   // pub orbit_zone: i32, /* the 'zone' of the planet          */
                   // pub radius: f64, /* equatorial radius (in km)	     */
                   // pub density: f64, /* density (in g/cc)		     */
                   // pub orbital_period: f64, /* length of the local year (days)   */
                   // pub day: f64, /* length of the local day (hours)   */
                   // pub resonant_period: f64, /* TRUE if in resonant rotation   */
                   // pub axial_tilt: f64, /* units of degrees		     */
                   // pub escape_velocity: f64, /* units of cm/sec		     */
                   // pub surface_accel: f64, /* units of cm/sec2		     */
                   // pub surface_grav: f64, /* units of Earth gravities	     */
                   // pub rms_velocity: f64, /* units of cm/sec		     */
                   // pub molecule_weight: f64, /* smallest molecular weight retained*/
                   // pub volatile_gas_inventory: f64,
                   // pub surface_pressure: f64, /* units of millibars (mb)	     */
                   // pub greenhouse_effect: f64, /* runaway greenhouse effect?	*/
                   // pub boil_point: f64, /* the boiling point of water (Kelvin)*/
                   // pub albedo: f64, /* albedo of the planet		     */
                   // pub surface_temp: f64, /* surface temperature in Kelvin     */
                   // pub hydrosphere: f64, /* fraction of surface covered	     */
                   // pub cloud_cover: f64, /* fraction of surface covered	     */
                   // pub ice_cover: f64, /* fraction of surface covered	     */
                   // pub moons: Vec<Planetismal>,
}

impl Planetismal {
    pub fn new(planetesimal_inner_bound: &f64, planetesimal_outer_bound: &f64) -> Self {
        let mut rng = rand::thread_rng();
        let gas_giant = false;
        let a = rng.gen_range(planetesimal_inner_bound, planetesimal_outer_bound);
        let e = random_eccentricity(rng.gen_range(0.0, 1.0));

        Planetismal {
            a,
            e,
            mass: PROTOPLANET_MASS,
            gas_giant,
        }
    }

    pub fn get_earth_mass(&self) -> f64 {
        self.mass * EARTH_MASSES_PER_SOLAR_MASS
    }
}

fn random_eccentricity(random: f64) -> f64 {
    1.0 - random.powf(ECCENTRICITY_COEFF)
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

// #[derive(Debug, PartialOrd, PartialEq, Clone)]
// pub struct AsteroidBelt {
//     pub axis: f64,
//     pub mass: f64,
// }

// impl AsteroidBelt {
//     pub fn new(
//         axis: f64,
//         mass: f64,
//     ) -> Self {
//         AsteroidBelt { axis, mass }
//     }

//     pub fn get_earth_mass(&self) -> f64 {
//         self.mass * EARTH_MASSES_PER_SOLAR_MASS
//     }
// }

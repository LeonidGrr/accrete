use crate::consts::PROTOPLANET_MASS;
use crate::consts::*;
use crate::enviro::*;
use crate::ring::*;
use crate::utils::*;
use rand::prelude::*;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Planetesimal {
    // axis, AU
    pub a: f64,
    // eccentricity of the orbit, unitless
    pub e: f64,
    pub gas_mass: f64,
    pub dust_mass: f64,
    pub distance_to_primary_star: f64,
    pub mass: f64,
    pub earth_masses: f64,
    pub gas_giant: bool,
    pub orbit_zone: i32,
    // equatorial radius, km
    pub radius: f64,
    pub earth_radii: f64,
    // density, g/cc
    pub density: f64,
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
    pub escape_velocity_km_per_sec: f64,
    pub orbital_period_days: f64,
    pub day_hours: f64,
    pub length_of_year: f64,
    pub molecule_weight: f64,
    pub smallest_molecular_weight: String,
    pub volatile_gas_inventory: f64,
    pub greenhouse_effect: bool,
    pub albedo: f64,
    pub is_tidally_locked: bool,
    pub surface_pressure_bar: f64,
    pub surface_temp_kelvin: f64,
    pub boiling_point_kelvin: f64,
    pub hydrosphere: f64,
    pub cloud_cover: f64,
    pub ice_cover: f64,
    pub moons: Vec<Planetesimal>,
    pub rings: Vec<Ring>,
    pub is_moon: bool,
    pub is_spherical: bool,
}

impl Planetesimal {
    pub fn new(
        planetesimal_inner_bound: &f64,
        planetesimal_outer_bound: &f64,
        cloud_eccentricity: &f64,
    ) -> Self {
        let mut rng = rand::thread_rng();
        let gas_giant = false;
        let a = rng.gen_range(planetesimal_inner_bound, planetesimal_outer_bound);
        let e = random_eccentricity(rng.gen_range(0.0, 1.0), cloud_eccentricity);

        Planetesimal {
            a,
            e,
            gas_mass: 0.0,
            dust_mass: 0.0,
            distance_to_primary_star: a,
            mass: PROTOPLANET_MASS,
            earth_masses: 0.0,
            gas_giant,
            orbit_zone: 0,
            radius: 0.0,
            earth_radii: 0.0,
            density: 0.0,
            orbital_period_days: 0.0,
            day_hours: 0.0,
            resonant_period: false,
            axial_tilt: 0.0,
            escape_velocity: 0.0,
            surface_accel: 0.0,
            surface_grav: 0.0,
            rms_velocity: 0.0,
            molecule_weight: 0.0,
            volatile_gas_inventory: 0.0,
            greenhouse_effect: false,
            albedo: 0.0,
            surface_temp_kelvin: 0.0,
            surface_pressure_bar: 0.0,
            boiling_point_kelvin: 0.0,
            hydrosphere: 0.0,
            cloud_cover: 0.0,
            ice_cover: 0.0,
            moons: Vec::new(),
            rings: Vec::new(),
            smallest_molecular_weight: String::new(),
            length_of_year: 0.0,
            escape_velocity_km_per_sec: 0.0,
            is_tidally_locked: false,
            is_moon: false,
            is_spherical: false,
        }
    }

    pub fn derive_planetary_environment(
        &mut self,
        stellar_luminosity: &f64,
        stellar_mass: &f64,
        main_seq_life: &f64,        
        ecosphere: &mut (f64, f64),
    ) {
        self.orbit_zone = orbital_zone(stellar_luminosity, self.a);
        if self.gas_giant {
            self.density = empirical_density(
                &self.mass,
                &self.distance_to_primary_star,
                &ecosphere.1,
                &self.gas_giant,
            );
            self.radius = volume_radius(&self.mass, &self.density);
        } else {
            self.radius = kothari_radius(&self.mass, &self.gas_giant, &self.orbit_zone);
            self.density = volume_density(&self.mass, &self.radius);
        }
        self.orbital_period_days = period(&self.a, &self.mass, &stellar_mass);
        self.day_hours = day_length(self, &stellar_mass, main_seq_life);
        self.axial_tilt = inclination(&self.a);
        self.escape_velocity = escape_vel(&self.mass, &self.radius);
        self.surface_accel = acceleration(&self.mass, &self.radius);
        self.rms_velocity = rms_vel(&MOLECULAR_NITROGEN, &self.a);
        self.molecule_weight = molecule_limit(&self.mass, &self.radius);

        if self.gas_giant {
            self.surface_grav = INCREDIBLY_LARGE_NUMBER;
            self.greenhouse_effect = false;
            self.volatile_gas_inventory = INCREDIBLY_LARGE_NUMBER;
            self.surface_pressure_bar = INCREDIBLY_LARGE_NUMBER;
            self.boiling_point_kelvin = INCREDIBLY_LARGE_NUMBER;
            self.hydrosphere = INCREDIBLY_LARGE_NUMBER;
            self.albedo = about(GAS_GIANT_ALBEDO, 0.1);
            self.surface_temp_kelvin = INCREDIBLY_LARGE_NUMBER;
        } else {
            self.surface_grav = gravity(&self.surface_accel);
            self.greenhouse_effect = greenhouse(
                &self.distance_to_primary_star,
                &self.orbit_zone,
                &self.surface_pressure_bar,
                &ecosphere.1,
            );
            self.volatile_gas_inventory = vol_inventory(
                &self.mass,
                &self.escape_velocity,
                &self.rms_velocity,
                stellar_mass,
                &self.orbit_zone,
                &self.greenhouse_effect,
            );
            self.surface_pressure_bar = pressure(
                &self.volatile_gas_inventory,
                &self.radius,
                &self.surface_grav,
            );
            if self.surface_pressure_bar == 0.0 {
                self.boiling_point_kelvin = 0.0;
            } else {
                self.boiling_point_kelvin =
                    boiling_point_kelvin(&self.surface_pressure_bar);
                iterate_surface_temp(self, &ecosphere.1);
            }
        }

        self.earth_masses = get_earth_mass(self.mass);
        self.earth_radii = self.radius / EARTH_RADIUS_IN_KM;
        self.smallest_molecular_weight =
            get_smallest_molecular_weight(self.molecule_weight);
        self.length_of_year = self.orbital_period_days / 365.25;
        self.escape_velocity_km_per_sec = self.escape_velocity / CM_PER_KM;
        self.is_tidally_locked = check_tidal_lock(self.day_hours, self.orbital_period_days);
    }
}

/// Check planetesimal coalescence
pub fn coalesce_planetesimals(primary_star_luminosity: &f64, planets: &mut Vec<Planetesimal>, cloud_eccentricity: &f64) {
    let mut next_planets = Vec::new();
    for (i, p) in planets.iter().enumerate() {
        if i == 0 {
            next_planets.push(p.clone());
        } else {
            if let Some(prev_p) = next_planets.last_mut() {
                // Check if planetesimals have an over-lapping orbits
                if check_planetesimals_intersect(p, prev_p, cloud_eccentricity) {            
                    // Moon not likely to capture other moon
                    if p.is_moon {
                        *prev_p = coalesce_two_planets(&prev_p, &p);
                    } else {
                        // Check for larger/smaller planetesimal
                        let (mut larger, mut smaller) = match p.mass >= prev_p.mass {
                            true => (p.clone(), prev_p.clone()),
                            false => (prev_p.clone(), p.clone()),
                        };
                
                        // Recalculate current radius of bodies
                        larger.orbit_zone = orbital_zone(primary_star_luminosity, larger.distance_to_primary_star);
                        larger.radius = kothari_radius(&larger.mass, &larger.gas_giant, &larger.orbit_zone);

                        smaller.orbit_zone = orbital_zone(primary_star_luminosity, smaller.distance_to_primary_star);
                        smaller.radius = kothari_radius(&smaller.mass, &smaller.gas_giant, &smaller.orbit_zone);

                        let roche_limit = roche_limit_au(&larger.mass, &smaller.mass, &smaller.radius);
                        
                        // Planetesimals collide or one capture another
                        if (prev_p.a - p.a).abs() <= roche_limit {
                            *prev_p = coalesce_two_planets(&prev_p, &p);
                        } else {
                            *prev_p = capture_moon(&larger, &smaller);
                            prev_p.moons.sort_by(|p1, p2| p1.a.partial_cmp(&p2.a).unwrap());
                            coalesce_planetesimals(primary_star_luminosity, &mut prev_p.moons, cloud_eccentricity);
                            moons_to_rings(prev_p);
                        }
                    }
                } else {
                    next_planets.push(p.clone());
                }
            }
        }
    }
    *planets = next_planets;
}

/// Check planetesimal intersection
fn check_planetesimals_intersect(p: &Planetesimal, prev_p: &Planetesimal, cloud_eccentricity: &f64) -> bool {
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
    dist.abs() < dist1.abs() || dist.abs() < dist2.abs()
}

/// Two planetesimals collide and form one planet
pub fn coalesce_two_planets(a: &Planetesimal, b: &Planetesimal) -> Planetesimal {
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
fn capture_moon(larger: &Planetesimal, smaller: &Planetesimal) -> Planetesimal {
    let mut planet = larger.clone();
    let mut moon = smaller.clone();
    moon.is_moon = true;

    // Recalcualte planetary axis
    let new_mass = planet.mass + moon.mass;
    let new_axis = new_mass / (planet.mass / planet.a + moon.mass / moon.a);
    let term1 = planet.mass * (planet.a * (1.0 - planet.e.powf(2.0))).sqrt();
    let term2 = moon.mass * (moon.a * (1.0 - moon.e.powf(2.0))).sqrt();
    let term3 = (term1 + term2) / (new_mass * new_axis.sqrt());
    let term4 = 1.0 - term3.powf(2.0);
    let new_eccn = term4.abs().sqrt();
    planet.a = new_axis;
    planet.e = new_eccn;
    planet.distance_to_primary_star = new_axis;

    // Add moon to planetary moons, recalculate disturbed orbits of moons
    let mut rng = rand::thread_rng();
    planet.moons.append(&mut moon.moons);
    planet.moons.push(moon);
    let planet_outer_moon = 4.0 * &planet.mass.powf(0.33);

    for m in planet.moons.iter_mut() {
        let _hill_sphere = hill_sphere_au(
            &planet.a,
            &planet.e,
            &planet.mass,
            &m.mass,
        );
        m.a = rng.gen_range(0.0, planet_outer_moon);
        m.distance_to_primary_star = planet.a;
    }

    planet
}

fn moons_to_rings(planet: &mut Planetesimal) {
    let mut next_moons = Vec::new();

    for m in planet.moons.iter_mut() {
        let roche_limit = roche_limit_au(
            &planet.mass,
            &m.mass,
            &m.radius,
        );
        if m.a <= roche_limit {
            let ring = Ring::new(roche_limit, m);
            planet.rings.push(ring);
        } else {
            next_moons.push(m.clone());
        }
    }

    planet.moons = next_moons;
}

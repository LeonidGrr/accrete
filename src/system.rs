use crate::consts::PROTOPLANET_MASS;
use crate::consts::*;
/// BIBLIOGRAPHY
/// Dole, Stephen H.
/// "Formation of Planetary Systems by Aggregation: a Computer Simulation"
/// October 1969, Rand  Corporation Paper P-4226.
use crate::dust::*;
use crate::enviro::*;
use crate::planetismal::*;
use crate::utils::*;
use rand::prelude::*;

#[derive(Debug, Clone)]
pub struct System {
    pub with_moons: bool,
    pub with_rings: bool,
    pub stellar_mass: f64,
    pub stellar_luminosity: f64,
    pub main_seq_life: f64,
    pub age: f64,
    pub ecosphere: (f64, f64),
    pub planets: Vec<Planetismal>,
    pub cloud_eccentricity: f64,
    pub planetismal_inner_bound: f64,
    pub planetismal_outer_bound: f64,
    pub dust_density_coeff: f64,
    pub planets_limit: Option<usize>,
    pub k: f64,
    pub b: f64,
}

impl System {
    pub fn set_initial_conditions(
        planets_limit: Option<usize>,
        stellar_mass: f64,
        dust_density_coeff: f64,
        k: f64,
        cloud_eccentricity: f64,
        b: f64,
        _with_moons: bool,
    ) -> Self {
        let stellar_luminosity = luminosity(&stellar_mass);
        let planetismal_inner_bound = innermost_planet(&stellar_mass);
        let planetismal_outer_bound = outermost_planet(&stellar_mass);

        let main_seq_life = main_sequence_age(&stellar_mass, &stellar_luminosity);

        let mut rng = rand::thread_rng();
        let age = match main_seq_life >= 6.0E9 {
            true => rng.gen_range(1.0E9, 6.0E9),
            false => rng.gen_range(1.0E9, main_seq_life),
        };
        let ecosphere = ecosphere(&stellar_luminosity);

        Self {
            stellar_mass,
            stellar_luminosity,
            main_seq_life,
            age,
            ecosphere,
            with_moons: false,
            with_rings: false,
            planets: Vec::new(),
            k,
            b,
            dust_density_coeff,
            planets_limit,
            cloud_eccentricity,
            planetismal_inner_bound,
            planetismal_outer_bound,
        }
    }

    pub fn distribute_planetary_masses(&mut self) {
        let Self {
            stellar_mass,
            stellar_luminosity,
            planets,
            k,
            b,
            dust_density_coeff,
            planets_limit,
            cloud_eccentricity,
            planetismal_inner_bound,
            planetismal_outer_bound,
            ..
        } = self;
        let inner_dust = 0.0;
        let outer_dust = stellar_dust_limit(&stellar_mass);
        let dust_band = DustBand::new(outer_dust, inner_dust, true, true);
        let mut dust_bands = Vec::new();
        dust_bands.push(dust_band);
        let mut dust_left = true;

        while dust_left {
            let mut p = Planetismal::new(
                &planetismal_inner_bound,
                &planetismal_outer_bound,
                &cloud_eccentricity,
            );

            let inside_range = inner_effect_limit(&p.a, &p.e, &p.mass, &cloud_eccentricity);
            let outside_range = outer_effect_limit(&p.a, &p.e, &p.mass, &cloud_eccentricity);

            if dust_availible(&dust_bands, &inside_range, &outside_range) {
                let dust_density = dust_density(&dust_density_coeff, &stellar_mass, &p.a);
                let crit_mass = critical_limit(&b, &p.a, &p.e, &stellar_luminosity);
                accrete_dust(
                    &mut p,
                    &mut dust_bands,
                    &crit_mass,
                    &dust_density,
                    &cloud_eccentricity,
                    &k,
                );

                let min = inner_effect_limit(&p.a, &p.e, &p.mass, &cloud_eccentricity);
                let max = outer_effect_limit(&p.a, &p.e, &p.mass, &cloud_eccentricity);
                update_dust_lanes(&mut dust_bands, min, max, &p.mass, &crit_mass);
                compress_dust_lanes(&mut dust_bands);

                if p.mass != 0.0 && p.mass != PROTOPLANET_MASS {
                    if p.mass > crit_mass {
                        p.gas_giant = true;
                    }
                    planets.push(p);
                    planets.sort_by(|p1, p2| p1.a.partial_cmp(&p2.a).unwrap());
                    coalesce_planetismals(planets, &cloud_eccentricity);
                } else {
                    // belt?
                    // console.debug(sprintf(".. failed due to large neighbor.\n"));
                }
            }

            let dust_still_left = dust_availible(
                &dust_bands,
                &planetismal_inner_bound,
                &planetismal_outer_bound,
            );

            dust_left = match planets_limit {
                Some(limit) => planets.len() < *limit && dust_still_left,
                None => dust_still_left,
            };
        }
    }

    pub fn generate_planetary_environment(&mut self) {
        for planet in self.planets.iter_mut() {
            planet.orbit_zone = orbital_zone(&self.stellar_luminosity, &planet.a);
            if planet.gas_giant {
                planet.density = empirical_density(
                    &planet.mass,
                    &planet.a,
                    &self.ecosphere.1,
                    &planet.gas_giant,
                );
                planet.radius = volume_radius(&planet.mass, &planet.density);
            } else {
                planet.radius = kothari_radius(&planet.mass, &planet.gas_giant, &planet.orbit_zone);
                planet.density = volume_density(&planet.mass, &planet.radius);
            }

            planet.orbital_period = period(&planet.a, &planet.mass, &self.stellar_mass);
            planet.day = day_length(planet, &self.stellar_mass, &self.main_seq_life);
            planet.axial_tilt = inclination(&planet.a);
            planet.escape_velocity = escape_vel(&planet.mass, &planet.radius);
            planet.surface_accel = acceleration(&planet.mass, &planet.radius);
            planet.rms_velocity = rms_vel(&MOLECULAR_NITROGEN, &planet.a);
            planet.molecule_weight = molecule_limit(&planet.mass, &planet.radius);

            if planet.gas_giant {
                planet.surface_grav = INCREDIBLY_LARGE_NUMBER;
                planet.greenhouse_effect = false;
                planet.volatile_gas_inventory = INCREDIBLY_LARGE_NUMBER;
                planet.surface_pressure = INCREDIBLY_LARGE_NUMBER;
                planet.boil_point = INCREDIBLY_LARGE_NUMBER;
                planet.hydrosphere = INCREDIBLY_LARGE_NUMBER;
                planet.albedo = about(GAS_GIANT_ALBEDO, 0.1);
                planet.surface_temp = INCREDIBLY_LARGE_NUMBER;
            } else {
                planet.surface_grav = gravity(&planet.surface_accel);
                planet.greenhouse_effect = greenhouse(
                    &planet.a,
                    &planet.orbit_zone,
                    &planet.surface_pressure,
                    &self.ecosphere.1,
                );
                planet.volatile_gas_inventory = vol_inventory(
                    &planet.mass,
                    &planet.escape_velocity,
                    &planet.rms_velocity,
                    &self.stellar_mass,
                    &planet.orbit_zone,
                    &planet.greenhouse_effect,
                );
                planet.surface_pressure = pressure(
                    &planet.volatile_gas_inventory,
                    &planet.radius,
                    &planet.surface_grav,
                );
                if planet.surface_pressure == 0.0 {
                    planet.boil_point = 0.0;
                } else {
                    planet.boil_point = boiling_point(&planet.surface_pressure);
                    iterate_surface_temp(planet, &self.ecosphere.1);
                }
            }
        }
    }
}

/// BIBLIOGRAPHY
/// Dole, Stephen H.
/// "Formation of Planetary Systems by Aggregation: a Computer Simulation"
/// October 1969, Rand  Corporation Paper P-4226.
use crate::dust::*;
use crate::enviro::*;
use crate::planetismal::*;
use crate::utils::*;
use crate::consts::PROTOPLANET_MASS;

#[derive(Debug, Clone)]
pub struct System {
    pub with_moons: bool,
    pub with_rings: bool,
    pub stellar_mass: f64,
    pub stellar_luminosity: f64,
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
        with_moons: bool,
    ) -> Self {
        let stellar_luminosity = luminosity(&stellar_mass);
        let planetismal_inner_bound = innermost_planet(&stellar_mass);
        let planetismal_outer_bound = outermost_planet(&stellar_mass);

        Self {
            stellar_mass,
            stellar_luminosity,
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
                    planets
                        .sort_by(|p1, p2| p1.a.partial_cmp(&p2.a).unwrap());
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
}

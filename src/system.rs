/// BIBLIOGRAPHY
/// Dole, Stephen H.
/// "Formation of Planetary Systems by Aggregation: a Computer Simulation"
/// October 1969, Rand  Corporation Paper P-4226.
use crate::consts::*;
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
    pub planets: Vec<Planetismal>,
    pub cloud_eccentricity: f64,
    pub planetismal_inner_bound: f64,
    pub planetismal_outer_bound: f64,
}

impl System {
    pub fn set_initial_conditions() -> Self {
        let mut rng = rand::thread_rng();
        let stellar_mass = rng.gen_range(0.6, 1.3);
        let stellar_luminosity = luminosity(&stellar_mass);
        let planetismal_inner_bound = innermost_planet(&stellar_mass);
        let planetismal_outer_bound = outermost_planet(&stellar_mass);

        Self {
            stellar_mass,
            stellar_luminosity,
            with_moons: false,
            with_rings: false,
            planets: Vec::new(),
            /// Eccentricity of dust cloud 0.15-0.25
            cloud_eccentricity: 0.2,
            planetismal_inner_bound,
            planetismal_outer_bound,
        }
    }

    pub fn distribute_planetary_masses(&mut self) {
    Let mut planetary_system = self;
    let inner_dust = 0.0;
    let outer_dust = stellar_dust_limit(&planetary_system.stellar_mass);
    let dust_band = DustBand::new(outer_dust, inner_dust, true, true);
    let mut dust_bands = Vec::new();
    dust_bands.push(dust_band);
    let mut dust_left = true;

    while dust_left {
        let mut p = Planetismal::new(
            &planetary_system.planetismal_inner_bound,
            &planetary_system.planetismal_outer_bound,
        );

        let inside_range = inner_effect_limit(&p.a, &p.e, &p.mass, &planetary_system.cloud_eccentricity);
        let outside_range = outer_effect_limit(&p.a, &p.e, &p.mass, &planetary_system.cloud_eccentricity);

        if dust_availible(&dust_bands, &inside_range, &outside_range) {
            let dust_density = dust_density(&planetary_system.stellar_mass, &p.a);
            let crit_mass = critical_limit(&p.a, &p.e, &planetary_system.stellar_luminosity);
            accrete_dust(
                &mut p,
                &mut dust_bands,
                &crit_mass,
                &dust_density,
                &planetary_system.cloud_eccentricity,
            );

            let min = inner_effect_limit(&p.a, &p.e, &p.mass, &planetary_system.cloud_eccentricity);
            let max = outer_effect_limit(&p.a, &p.e, &p.mass, &planetary_system.cloud_eccentricity);
            update_dust_lanes(&mut dust_bands, min, max, &p.mass, &crit_mass);

            compress_dust_lanes(&mut dust_bands);

            if p.mass != 0.0 && p.mass != PROTOPLANET_MASS {
                if p.mass > crit_mass {
                    p.gas_giant = true;
                }
                planetary_system.planets.push(p);
                planetary_system
                    .planets
                    .sort_by(|p1, p2| p1.a.partial_cmp(&p2.a).unwrap());
                coalesce_planetismals(&mut planetary_system.planets, &planetary_system.cloud_eccentricity);
            } else {
                // belt?
                // console.debug(sprintf(".. failed due to large neighbor.\n"));
            }
        }

        dust_left = dust_availible(
            &dust_bands,
            &planetary_system.planetismal_inner_bound,
            &planetary_system.planetismal_outer_bound,
        );
    }
}

}

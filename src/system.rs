use crate::consts::*;
use crate::dust::*;
use crate::planetesimal::*;
use crate::utils::*;

#[derive(Debug, Clone)]
pub struct PrimaryStar {
    pub stellar_mass: f64,
    pub stellar_luminosity: f64,
    pub stellar_surface_temp: f64,
    pub stellar_radius_au: f64,
    pub spectral_class: SpectralClass,
    pub bv_color_index: f64,
    pub color: String,
    pub main_seq_life: f64,
    // pub age: f64,
    pub ecosphere: (f64, f64),
    pub planets: Vec<Planetesimal>,
    pub cloud_eccentricity: f64,
    pub dust_density_coeff: f64,
    pub planets_limit: Option<usize>,
    pub k: f64,
    pub b: f64,
    pub has_nebulae: bool,
}

impl PrimaryStar {
    pub fn set_initial_conditions(
        planets_limit: Option<usize>,
        stellar_mass: f64,
        dust_density_coeff: f64,
        k: f64,
        cloud_eccentricity: f64,
        b: f64,
    ) -> Self {
        let stellar_luminosity = luminosity(stellar_mass);
        let main_seq_life = main_sequence_age(stellar_mass, stellar_luminosity);
        let stellar_radius_au = stellar_radius_au(stellar_mass);
        let stellar_surface_temp = stellar_surface_temp(stellar_radius_au, stellar_luminosity);
        let spectral_class = spectral_class(&stellar_surface_temp);
        let bv_color_index = bv_color_index(stellar_surface_temp);
        let color = bv_to_rgb(bv_color_index);
        let ecosphere = ecosphere(&stellar_luminosity, &spectral_class);

        Self {
            stellar_mass,
            stellar_luminosity,
            main_seq_life,
            // age,
            ecosphere,
            planets: Vec::new(),
            k,
            b,
            dust_density_coeff,
            planets_limit,
            cloud_eccentricity,
            stellar_surface_temp,
            stellar_radius_au,
            spectral_class,
            bv_color_index,
            color,
            has_nebulae: false,
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
            ..
        } = self;
        let planetesimal_inner_bound = innermost_planet(stellar_mass);
        let planetesimal_outer_bound = outermost_planet(stellar_mass);

        let inner_dust = 0.0;
        let outer_dust = stellar_dust_limit(&stellar_mass);
        let dust_band = DustBand::new(outer_dust, inner_dust, true, true);
        let mut dust_bands = Vec::new();
        dust_bands.push(dust_band);
        let mut dust_left = true;

        while dust_left {
            let mut p = Planetesimal::new(
                &planetesimal_inner_bound,
                &planetesimal_outer_bound,
                &cloud_eccentricity,
            );

            let inside_range = inner_effect_limit(&p.a, &p.e, &p.mass, &cloud_eccentricity);
            let outside_range = outer_effect_limit(&p.a, &p.e, &p.mass, &cloud_eccentricity);

            if dust_availible(&dust_bands, &inside_range, &outside_range) {
                let dust_density = dust_density(&dust_density_coeff, &stellar_mass, &p.a);
                let crit_mass = critical_limit(&b, &p.a, &p.e, &stellar_luminosity);

                let mut gas_mass = 0.0;
                let mut dust_mass = 0.0;
                accrete_dust(
                    &mut p.mass,
                    &mut dust_mass,
                    &mut gas_mass,
                    &mut p.a,
                    &mut p.e,
                    &crit_mass,
                    &mut dust_bands,
                    &cloud_eccentricity,
                    &dust_density,
                    k,
                );

                let min = inner_effect_limit(&p.a, &p.e, &p.mass, &cloud_eccentricity);
                let max = outer_effect_limit(&p.a, &p.e, &p.mass, &cloud_eccentricity);

                update_dust_lanes(&mut dust_bands, min, max, &p.mass, &crit_mass);
                compress_dust_lanes(&mut dust_bands);

                if p.mass > PROTOPLANET_MASS {
                    if p.mass > crit_mass {
                        p.gas_giant = true;
                    }
                    planets.push(p);
                    planets.sort_by(|p1, p2| p1.a.partial_cmp(&p2.a).unwrap());
                    coalesce_planetesimals(stellar_luminosity, planets, &cloud_eccentricity);
                } else {
                    println!("Belt??");
                }
            }

            let dust_still_left = dust_availible(
                &dust_bands,
                &planetesimal_inner_bound,
                &planetesimal_outer_bound,
            );

            dust_left = match planets_limit {
                Some(limit) => planets.len() < *limit && dust_still_left,
                None => dust_still_left,
            };
        }
    }

    pub fn process_planets(&mut self) {
        let PrimaryStar {
            stellar_luminosity,
            stellar_mass,
            main_seq_life,
            ecosphere,
            planets,
            ..
        } = self;
        for planet in planets.iter_mut() {
            planet.derive_planetary_environment(
                stellar_luminosity,
                stellar_mass,
                main_seq_life,
                ecosphere,
            );

            // for moon in planet.moons.iter_mut() {
            //     moon.derive_planetary_environment(
            //         stellar_luminosity,
            //         stellar_mass,
            //         main_seq_life,
            //         ecosphere,
            //     );
            // }
        }
    }
}

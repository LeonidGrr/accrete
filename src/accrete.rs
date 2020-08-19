use crate::astro;
use crate::dole_params;
use crate::dust::{DustBand, DustBands};
use crate::planetismal::Planetismal;
use crate::asteroid_belt::AsteroidBelt;

use rand::prelude::*;
use std::f64::consts::PI;

pub struct Accrete {
    with_moons: bool,
    with_rings: bool,
}

impl Accrete {
    pub fn new(with_moons: bool, with_rings: bool) -> Self {
        Accrete { with_moons, with_rings }
    }

    pub fn distribute_planets(
        &self,
        mass: Option<f64>,
        luminosity: Option<f64>,
    ) -> (Vec<Planetismal>, f64, f64) {
        let stellar_mass = mass.unwrap_or(1.0);
        let stellar_luminosity = luminosity.unwrap_or(astro::luminosity(stellar_mass));
        let mut rng = rand::thread_rng();

        let mut planets = Vec::new();
        // let mut asteroid_belts = Vec::new();
        let mut dust_left = true;
        let mut dust_bands = DustBands::new(
            dole_params::inner_dust_limit(&stellar_mass),
            dole_params::outer_dust_limit(&stellar_mass),
        );

        while dust_left {
            let a = rng.gen_range(0.0, 1.0);
            let e = rng.gen_range(0.0, 1.0);
            let mut p = Planetismal::new(
                Some(
                    a * dole_params::outermost_planet(&stellar_mass)
                        + dole_params::innermost_planet(&stellar_mass),
                ),
                Some(dole_params::random_eccentricity(e)),
                None,
                None,
                None,
            );

            let dust_density = dole_params::dust_density(&stellar_mass, &p.axis);
            let critical_mass = dole_params::critical_mass(&p.axis, &p.eccn, &stellar_luminosity);
            let mass =
                Accrete::accrete_dust(&mut p, &mut dust_bands.bands, critical_mass, dust_density);

            if mass > 0.0 {
                if mass > astro::PROTOPLANET_MASS {
                    if mass >= critical_mass {
                        p.gas_giant = true;
                    }

                    dust_bands.update_lanes(
                        dole_params::inner_swept_limit(&p.mass, &p.axis, &p.eccn),
                        dole_params::outer_swept_limit(&p.mass, &p.axis, &p.eccn),
                        &p.gas_giant,
                    );

                    dust_left = dust_bands.dust_remaining(
                        dole_params::innermost_planet(&stellar_mass),
                        dole_params::outermost_planet(&stellar_mass),
                    );

                    dust_bands.compress_lanes();

                    if self.with_moons {
                        p.moons = Accrete::distribute_moons(self, p.mass, stellar_luminosity);
                    }

                    planets.push(p);

                    planets.sort_by(|p1, p2| p1.axis.partial_cmp(&p2.axis).unwrap());
        
                    Accrete::coalesce_planetismals(&mut planets);
                }
            }
        }

        (planets, stellar_mass, stellar_luminosity)
    }

    fn distribute_moons(
        &self,
        planetary_mass: f64,
        stellar_luminosity: f64,
    ) -> Vec<Planetismal> {
        let mut rng = rand::thread_rng();
        let mut moons = Vec::new();
        let mut dust_left = true;
        let mut dust_bands = DustBands::new(0.0, dole_params::planet_outer_dust_limit(&planetary_mass));

        while dust_left {
            let a = rng.gen_range(0.0, 1.0);
            let e = rng.gen_range(0.0, 1.0);
            
            let mut p = Planetismal::new(
                Some(
                    a * dole_params::outermost_moon(&planetary_mass)
                        + dole_params::innermost_planet(&planetary_mass),
                ),
                Some(dole_params::random_eccentricity(e)),
                None,
                None,
                None,
            );

            let dust_density = dole_params::dust_density(&planetary_mass, &p.axis);
            let critical_mass = dole_params::critical_mass(&p.axis, &p.eccn, &stellar_luminosity);
            let mass =
                Accrete::accrete_dust(&mut p, &mut dust_bands.bands, critical_mass, dust_density);

            if mass != 0.0 && mass != astro::PROTOMOON_MASS {
                if mass >= critical_mass {
                    p.gas_giant = true;
                }

                dust_bands.update_lanes(
                    0.0,
                    dole_params::planet_outer_swept_limit(&planetary_mass),
                    &false,
                );

                dust_left = dust_bands.dust_remaining(
                    dole_params::innermost_planet(&planetary_mass),
                    dole_params::outermost_planet(&planetary_mass),
                );

                dust_bands.compress_lanes();
                
                moons.push(p);

                moons.sort_by(|p1, p2| p1.axis.partial_cmp(&p2.axis).unwrap());

                Accrete::coalesce_planetismals(&mut moons);
            }
        }
        
        moons
    }

    fn form_asteroid_belt(
        asterois_belts: &Vec<AsteroidBelt>,
        mass: &f64,
        eccn: f64,
    ) -> Vec<AsteroidBelt> {
        Vec::new()
    }

    fn accrete_dust(
        nucleus: &mut Planetismal,
        bands: &mut Vec<DustBand>,
        critical_mass: f64,
        dust_density: f64,
    ) -> f64 {
        let mut new_mass = nucleus.mass;

        loop {
            nucleus.mass = new_mass;
            new_mass = 0.0;

            for b in bands.iter() {
                new_mass += Accrete::collect_dust(nucleus, &b, critical_mass, dust_density);
            }

            if !(new_mass - nucleus.mass > 0.0001 * nucleus.mass) {
                break;
            }
        }

        nucleus.mass = new_mass;
        nucleus.mass
    }

    pub fn collect_dust(
        nucleus: &mut Planetismal,
        b: &DustBand,
        critical_mass: f64,
        dust_density: f64,
    ) -> f64 {
        let Planetismal {
            mass, axis, eccn, ..
        } = nucleus;
        let mut swept_inner = dole_params::inner_swept_limit(mass, axis, eccn);
        let swept_outer = dole_params::outer_swept_limit(mass, axis, eccn);

        if swept_inner < 0.0 {
            swept_inner = 0.0;
        }

        if b.outer <= swept_inner || b.inner >= swept_outer || !b.dust {
            return 0.0;
        }

        let mass_density = dole_params::mass_density(&dust_density, &critical_mass, *&mass);
        let density = match !b.gas || *axis < critical_mass {
            true => dust_density,
            false => mass_density,
        };
        let swept_width = swept_outer - swept_inner;
        let outside = match swept_outer - b.outer > 0.0 {
            true => swept_outer - b.outer,
            false => 0.0,
        };
        let inside = match b.inner - swept_inner > 0.0 {
            true => b.inner - swept_inner,
            false => 0.0,
        };
        let width = swept_width - outside - inside;
        let term1 = 4.0 * PI * axis.powf(2.0);
        let term2 = 1.0 - *eccn * (outside - inside) / swept_width;
        let volume = term1 * dole_params::reduced_margin(mass) * width * term2;

        volume * density
    }

    fn coalesce_planetismals(planets: &mut Vec<Planetismal>) {
        *planets = planets
            .iter_mut()
            .enumerate()
            .fold(Vec::new(), |mut acc, (i, p)| {
                if i == 0 {
                    acc.push(p.clone());
                } else {
                    if let Some(prev_p) = acc.get_mut(i - 1) {
                        let dist = prev_p.axis - p.axis;

                        let (dist1, dist2) = match dist > 0.0 {
                            true => {
                                let dist1 =
                                    dole_params::outer_effect_limit(&p.mass, &p.axis, &p.eccn) - p.axis;
                                let dist2 = prev_p.axis
                                    - dole_params::inner_effect_limit(
                                        &prev_p.mass,
                                        &prev_p.axis,
                                        &prev_p.eccn,
                                    );
                                (dist1, dist2)
                            },
                            false => {
                                let dist1 =
                                    p.axis - dole_params::inner_effect_limit(&p.mass, &p.axis, &p.eccn);
                                let dist2 = dole_params::outer_effect_limit(
                                    &prev_p.mass,
                                    &prev_p.axis,
                                    &prev_p.eccn,
                                ) - prev_p.axis;
                                (dist1, dist2)
                            },
                        };

                        if dist.abs() <= dist1.abs() || dist.abs() <= dist2.abs() {
                            *prev_p = Accrete::coalesce_two_planets(&prev_p, p);
                        } else {
                            acc.push(p.clone());
                        }
                    }
                }
                acc
            });
    }

    fn coalesce_two_planets(a: &Planetismal, b: &Planetismal) -> Planetismal {
        println!("Collsion!");
        let new_mass = a.mass + b.mass;
        let new_axis = new_mass / (a.mass / a.axis + b.mass / b.axis);
        let term1 = a.mass * (a.axis * (1.0 - a.eccn.powf(2.0))).sqrt();
        let term2 = b.mass * (b.axis * (1.0 - b.eccn.powf(2.0))).sqrt();
        let term3 = (term1 + term2) / (new_mass * new_axis.sqrt());
        let term4 = 1.0 - term3.powf(2.0);
        let new_eccn = term4.abs().sqrt();
        let mut coalesced = a.clone();
        coalesced.mass = new_mass;
        coalesced.axis = new_axis;
        coalesced.eccn = new_eccn;
        coalesced.gas_giant = a.gas_giant || b.gas_giant;
        coalesced
    }
}

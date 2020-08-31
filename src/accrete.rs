/// BIBLIOGRAPHY
/// Dole, Stephen H.
/// "Formation of Planetary Systems by Aggregation: a Computer Simulation"
/// October 1969, Rand  Corporation Paper P-4226.
use crate::consts::*;
use crate::enviro::*;
use crate::structs::*;
use rand::prelude::*;

#[derive(Debug, Clone)]
pub struct Accrete {
    pub with_moons: bool,
    pub with_rings: bool,
    pub stellar_mass: f64,
    pub stellar_luminosity: f64,
    pub planets: Vec<Planetismal>,
    pub cloud_eccentricity: f64,
    pub planetismal_inner_bound: f64,
    pub planetismal_outer_bound: f64,
}

impl Accrete {
    fn set_initial_conditions() -> Self {
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
            cloud_eccentricity: 0.2,
            planetismal_inner_bound,
            planetismal_outer_bound,
        }
    }
}

/// Orbital radius is in AU, eccentricity is unitless, and the stellar luminosity ratio is with respect to the sun.
/// The value returned is the mass at which the planet begins to accrete gas as well as dust, and is in units of solar masses.
fn critical_limit(orbital_radius: &f64, eccentricity: &f64, stellar_luminosity_ratio: &f64) -> f64 {
    let perihelion_dist = orbital_radius - orbital_radius * eccentricity;
    let temp = perihelion_dist * stellar_luminosity_ratio.sqrt();
    B * temp.powf(-0.75)
}

fn dust_availible(dust_bands: &Vec<DustBand>, inside_range: &f64, outside_range: &f64) -> bool {
    dust_bands.iter().rev().fold(false, |mut acc, band| {
        if band.dust_present && &band.outer_edge > inside_range && &band.inner_edge < outside_range
        {
            acc = true;
        }
        acc
    })
}

fn accrete_dust(
    planetismal: &mut Planetismal,
    dust_bands: &mut Vec<DustBand>,
    crit_mass: &f64,
    dust_density: &f64,
    cloud_eccentricity: &f64,
) {
    let mut new_mass = planetismal.mass;

    loop {
        planetismal.mass = new_mass;
        new_mass = 0.0;

        for d in dust_bands.iter_mut() {
            new_mass += collect_dust(planetismal, crit_mass, d, cloud_eccentricity, dust_density);
        }

        if !(new_mass - planetismal.mass > 0.0001 * planetismal.mass) {
            break;
        }
    }
    planetismal.mass = new_mass;
}

fn collect_dust(
    p: &Planetismal,
    crit_mass: &f64,
    dust_band: &mut DustBand,
    cloud_eccentricity: &f64,
    dust_density: &f64,
) -> f64 {
    let Planetismal { mass, a, e, .. } = p;
    let mut temp = mass / (1.0 + mass);
    let reduced_mass = temp.powf(0.25);
    let mut r_inner = inner_effect_limit(a, e, mass, cloud_eccentricity);
    let r_outer = outer_effect_limit(a, e, mass, cloud_eccentricity);

    if r_inner < 0.0 {
        r_inner = 0.0;
    }

    if dust_band.outer_edge <= r_inner || dust_band.inner_edge >= r_outer {
        return 0.0;
    }

    let temp_density = match dust_band.dust_present == true {
        true => *dust_density,
        false => 0.0,
    };

    let mass_density = match mass < crit_mass || dust_band.gas_present {
        true => K * temp_density / (1.0 + (crit_mass / mass).sqrt() * (K - 1.0)),
        false => temp_density,
    };

    let bandwidth = r_outer - r_inner;

    let mut temp1 = r_outer - dust_band.outer_edge;
    if temp1 < 0.0 {
        temp1 = 0.0;
    }

    let mut width = bandwidth - temp1;

    let mut temp2 = dust_band.inner_edge - r_inner;
    if temp2 < 0.0 {
        temp2 = 0.0;
    }

    width = width - temp2;
    temp = 4.0 * PI * a.powf(2.0) * reduced_mass * (1.0 - e * (temp1 - temp2) / bandwidth);
    let volume = temp * width;
    volume * mass_density
}

fn update_dust_lanes(
    dust_bands: &mut Vec<DustBand>,
    min: f64,
    max: f64,
    mass: &f64,
    crit_mass: &f64,
) {
    let mut gas = true;
    if mass > crit_mass {
        gas = false;
    }
    *dust_bands = dust_bands.iter_mut().fold(Vec::new(), |mut acc, band| {
        let new_gas = band.gas_present && gas;
        if band.inner_edge < min && band.outer_edge > max {
            let mut inner = band.clone();
            inner.outer_edge = min;
            let middle = DustBand::new(max, min, false, new_gas);
            let outer = DustBand::new(band.outer_edge, max, band.dust_present, band.gas_present);
            acc.push(inner);
            acc.push(middle);
            acc.push(outer);
        } else if band.inner_edge < max && band.outer_edge > max {
            let outer = DustBand::new(band.outer_edge, max, band.dust_present, band.gas_present);
            let inner = DustBand::new(max, band.inner_edge, false, new_gas);
            acc.push(inner);
            acc.push(outer);
        } else if band.inner_edge < min && band.outer_edge > min {
            let outer = DustBand::new(band.outer_edge, min, false, new_gas);
            let inner = DustBand::new(min, band.inner_edge, band.dust_present, band.gas_present);
            acc.push(inner);
            acc.push(outer);
        } else if band.inner_edge >= min && band.outer_edge <= max {
            let dust_band = DustBand::new(band.outer_edge, band.inner_edge, false, new_gas);
            acc.push(dust_band)
        } else if band.outer_edge < min || band.inner_edge > max {
            acc.push(band.clone());
        }
        acc
    });
}

fn compress_dust_lanes(dust_bands: &mut Vec<DustBand>) {
    *dust_bands = dust_bands
        .iter()
        .enumerate()
        .fold(Vec::new(), |mut acc, (i, band)| {
            match dust_bands.get(i + 1) {
                Some(next_band) => {
                    if band.dust_present == next_band.dust_present
                        && band.gas_present == next_band.gas_present
                    {
                        let mut band = band.clone();
                        band.outer_edge = next_band.outer_edge;
                        acc.push(band);
                    } else {
                        acc.push(band.clone());
                    }
                }
                None => acc.push(band.clone()),
            }
            acc
        });
}

fn coalesce_planetismals(planets: &mut Vec<Planetismal>, cloud_eccentricity: &f64) {
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

fn coalesce_two_planets(a: &Planetismal, b: &Planetismal) -> Planetismal {
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

pub fn distribute_planetary_masses() -> Vec<Planetismal> {
    let mut accrete = Accrete::set_initial_conditions();
    let inner_dust = 0.0;
    let outer_dust = stellar_dust_limit(&accrete.stellar_mass);
    let dust_band = DustBand::new(outer_dust, inner_dust, true, true);
    let mut dust_bands = Vec::new();
    dust_bands.push(dust_band);
    let mut dust_left = true;

    while dust_left {
        let mut p = Planetismal::new(
            &accrete.planetismal_inner_bound,
            &accrete.planetismal_outer_bound,
        );

        let inside_range = inner_effect_limit(&p.a, &p.e, &p.mass, &accrete.cloud_eccentricity);
        let outside_range = outer_effect_limit(&p.a, &p.e, &p.mass, &accrete.cloud_eccentricity);

        if dust_availible(&dust_bands, &inside_range, &outside_range) {
            let dust_density = DUST_DENSITY_COEFF
                * accrete.stellar_mass.sqrt()
                * (-ALPHA * p.a.powf(1.0 / N)).exp();
            let crit_mass = critical_limit(&p.a, &p.e, &accrete.stellar_luminosity);
            accrete_dust(
                &mut p,
                &mut dust_bands,
                &crit_mass,
                &dust_density,
                &accrete.cloud_eccentricity,
            );

            let min = inner_effect_limit(&p.a, &p.e, &p.mass, &accrete.cloud_eccentricity);
            let max = outer_effect_limit(&p.a, &p.e, &p.mass, &accrete.cloud_eccentricity);
            update_dust_lanes(&mut dust_bands, min, max, &p.mass, &crit_mass);

            compress_dust_lanes(&mut dust_bands);

            if p.mass != 0.0 && p.mass != PROTOPLANET_MASS {
                if p.mass > crit_mass {
                    p.gas_giant = true;
                }
                accrete.planets.push(p);
                accrete
                    .planets
                    .sort_by(|p1, p2| p1.a.partial_cmp(&p2.a).unwrap());
                coalesce_planetismals(&mut accrete.planets, &accrete.cloud_eccentricity);
            } else {
                // belt?
                // console.debug(sprintf(".. failed due to large neighbor.\n"));
            }
        }

        dust_left = dust_availible(
            &dust_bands,
            &accrete.planetismal_inner_bound,
            &accrete.planetismal_outer_bound,
        );
    }

    accrete.planets
}

pub fn stellar_dust_limit(stellar_mass_ratio: &f64) -> f64 {
    200.0 * stellar_mass_ratio.powf(0.33)
}

fn innermost_planet(stellar_mass_ratio: &f64) -> f64 {
    0.3 * stellar_mass_ratio.powf(0.33)
}

fn outermost_planet(stellar_mass_ratio: &f64) -> f64 {
    50.0 * stellar_mass_ratio.powf(0.33)
}

fn inner_effect_limit(a: &f64, e: &f64, mass: &f64, cloud_eccentricity: &f64) -> f64 {
    a * (1.0 - e) * (1.0 - mass) / (1.0 + cloud_eccentricity)
}

fn outer_effect_limit(a: &f64, e: &f64, mass: &f64, cloud_eccentricity: &f64) -> f64 {
    a * (1.0 + e) * (1.0 + mass) / (1.0 - cloud_eccentricity)
}

use crate::consts::*;
use crate::utils::*;

#[derive(Debug, Copy, Clone)]
pub struct DustBand {
    pub outer_edge: f64,
    pub inner_edge: f64,
    pub dust_present: bool,
    pub gas_present: bool,
}

impl DustBand {
    pub fn new(outer_edge: f64, inner_edge: f64, dust_present: bool, gas_present: bool) -> Self {
        Self {
            outer_edge,
            inner_edge,
            dust_present,
            gas_present,
        }
    }
}

pub fn dust_availible(dust_bands: &Vec<DustBand>, inside_range: &f64, outside_range: &f64) -> bool {
    dust_bands.iter().rev().fold(false, |mut acc, band| {
        if band.dust_present && &band.outer_edge > inside_range && &band.inner_edge < outside_range
        {
            acc = true;
        }
        acc
    })
}

pub fn accrete_dust(
    mass: &mut f64,
    new_dust: &mut f64,
    new_gas: &mut f64,
    a: &f64,
    e: &f64,
    crit_mass: &f64,
    dust_bands: &mut Vec<DustBand>,
    cloud_eccentricity: &f64,
    dust_density: &f64,
    k: &f64,
) {
    let mut new_mass = *mass;
    let mut temp_mass;

    loop {
        temp_mass = new_mass;
        new_mass = collect_dust(
            &mut new_mass,
            new_dust,
            new_gas,
            a,
            e,
            crit_mass,
            dust_bands,
            0,
            cloud_eccentricity,
            dust_density,
            k,
        );

        if !(new_mass - temp_mass >= 0.0001 * temp_mass) {
            break;
        }
    }
    *mass += new_mass;
}

pub fn collect_dust(
    last_mass: &mut f64,
    new_dust: &mut f64,
    new_gas: &mut f64,
    a: &f64,
    e: &f64,
    crit_mass: &f64,
    dust_bands: &mut Vec<DustBand>,
    i: usize,
    cloud_eccentricity: &f64,
    dust_density: &f64,
    k: &f64,
) -> f64 {
    // for d in dust_bands.iter() {
    match dust_bands.get(i) {
        None => 0.0,
        Some(d) => {
            let mut temp = *last_mass / (1.0 + *last_mass);
            let reduced_mass = temp.powf(1.0 / 4.0);
            let mut r_inner = inner_effect_limit(a, e, &reduced_mass, cloud_eccentricity);
            let r_outer = outer_effect_limit(a, e, &reduced_mass, cloud_eccentricity);

            if r_inner < 0.0 {
                r_inner = 0.0;
            }

            let temp_density = match d.dust_present == true {
                true => *dust_density,
                false => 0.0,
            };

            let mass_density;
            let mut gas_density = 0.0;
            if *last_mass < *crit_mass || !d.gas_present {
                mass_density = temp_density
            } else {
                mass_density = get_mass_density(k, &temp_density, &crit_mass, &last_mass);
                gas_density = mass_density - temp_density;
            }

            if d.outer_edge <= r_inner || d.inner_edge >= r_outer {
                return collect_dust(
                    last_mass,
                    new_dust,
                    new_gas,
                    a,
                    e,
                    crit_mass,
                    dust_bands,
                    i + 1,
                    cloud_eccentricity,
                    dust_density,
                    k,
                );
            } else {
                let bandwidth = r_outer - r_inner;

                let mut temp1 = r_outer - d.outer_edge;
                if temp1 < 0.0 {
                    temp1 = 0.0;
                }
                let mut width = bandwidth - temp1;

                let mut temp2 = d.inner_edge - r_inner;
                if temp2 < 0.0 {
                    temp2 = 0.0;
                }
                width = width - temp2;

                temp =
                    4.0 * PI * a.powf(2.0) * reduced_mass * (1.0 - e * (temp1 - temp2) / bandwidth);
                let volume = temp * width;

                let new_mass = volume * mass_density;
                *new_gas = volume * gas_density;
                *new_dust = new_mass - *new_gas;
                let mut next_dust = 0.0;
                let mut next_gas = 0.0;

                let next_mass = collect_dust(
                    last_mass,
                    &mut next_dust,
                    &mut next_gas,
                    a,
                    e,
                    crit_mass,
                    dust_bands,
                    i + 1,
                    cloud_eccentricity,
                    dust_density,
                    k,
                );

                *new_gas = *new_gas + next_gas;
                *new_dust = *new_dust + next_dust;

                new_mass + next_mass
            }
        } // }

          // volume * mass_density
    }
}

pub fn update_dust_lanes(
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

pub fn compress_dust_lanes(dust_bands: &mut Vec<DustBand>) {
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

pub fn stellar_dust_limit(stellar_mass_ratio: &f64) -> f64 {
    200.0 * stellar_mass_ratio.powf(1.0 / 3.0)
}

pub fn get_mass_density(k: &f64, dust_density: &f64, critical_mass: &f64, mass: &f64) -> f64 {
    k * dust_density / (1.0 + (critical_mass / mass).sqrt() * (k - 1.0))
}

pub fn dust_density(dust_density_coeff: &f64, stellar_mass: &f64, oribital_radius: &f64) -> f64 {
    dust_density_coeff * stellar_mass.sqrt() * (-ALPHA * oribital_radius.powf(1.0 / N)).exp()
}

use crate::consts::PI;
use crate::planetismal::*;
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
    planetismal: &mut Planetismal,
    dust_bands: &mut Vec<DustBand>,
    crit_mass: &f64,
    dust_density: &f64,
    cloud_eccentricity: &f64,
    k: &f64,
) {
    let mut new_mass = planetismal.mass;

    loop {
        planetismal.mass = new_mass;
        new_mass = 0.0;

        for d in dust_bands.iter_mut() {
            new_mass += collect_dust(planetismal, crit_mass, d, cloud_eccentricity, dust_density, k);
        }

        if !(new_mass - planetismal.mass > 0.0001 * planetismal.mass) {
            break;
        }
    }
    planetismal.mass = new_mass;
}

pub fn collect_dust(
    p: &Planetismal,
    crit_mass: &f64,
    dust_band: &mut DustBand,
    cloud_eccentricity: &f64,
    dust_density: &f64,
    k: &f64,
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
        true => mass_density(k, &temp_density, &crit_mass, &mass),
        // K * temp_density / (1.0 + (crit_mass / mass).sqrt() * (K - 1.0)),
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

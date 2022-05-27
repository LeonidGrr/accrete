use crate::consts::*;
use crate::utils::*;

use serde::{Deserialize, Serialize};

pub type DustBands = Vec<DustBand>;
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
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

pub fn dust_availible(dust_bands: &[DustBand], inside_range: &f64, outside_range: &f64) -> bool {
    dust_bands.iter().rev().fold(false, |mut acc, band| {
        if band.dust_present && &band.outer_edge > inside_range && &band.inner_edge < outside_range
        {
            acc = true;
        }
        acc
    })
}

/// "The center of mass is occupied by a star with a mass of one unit (one solar mass). All particles in the cloud are moving on elliptical orbits, with the center of mass at one focus. The density of dust (p1) within the cloud depends on a function of the form p1 = A exp (-arl/n). The overall density of gas and dust (p2) within the cloud equals Kpl, where r is distance from the center of mass (in astronomical units, a.u.) and A. a. n. and K (the vas/dust ratio) are constants."
/// "There is a spherically symmetrical cloud of dust and gas with a constant ratio of gas to dust, the density decreasing with distance from the center."
pub fn accrete_dust(
    mass: &mut f64,
    a: &f64,
    e: &f64,
    crit_mass: &f64,
    dust_bands: &mut [DustBand],
    cloud_eccentricity: &f64,
    dust_density: &f64,
    k: &f64,
) {
    let mut new_mass = *mass;
    loop {
        *mass = new_mass;
        new_mass = 0.0;

        for d in dust_bands.iter_mut() {
            new_mass += collect_dust(
                mass,
                a,
                e,
                crit_mass,
                cloud_eccentricity,
                dust_density,
                k,
                d,
            );
        }

        if !((new_mass - *mass) >= (0.0001 * *mass)) {
            break;
        }
    }
    *mass = new_mass;
}

pub fn collect_dust(
    mass: &f64,
    a: &f64,
    e: &f64,
    crit_mass: &f64,
    cloud_eccentricity: &f64,
    dust_density: &f64,
    k: &f64,
    band: &mut DustBand,
) -> f64 {
    let mut r_inner = inner_swept_limit(a, e, mass, cloud_eccentricity);
    let r_outer = outer_swept_limit(a, e, mass, cloud_eccentricity);

    if r_inner < 0.0 {
        r_inner = 0.0;
    }

    if band.outer_edge <= r_inner || band.inner_edge >= r_outer || !band.dust_present {
        return 0.0;
    };

    let density = match !band.gas_present || mass < crit_mass {
        true => *dust_density,
        false => get_mass_density(k, dust_density, crit_mass, mass),
    };
    let bandwidth = r_outer - r_inner;
    let temp1 = match r_outer - band.outer_edge > 0.0 {
        true => r_outer - band.outer_edge,
        false => 0.0,
    };
    let temp2 = match band.inner_edge - r_inner > 0.0 {
        true => band.inner_edge - r_inner,
        false => 0.0,
    };
    let width = bandwidth - temp1 - temp2;
    let term1 = 4.0 * PI * a.powf(2.0);
    let term2 = 1.0 - e * (temp1 - temp2) / bandwidth;
    let volume = term1 * reduced_mass(mass) * width * term2;

    volume * density
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
            let mut inner = *band;
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
            acc.push(*band);
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
                        let mut band = *band;
                        band.outer_edge = next_band.outer_edge;
                        acc.push(band);
                    } else {
                        acc.push(*band);
                    }
                }
                None => acc.push(*band),
            }
            acc
        });
}

pub fn get_mass_density(k: &f64, dust_density: &f64, critical_mass: &f64, mass: &f64) -> f64 {
    k * dust_density / (1.0 + (critical_mass / mass).sqrt() * (k - 1.0))
}

pub fn dust_density(dust_density_coeff: &f64, stellar_mass: &f64, oribital_radius: &f64) -> f64 {
    dust_density_coeff * stellar_mass.sqrt() * (-ALPHA * oribital_radius.powf(1.0 / N)).exp()
}

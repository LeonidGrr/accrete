#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
pub struct DustBand {
    pub inner: f64,
    pub outer: f64,
    pub dust: bool,
    pub gas: bool,
}

impl DustBand {
    pub fn new(
        inner_limit: f64,
        outer_limit: f64,
        dust_present: Option<bool>,
        gas_present: Option<bool>,
    ) -> Self {
        DustBand {
            inner: inner_limit,
            outer: outer_limit,
            dust: dust_present.unwrap_or(true),
            gas: gas_present.unwrap_or(true),
        }
    }
}

#[derive(Debug)]
pub struct DustBands {
    pub bands: Vec<DustBand>,
}

impl DustBands {
    pub fn new(inner: f64, outer: f64) -> Self {
        let mut bands = Vec::new();
        let dust_head = DustBand::new(inner, outer, None, None);
        bands.push(dust_head);
        DustBands { bands }
    }

    pub fn add_band(
        bands: &mut Vec<DustBand>,
        min: f64,
        max: f64,
        dust: Option<bool>,
        gas: Option<bool>,
        after: Option<usize>,
    ) -> usize {
        let band = DustBand::new(min, max, dust, gas);

        match after {
            Some(i) => {
                if i + 1 <= bands.len() {
                    bands.insert(i + 1, band);
                } else {
                    bands.push(band);
                }

                return i + 1;
            }
            None => {
                bands.push(band);
                return bands.len() - 1;
            }
        }
    }

    pub fn update_lanes(&mut self, min: f64, max: f64, used_gas: &bool) {
        self.bands = self
            .bands
            .iter_mut()
            .enumerate()
            .fold(Vec::new(), |mut acc, (i, band)| {
                let new_gas = band.gas && !*used_gas;
                if band.inner < min && band.outer > max {
                    DustBands::add_band(&mut acc, min, max, Some(false), Some(new_gas), Some(i));

                    DustBands::add_band(
                        &mut acc,
                        min,
                        band.outer,
                        Some(band.dust),
                        Some(band.gas),
                        Some(i + 1),
                    );

                    band.outer = min;
                    acc.push(band.clone());
                } else if band.inner < max && band.outer > max {
                    DustBands::add_band(
                        &mut acc,
                        max,
                        band.outer,
                        Some(band.dust),
                        Some(band.gas),
                        Some(i),
                    );

                    band.outer = max;
                    band.dust = false;
                    band.gas = new_gas;
                    acc.push(band.clone());
                } else if band.inner < min && band.outer > min {
                    DustBands::add_band(
                        &mut acc,
                        min,
                        band.outer,
                        Some(false),
                        Some(new_gas),
                        Some(i),
                    );

                    band.outer = min;
                    acc.push(band.clone());
                } else if band.inner >= min && band.outer <= max {
                    band.dust = false;
                    band.gas = new_gas;
                    acc.push(band.clone());
                } else if band.inner > max || band.outer < min {
                    acc.push(band.clone());
                }
                acc
            });
    }

    pub fn dust_remaining(&self, inner_bound: f64, outer_bound: f64) -> bool {
        self.bands.iter().fold(false, |mut acc, band| {
            if band.dust && band.outer >= inner_bound && band.inner <= outer_bound {
                acc = true;
            }
            acc
        })
    }

    pub fn compress_lanes(&mut self) {
        self.bands = self
            .bands
            .iter()
            .enumerate()
            .fold(Vec::new(), |mut acc, (i, band)| {
                if i == 0 {
                    acc.push(*band);
                    return acc;
                }
                match acc.get(i - 1) {
                    Some(prev) => {
                        if band.dust != prev.dust || band.gas != prev.gas {
                            acc.push(*band);
                        }
                    }
                    None => acc.push(*band),
                }
                acc
            });
    }
}

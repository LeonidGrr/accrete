/// BIBLIOGRAPHY
/// Dole, Stephen H.
/// "Formation of Planetary Systems by Aggregation: a Computer Simulation"
/// October 1969, Rand  Corporation Paper P-4226.
use crate::structs::*;

#[derive(Debug, Clone)]
pub struct Accrete {
	dust_left: bool,
	r_inner: f64,
	r_outer: f64,
	reduced_mass: f64,
	dust_density: f64,
	cloud_eccentricity: f64,
	dust_bands: Vec<DustBand>,
	planets: Vec<Planetismal>,
}

impl Accrete {
	fn set_initial_conditions(inner_limit_of_dust: f64, outer_limit_of_dust: f64) -> Self {
		let mut dust_band = DustBand::new(outer_limit_of_dust, inner_limit_of_dust, true, true);
		let mut dust_bands = Vec::new();
		dust_bands.push(dust_band);

		Self {
			dust_left: true,
			planets: Vec::new(),
			cloud_eccentricity: 0.2,
			r_inner: 0.0,
			r_outer: 0.0,
			reduced_mass: 0.0,
			dust_density: 0.0,
			dust_bands,
		}
	}

	fn dust_available(&self, inside_range: &f64, outside_range: &f64) -> bool {
		self.dust_bands.iter().rev().fold(false, |mut acc, band| {
            if band.dust_present && band.outer_edge > *inside_range && band.inner_edge < *outside_range {
                acc = true;
            }
            acc
        })
	}
        
	fn update_dust_lanes(&mut self, min: &f64, max: &f64, mass: &f64, crit_mass: &f64, body_inner_bound: &f64, body_outer_bound: &f64) {
		let mut gas = true;
		let mut dust_left = false;

		if mass > crit_mass {
			gas = false;
		}

		self.dust_bands = self
            .dust_bands
            .iter_mut()
            .fold(Vec::new(), |mut acc, band| {
                let new_gas = band.gas_present && gas;
				if band.inner_edge < min && band.outer_edge > max {
					let inner = DustBand::new(max, min, false, new_gas);
					let outer = DustBand::new(band.outer_edge, max, band.dust_present, band.gas_present);
					acc.push(inner);
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
            
            dust_left = self.dust_availible(&body_inner_bound, &body_outer_bound);

		// Compress lanes
		self.dust_bands = self
            .dust_bands
            .iter()
            .enumerate()
            .fold(Vec::new(), |mut acc, (i, band)| {
					match self.dust_bands.get(i + 1) {
						Some(next_band) => {
							if band.dust_present == next_band.dust_present && band.gas_present == next_band.gas_present {
								let mut band = band.clone();
								band.outer_edge = next_band.outer_edge;
								acc.push(band);
							}
						},
						None => acc.push(band.clone()),
					} 
				} else {
					acc.push(band.clone());
				}
				acc
            });
		self.dust_left = dust_left;
	}

fn collect_dust(&mut self, last_mass: &f64, a: &f64, e: &f64, crit_mass: &f64, dust_band: &mut DustBand) {
  Let mut temp1 = 0.0;
  Let mut temp2 = 0.0;
  Let mut temp = 0.0;
  Let mut bandwidth = 0.0;
  Let mut width = 0.0;
  Let mut volume = 0.0;
  let mut temp_density = 0.0;
  let mut mass_density = 0.0;
  let temp = last_mass / (1.0 + last_mass);
  self.reduced_mass = temp.powf(0.25);
  self.r_inner = inner_effect_limit(a, e, &self.reduced_mass, &self.cloud_eccentricity);
  self.r_outer = outer_effect_limit(a, e, &self.reduced_mass, &self.cloud_eccentricity);
    
  if self.r_inner < 0.0 {
      self.r_inner = 0.0;
  }
  
  if dust_band.dust_present == false {
      temp_density = 0.0;
  } else {
      temp_density = dust_density;
  }

  if last_mass < crit_mass || dust_band.gas_present == false {
      mass_density = temp_density;
  } else {
      mass_density = K * temp_density / (1.0 + (crit_mass / last_mass).sqrt() * (K - 1.0));
  }

  if dust_band.outer_edge <= self.r_inner || dust_band.inner_edge >= self.r_outer {
     return 0.0;
  }
     bandwidth = (self.r_outer - self.r_inner);
     temp1 = self.r_outer - dust_band.outer_edge;
	    if temp1 < 0.0 {
		temp1 = 0.0;
            }
 	    width = bandwidth - temp1;
	    temp2 = dust_band.inner_edge - self.r_inner;
	    if temp2 < 0.0 {
		temp2 = 0.0;
            }
	    width = width - temp2;
	    temp = 4.0 * PI * a.powf(2.0) * self.reduced_mass * (1.0 - e * (temp1 - temp2) / bandwidth);
 	    volume = temp * width;
	    volume * mass_density
}

/// Orbital radius is in AU, eccentricity is unitless, and the stellar luminosity ratio is with respect to the sun.
/// The value returned is the mass at which the planet begins to accrete gas as well as dust, and is in units of solar masses.

fn critical_limit(orbital_radius: &f64, eccentricity: &f64, stellar_luminosity_ratio: &f64) -> f64 {
   let perihelion_dist = orbital_radius - orbital_radius * eccentricity;
   let temp = perihelion_dist * stellar_luminosity_ratio.sqrt();
   B * temp.powf(-0.75)
}

fn accrete_dust(&mut self, planetismal_mass: &mut f64, a: &f64, e: &f64, crit_mass: &f64) {
     let mut new_mass = planetismal_mass;
     let mut temp_mass = planetismal_mass;
     loop {
       for d in self.dust_bands.iter_mut() {
          temp_mass = new_mass;
          new_mass += self.collect_dust(&new_mass, a, e, crit_mass, d);
       }
       if !(new_mass - temp_mass > 0.0001 * temp_mass) {
          break;
       }
      }
     planetismal_mass = planetismal_mass + new_mass;
}

 fn coalesce_planetesimals(a, e, mass, crit_mass, stellar_luminosity_ratio, body_inner_bound, body_outer_bound) {
 //    var node1, node2, node3;
//     var coalesced;
//     var dist1, dist2, a3;

//     var temp;

//     coalesced = FALSE;
//     node1 = planet_head;
//     node2 = NULL;
//     node3 = NULL;
//     while ((node1 != NULL)) {
// 	node2 = node1;
// 	temp = new MASS(node1.a - a);
// 	if ((temp.VALUE > 0.0)) {
// 	    dist1 = (a * (1.0 + e) * (1.0 + reduced_mass)) - a;
// 	    /* x aphelion */
// 	    reduced_mass = Math.pow((node1.mass / (1.0 + node1.mass)), (1.0 / 4.0));
// 	    dist2 = node1.a - (node1.a * (1.0 - node1.e) * (1.0 - reduced_mass));
// 	} else {
// 	    dist1 = a - (a * (1.0 - e) * (1.0 - reduced_mass));
// 	    /* x perihelion */
// 	    reduced_mass = Math.pow(node1.mass / (1.0 + node1.mass), (1.0 / 4.0));
// 	    dist2 = (node1.a * (1.0 + node1.e) * (1.0 + reduced_mass)) - node1.a;
// 	}
// 	if (((Math.abs(temp.VALUE) <= Math.abs(dist1)) || (Math.abs(temp.VALUE) <= Math.abs(dist2)))) {
// 	    console.debug(sprintf("Collision between two planetesimals!\n"));
// 	    a3 = (node1.mass + mass) / ((node1.mass / node1.a) + (mass / a));
// 	    temp = new MASS(node1.mass * Math.sqrt(node1.a) * Math.sqrt(1.0 - Math.pow(node1.e, 2.0)));
// 	    temp.VALUE = temp.VALUE + (mass * Math.sqrt(a) * Math.sqrt(Math.sqrt(1.0 - Math.pow(e, 2.0))));
// 	    temp.VALUE = temp.VALUE / ((node1.mass + mass) * Math.sqrt(a3));
// 	    temp.VALUE = 1.0 - Math.pow(temp.VALUE, 2.0);
// 	    if (((temp.VALUE < 0.0) || (temp.VALUE >= 1.0)))
// 		temp.VALUE = 0.0;
// 	    e = Math.sqrt(temp.VALUE);
// 	    temp.VALUE = node1.mass + mass;
// 	    accrete_dust(temp, a3, e, stellar_luminosity_ratio, body_inner_bound, body_outer_bound);
// 	    node1.a = a3;
// 	    node1.e = e;
// 	    node1.mass = temp.VALUE;
// 	    node1 = NULL;
// 	    coalesced = TRUE;
// 	} else
// 	    node1 = node1.next_planet;
//     }
//     if (!(coalesced)) {
// 	node3 = new planets_record();
// 	node3.a = a;
// 	node3.e = e;
// 	if ((mass >= crit_mass))
// 	    node3.gas_giant = TRUE;
// 	else
// 	    node3.gas_giant = FALSE;
// 	node3.mass = mass;
// 	if ((planet_head == NULL)) {
// 	    planet_head = node3;
// 	    node3.next_planet = NULL;
// 	} else {
// 	    node1 = planet_head;
// 	    if ((a < node1.a)) {
// 		node3.next_planet = node1;
// 		planet_head = node3;
// 	    } else if ((planet_head.next_planet == NULL)) {
// 		planet_head.next_planet = node3;
// 		node3.next_planet = NULL;
// 	    } else {
// 		while (((node1 != NULL) && (node1.a < a))) {
// 		    node2 = node1;
// 		    node1 = node1.next_planet;
// 		}
// 		node3.next_planet = node1;
// 		node2.next_planet = node3;
// 	    }
// 	}
//     }
// }

 fn distribute_planetary_masses(stellar_mass_ratio: f64, stellar_luminosity_ratio: f64, inner_dust: f64, outer_dust: f64) {
       let mut accrete = Accrete::set_initial_conditions(inner_dust, outer_dust);
       let planetesimal_inner_bound = innermost_planet(stellar_mass_ratio);
       let planetesimal_outer_bound = outermost_planet(stellar_mass_ratio);
       while (accrete.dust_left) {
 	let a = random_number(planetesimal_inner_bound, planetesimal_outer_bound);
 	let e = random_eccentricity();
 	let mass = PROTOPLANET_MASS;
        let inside_range = inner_effect_limit(a, e, mass, accrete.cloud_eccentricity);
        let outside_range = outer_effect_limit(a, e, mass, accrete.cloud_eccentricity);
 	if accrete.dust_available(inside_range, outside_range) {
 	    let dust_density = DUST_DENSITY_COEFF * stellar_mass_ratio.sqrt() * (-ALPHA * a.powf(1.0 / N)).exp());
 	    let crit_mass = critical_limit(a, e, stellar_luminosity_ratio);
 	    accrete_dust(mass, a, e, crit_mass);
            update_dust_lanes(accrete.r_inner, accrete.r_outer, mass, crit_mass, planetesimal_inner_bound, planetesimal_outer_bound);
	    if mass != 0.0 && mass != PROTOPLANET_MASS {
 		coalesce_planetesimals(a, e, mass.VALUE, crit_mass, stellar_luminosity_ratio, planetesimal_inner_bound, planetesimal_outer_bound);
 	    } else { 
               // belt?
 		// console.debug(sprintf(".. failed due to large neighbor.\n"));
	    }
 	}
     }
     accrete 
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

use crate::consts::PROTOPLANET_MASS;
use crate::utils::*;
use rand::prelude::*;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Planetismal {
    // axis in AU
    pub a: f64,
    // eccentricity of the orbit
    pub e: f64,
    pub mass: f64,
    pub earth_masses: f64,
    // if the planet is a gas giant
    pub gas_giant: bool,
    // the 'zone' of the planet
    pub orbit_zone: i32,
    // equatorial radius (in km)
    pub radius: f64,
    // density (in g/cc)
    pub density: f64,
    // TRUE if in resonant rotation
    pub resonant_period: bool,
    // units of degrees
    pub axial_tilt: f64,
    // units of cm/sec
    pub escape_velocity: f64,
    // units of cm/sec2
    pub surface_accel: f64,
    // units of Earth gravities
    pub surface_grav: f64,
    // units of cm/sec
    pub rms_velocity: f64,
    pub escape_velocity_km_per_sec: f64,
    pub orbital_period_days: f64,
    pub day_hours: f64,
    pub length_of_year: f64,
    pub molecule_weight: f64,
    pub smallest_molecular_weight: String,
    pub volatile_gas_inventory: f64,
    pub greenhouse_effect: bool,
    pub albedo: f64,
    pub surface_pressure_millibar: f64,
    pub surface_pressure_bar: f64,
    pub surface_temp_celsium: f64,
    pub surface_temp_kelvin: f64,
    pub boiling_point_kelvin: f64,
    pub boiling_point_celsium: f64,
    pub hydrosphere: f64,
    pub cloud_cover: f64,
    pub ice_cover: f64,
    pub moons: Vec<Planetismal>,

    // the_planet->gases			= 0;
	// 	the_planet->surf_temp		= 0;
	// 	the_planet->high_temp		= 0;
	// 	the_planet->low_temp		= 0;
	// 	the_planet->max_temp		= 0;
	// 	the_planet->min_temp		= 0;
	// 	the_planet->greenhs_rise	= 0;
    // 	the_planet->minor_moons 	= 0;
    // long double	roche_limit = 0.0;
    // 					long double	hill_sphere = 0.0;
    
    // roche_limit = 2.44 * planet->radius * pow((planet->density / ptr->density), (1.0 / 3.0));
	// 					hill_sphere = planet->a * KM_PER_AU * pow((planet->mass / (3.0 * sun->mass)), (1.0 / 3.0));
						
	// 					if ((roche_limit * 3.0) < hill_sphere)
	// 					{
	// 						ptr->moon_a = random_number(roche_limit * 1.5, hill_sphere / 2.0) / KM_PER_AU;
	// 						ptr->moon_e = random_eccentricity ();
	// 					}
	// 					else
	// 					{
	// 						ptr->moon_a = 0;
	// 						ptr->moon_e = 0;
	// 					}
						
	// 					if (flag_verbose & 0x40000)
	// 					{
	// 						fprintf (stderr, 
	// 									"   Roche limit: R = %4.2Lg, rM = %4.2Lg, rm = %4.2Lg -> %.0Lf km\n"
	// 									"   Hill Sphere: a = %4.2Lg, m = %4.2Lg, M = %4.2Lg -> %.0Lf km\n"
	// 									"%s Moon orbit: a = %.0Lf km, e = %.0Lg\n",
	// 									planet->radius, planet->density, ptr->density,
	// 									roche_limit,
	// 									planet->a * KM_PER_AU, planet->mass * SOLAR_MASS_IN_KILOGRAMS, sun->mass * SOLAR_MASS_IN_KILOGRAMS,
	// 									hill_sphere,
	// 									moon_id,
	// 									ptr->moon_a * KM_PER_AU, ptr->moon_e
	// 								);
    // 					}
    


        /// tidal lock
    // if ((int)planet->day == (int)(planet->orb_period * 24.0))
    // 		printf("Planet is tidally locked with one face to star.\n");
    
    // printf (file,
	// 	"<tr><th>Equatorial radius</th>"
	// 	"<td>%3.1Lf Km</td>"
	// 	"<td>%.2LG Earth radii</td></tr>\n",
	// 		planet->radius,
	// 		planet->radius / KM_EARTH_RADIUS);
	// fprintf (file,
	// 	"<tr><th>Density</th>"
	// 	"<td>%4.2Lf grams/cc</td>"
	// 	"<td>%.2LG Earth densities</td></tr>\n",
	// 		planet->density,
	// 		planet->den
    
			// "high_temp",
			// "low_temp",
			// "max_temp",
            // "min_temp",
        //     fprintf (file, 
		// 		"<tr><th>Normal temperature range</th>"
		// 		"<td><center><table>\n");

		// 	if (fabs(planet->high_temp - planet->max_temp) > 10 
		// 	 || fabs(planet->low_temp - planet->min_temp) > 10)
		// 	{
		// 		fprintf (file, "\t<tr><th>Night</th><th></th><th>Day</th></tr>\n");
				
		// 		fprintf (file, 
		// 			"\t<tr><td>%5.1Lf&deg; C<br>%5.1Lf&deg; F</td>"
		// 			"<td> - </td>",
		// 				planet->low_temp - FREEZING_POINT_OF_WATER,
		// 				32.0 + (1.8 * (planet->low_temp - FREEZING_POINT_OF_WATER)));

		// 		fprintf (file, 
		// 			"<td>%5.1Lf&deg; C<br>%5.1Lf&deg; F</td>"
		// 			"</tr>\n",
		// 				planet->high_temp - FREEZING_POINT_OF_WATER,
		// 				32.0 + (1.8 * (planet->high_temp - FREEZING_POINT_OF_WATER)));
		// 	}
			
		// 	fprintf (file, "\t<tr><th>Min</th><th></th><th>Max</th></tr>\n");
				
		// 	fprintf (file, 
		// 		"\t<tr><td>%5.1Lf&deg; C<br>%5.1Lf&deg; F</td>"
		// 		"<td> - </td>",
		// 		planet->min_temp - FREEZING_POINT_OF_WATER,
		// 		32.0 + (1.8 * (planet->min_temp - FREEZING_POINT_OF_WATER)));

		// 	fprintf (file, 
		// 		"<td>%5.1Lf&deg; C<br>%5.1Lf&deg; F</td>"
		// 		"</tr>\n",
		// 			planet->max_temp - FREEZING_POINT_OF_WATER,
		// 			32.0 + (1.8 * (planet->max_temp - FREEZING_POINT_OF_WATER)));

		// fprintf (file, 
		// 	"</table></center></td></tr>\n");
		// }
}

impl Planetismal {
    pub fn new(
        planetesimal_inner_bound: &f64,
        planetesimal_outer_bound: &f64,
        cloud_eccentricity: &f64,
    ) -> Self {
        let mut rng = rand::thread_rng();
        let gas_giant = false;
        let a = rng.gen_range(planetesimal_inner_bound, planetesimal_outer_bound);
        let e = random_eccentricity(rng.gen_range(0.0, 1.0), cloud_eccentricity);

        Planetismal {
            a,
            e,
            mass: PROTOPLANET_MASS,
            earth_masses: 0.0,
            gas_giant,
            orbit_zone: 0,
            radius: 0.0,
            density: 0.0,
            orbital_period_days: 0.0,
            day_hours: 0.0,
            resonant_period: false,
            axial_tilt: 0.0,
            escape_velocity: 0.0,
            surface_accel: 0.0,
            surface_grav: 0.0,
            rms_velocity: 0.0,
            molecule_weight: 0.0,
            volatile_gas_inventory: 0.0,
            greenhouse_effect: false,
            albedo: 0.0,
            surface_temp_kelvin: 0.0,
            surface_temp_celsium: 0.0,
            surface_pressure_millibar: 0.0,
            surface_pressure_bar: 0.0,
            boiling_point_kelvin: 0.0,
            boiling_point_celsium: 0.0,
            hydrosphere: 0.0,
            cloud_cover: 0.0,
            ice_cover: 0.0,
            moons: Vec::new(),
            smallest_molecular_weight: String::new(),
            length_of_year: 0.0,
            escape_velocity_km_per_sec: 0.0,
            // atmosphere
        }
    }

    // if (do_moons)
	// 		{
	// 			long double existing_mass = 0.0;
				
	// 			if (the_planet->first_moon != NULL)
	// 			{
	// 				planet_pointer	m;
					
	// 				for (m = the_planet->first_moon;
	// 					 m != NULL;
	// 					 m = m->next_planet)
	// 				{
	// 					existing_mass += m->mass;
	// 				}
	// 			}

	// 			if (mass < crit_mass)
	// 			{
	// 				if ((mass * SUN_MASS_IN_EARTH_MASSES) < 2.5
	// 				 && (mass * SUN_MASS_IN_EARTH_MASSES) > .0001
	// 				 && existing_mass < (the_planet->mass * .05)
	// 				   )
	// 				{
	// 					planet_pointer	the_moon = (planets *)malloc(sizeof(planets));
						
	// 					the_moon->type 			= tUnknown;
	// /* 					the_moon->a 			= a; */
	// /* 					the_moon->e 			= e; */
	// 					the_moon->mass 			= mass;
	// 					the_moon->dust_mass 	= dust_mass;
	// 					the_moon->gas_mass 		= gas_mass;
	// 					the_moon->atmosphere 	= NULL;
	// 					the_moon->next_planet 	= NULL;
	// 					the_moon->first_moon 	= NULL;
	// 					the_moon->gas_giant 	= FALSE;
	// 					the_moon->atmosphere	= NULL;
	// 					the_moon->albedo		= 0;
	// 					the_moon->gases			= 0;
	// 					the_moon->surf_temp		= 0;
	// 					the_moon->high_temp		= 0;
	// 					the_moon->low_temp		= 0;
	// 					the_moon->max_temp		= 0;
	// 					the_moon->min_temp		= 0;
	// 					the_moon->greenhs_rise	= 0;
	// 					the_moon->minor_moons 	= 0;
	
	// 					if ((the_moon->dust_mass + the_moon->gas_mass)
	// 					  > (the_planet->dust_mass + the_planet->gas_mass))
	// 					{
	// 						long double	temp_dust = the_planet->dust_mass;
	// 						long double temp_gas  = the_planet->gas_mass;
	// 						long double temp_mass = the_planet->mass;
							
	// 						the_planet->dust_mass = the_moon->dust_mass;
	// 						the_planet->gas_mass  = the_moon->gas_mass;
	// 						the_planet->mass      = the_moon->mass;
							
	// 						the_moon->dust_mass   = temp_dust;
	// 						the_moon->gas_mass    = temp_gas;
	// 						the_moon->mass        = temp_mass;
	// 					}
	
	// 					if (the_planet->first_moon == NULL)
	// 						the_planet->first_moon = the_moon;
	// 					else
	// 					{
	// 						the_moon->next_planet = the_planet->first_moon;
	// 						the_planet->first_moon = the_moon;
	// 					}
						
	// 					finished = TRUE;
						
	// 					if (flag_verbose & 0x0100)
	// 						fprintf (stderr, "Moon Captured... "
	// 								 "%5.3Lf AU (%.2LfEM) <- %.2LfEM\n",
	// 								the_planet->a, the_planet->mass * SUN_MASS_IN_EARTH_MASSES, 
	// 								mass * SUN_MASS_IN_EARTH_MASSES
	// 								);
	// 				}
	// 				else 
	// 				{
	// 					if (flag_verbose & 0x0100)
	// 						fprintf (stderr, "Moon Escapes... "
	// 								 "%5.3Lf AU (%.2LfEM)%s %.2LfEM%s\n",
	// 								the_planet->a, the_planet->mass * SUN_MASS_IN_EARTH_MASSES, 
	// 								existing_mass < (the_planet->mass * .05) ? "" : " (big moons)",
	// 								mass * SUN_MASS_IN_EARTH_MASSES,
	// 								(mass * SUN_MASS_IN_EARTH_MASSES) >= 2.5 ? ", too big" : 
	// 								  (mass * SUN_MASS_IN_EARTH_MASSES) <= .0001 ? ", too small" : ""
	// 								);
	// 				}
	// 			}
	// 		}
}

pub fn coalesce_planetismals(planets: &mut Vec<Planetismal>, cloud_eccentricity: &f64) {
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

pub fn coalesce_two_planets(a: &Planetismal, b: &Planetismal) -> Planetismal {
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

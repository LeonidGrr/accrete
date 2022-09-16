use crate::enviro::*;
use crate::events_log::accrete_event::AccreteEvents;
use crate::events_log::event_source::EventSource;
use crate::structs::*;
use crate::utils::*;

use rand::distributions::WeightedIndex;
use rand::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct System {
    pub primary_star: PrimaryStar,
    pub planets: Vec<Planetesimal>,
    pub cloud_eccentricity: f64,
    pub dust_density_coeff: f64,
    pub k: f64,
    pub b: f64,
    pub planetesimal_inner_bound: f64,
    pub planetesimal_outer_bound: f64,
    pub inner_dust: f64,
    pub outer_dust: f64,
    pub dust_bands: Vec<DustBand>,
    pub dust_left: bool,
}

impl System {
    pub fn set_initial_conditions(
        stellar_mass: f64,
        dust_density_coeff: f64,
        k: f64,
        cloud_eccentricity: f64,
        b: f64,
    ) -> Self {
        let primary_star = PrimaryStar::new(stellar_mass);
        let planetesimal_inner_bound = innermost_planet(&stellar_mass);
        let planetesimal_outer_bound = outermost_planet(&stellar_mass);
        let inner_dust = 0.0;
        let outer_dust = stellar_dust_limit(&stellar_mass);
        let dust_band = DustBand::new(outer_dust, inner_dust, true, true);
        let dust_bands = vec![dust_band];

        Self {
            primary_star,
            planets: Vec::new(),
            k,
            b,
            dust_density_coeff,
            cloud_eccentricity,
            planetesimal_inner_bound,
            planetesimal_outer_bound,
            inner_dust,
            outer_dust,
            dust_bands,
            dust_left: true,
        }
    }

    pub fn distribute_planetary_masses(
        &mut self,
        rng: &mut dyn RngCore,
        events_log: &mut AccreteEvents,
    ) {
        let Self {
            primary_star,
            planets,
            k,
            b,
            dust_density_coeff,
            cloud_eccentricity,
            planetesimal_inner_bound,
            planetesimal_outer_bound,
            dust_bands,
            dust_left,
            ..
        } = self;
        let PrimaryStar {
            stellar_mass,
            stellar_luminosity,
            ..
        } = primary_star;

        while *dust_left {
            let mut p = Planetesimal::new(planetesimal_inner_bound, planetesimal_outer_bound, rng);
            p.event("planetesimal_created", events_log);

            let inside_range = inner_swept_limit(&p.a, &p.e, &p.mass, cloud_eccentricity);
            let outside_range = outer_swept_limit(&p.a, &p.e, &p.mass, cloud_eccentricity);
            let dust_density = dust_density(dust_density_coeff, stellar_mass, &p.a);
            let crit_mass = critical_limit(b, &p.a, &p.e, stellar_luminosity);

            if dust_availible(dust_bands, &inside_range, &outside_range) {
                accrete_dust(
                    &mut p.mass,
                    &p.a,
                    &p.e,
                    &crit_mass,
                    dust_bands,
                    cloud_eccentricity,
                    &dust_density,
                    k,
                );

                let min = inner_swept_limit(&p.a, &p.e, &p.mass, cloud_eccentricity);
                let max = outer_swept_limit(&p.a, &p.e, &p.mass, cloud_eccentricity);

                update_dust_lanes(dust_bands, min, max, &p.mass, &crit_mass);
                compress_dust_lanes(dust_bands);

                dust_bands.event("dust_bands_updated", events_log);

                if p.mass > crit_mass {
                    p.is_gas_giant = true;
                    p.event("planetesimal_to_gas_giant", events_log);
                }

                p.orbit_clearing = clearing_neightbourhood(&p.mass, &p.a, stellar_mass);
                if p.orbit_clearing < 1.0 {
                    p.is_dwarf_planet = true;
                }
                p.orbit_zone = orbital_zone(stellar_luminosity, p.distance_to_primary_star);
                p.radius = kothari_radius(&p.mass, &p.is_gas_giant, &p.orbit_zone);

                p.event("planetesimal_updated", events_log);

                planets.push(p);
                planets.sort_by(|p1, p2| p1.a.partial_cmp(&p2.a).unwrap());
                coalesce_planetesimals(stellar_luminosity, stellar_mass, planets, rng, events_log);
            }

            *dust_left = dust_availible(
                dust_bands,
                planetesimal_inner_bound,
                planetesimal_outer_bound,
            );
        }
    }

    pub fn post_accretion(
        &mut self,
        intensity: u32,
        rng: &mut dyn RngCore,
        events_log: &mut AccreteEvents,
    ) {
        self.event("post_accretion_started", events_log);

        let Self {
            primary_star,
            planets,
            ..
        } = self;

        let mut weights = Vec::new();
        for p in planets.iter_mut() {
            weights.push(p.mass * p.a);
        }

        let dist = WeightedIndex::new(&weights).unwrap();
        for _i in 0..intensity {
            let p = &mut planets[dist.sample(rng)];
            let Planetesimal { a, e, mass, .. } = p;
            let r_inner = inner_effect_limit(a, e, mass);
            let r_outer = outer_effect_limit(a, e, mass);
            let mut outer_body = Planetesimal::random_outer_body(&r_inner, &r_outer, rng);

            outer_body.event("outer_body_injected", events_log);

            planetesimals_intersect(
                &mut outer_body,
                p,
                &primary_star.stellar_luminosity,
                &primary_star.stellar_mass,
                rng,
                events_log,
            );
        }
    }

    pub fn process_planets(&mut self, rng: &mut dyn RngCore) {
        let System {
            primary_star,
            planets,
            ..
        } = self;
        let PrimaryStar {
            stellar_luminosity,
            stellar_mass,
            main_seq_age,
            ecosphere,
            ..
        } = primary_star;

        for planet in planets.iter_mut() {
            planet.derive_planetary_environment(
                stellar_luminosity,
                stellar_mass,
                main_seq_age,
                ecosphere,
                rng,
            );
            for moon in planet.moons.iter_mut() {
                moon.derive_planetary_environment(
                    stellar_luminosity,
                    &planet.mass,
                    main_seq_age,
                    ecosphere,
                    rng,
                );
            }
        }
    }
}

/// Check planetesimal coalescence
pub fn coalesce_planetesimals(
    primary_star_luminosity: &f64,
    primary_star_mass: &f64,
    planets: &mut Vec<Planetesimal>,
    rng: &mut dyn RngCore,
    events_log: &mut AccreteEvents,
) {
    let mut next_planets = Vec::new();
    for (i, p) in planets.iter_mut().enumerate() {
        if i == 0 {
            next_planets.push(p.clone());
        } else if let Some(prev_p) = next_planets.last_mut() {
            if check_orbits_intersect(p.a, p.e, p.mass, prev_p.a, prev_p.e, prev_p.mass) {
                planetesimals_intersect(
                    p,
                    prev_p,
                    primary_star_luminosity,
                    primary_star_mass,
                    rng,
                    events_log,
                );
            } else {
                next_planets.push(p.clone());
            }
        }
    }
    *planets = next_planets;
}

/// Two planetesimals intersect
pub fn planetesimals_intersect(
    p: &mut Planetesimal,
    prev_p: &mut Planetesimal,
    primary_star_luminosity: &f64,
    primary_star_mass: &f64,
    rng: &mut dyn RngCore,
    events_log: &mut AccreteEvents,
) {
    // Moon is not likely to capture other moon in a presence of planet
    if p.is_moon {
        *prev_p = coalesce_two_planets(prev_p, p, events_log);
    } else {
        // Check for larger/smaller planetesimal
        let (larger, smaller) = match p.mass >= prev_p.mass {
            true => (p.clone(), prev_p.clone()),
            false => (prev_p.clone(), p.clone()),
        };
        let roche_limit = roche_limit_au(&larger.mass, &smaller.mass, &smaller.radius);
        // Planetesimals collide or one capture another as moon
        if (prev_p.a - p.a).abs() <= roche_limit * 2.0 {
            *prev_p = coalesce_two_planets(prev_p, p, events_log);
        } else {
            *prev_p = capture_moon(&larger, &smaller, primary_star_mass, rng, events_log);
            prev_p
                .moons
                .sort_by(|p1, p2| p1.a.partial_cmp(&p2.a).unwrap());
            coalesce_planetesimals(
                primary_star_luminosity,
                primary_star_mass,
                &mut prev_p.moons,
                rng,
                events_log,
            );
            moons_to_rings(prev_p, events_log);
        }
    }
}

/// Check if planetesimals have an overlapping orbits
fn check_orbits_intersect(
    p1_a: f64,
    p1_e: f64,
    p1_mass: f64,
    p2_a: f64,
    p2_e: f64,
    p2_mass: f64,
) -> bool {
    let diff = p2_a - p1_a;
    let (dist1, dist2) = match diff > 0.0 {
        true => {
            let dist1 = outer_effect_limit(&p1_a, &p1_e, &p1_mass) - p1_a;
            let dist2 = p2_a - inner_effect_limit(&p2_a, &p2_e, &p2_mass);
            (dist1, dist2)
        }
        false => {
            let dist1 = p1_a - inner_effect_limit(&p1_a, &p1_e, &p1_mass);
            let dist2 = outer_effect_limit(&p2_a, &p2_e, &p2_mass) - p2_a;
            (dist1, dist2)
        }
    };
    diff.abs() < dist1.abs() || diff.abs() < dist2.abs()
}

/// Two planetesimals collide and form one planet
fn coalesce_two_planets(
    a: &Planetesimal,
    b: &Planetesimal,
    events_log: &mut AccreteEvents,
) -> Planetesimal {
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
    coalesced.is_gas_giant = a.is_gas_giant || b.is_gas_giant;
    coalesced.radius = kothari_radius(
        &coalesced.mass,
        &coalesced.is_gas_giant,
        &coalesced.orbit_zone,
    );
    coalesced.has_collision = true;

    let event_type = match a.is_moon && b.is_moon {
        true => format!("moons_coalesced:{}:{}", a.id, b.id),
        false => format!("planetesimals_coalesced:{}:{}", a.id, b.id),
    };

    coalesced.event(event_type.as_str(), events_log);

    coalesced
}

/// Larger planetsimal capture smaller as moon
fn capture_moon(
    larger: &Planetesimal,
    smaller: &Planetesimal,
    stellar_mass: &f64,
    rng: &mut dyn RngCore,
    events_log: &mut AccreteEvents,
) -> Planetesimal {
    let mut planet = larger.clone();
    let mut moon = smaller.clone();
    moon.is_moon = true;
    let moon_id = moon.id.clone();

    // Recalcualte planetary axis
    let new_mass = planet.mass + moon.mass;
    let new_axis = new_mass / (planet.mass / planet.a + moon.mass / moon.a);
    let term1 = planet.mass * (planet.a * (1.0 - planet.e.powf(2.0))).sqrt();
    let term2 = moon.mass * (moon.a * (1.0 - moon.e.powf(2.0))).sqrt();
    let term3 = (term1 + term2) / (new_mass * new_axis.sqrt());
    let term4 = 1.0 - term3.powf(2.0);
    let new_eccn = term4.abs().sqrt();
    planet.a = new_axis;
    planet.e = new_eccn;
    planet.distance_to_primary_star = new_axis;
    planet.hill_sphere = hill_sphere_au(&planet.a, &planet.e, &planet.mass, stellar_mass);

    // Add moon to planetary moons, recalculate disturbed orbits of moons
    planet.moons.append(&mut moon.moons);
    planet.moons.push(moon);

    for m in planet.moons.iter_mut() {
        m.a = rng.gen_range(0.0..planet.hill_sphere);
        m.e = random_eccentricity(rng);
        m.b = m.a * (1.0 - m.e.powf(2.0)).sqrt();
        m.distance_to_primary_star = planet.a;
    }

    planet.event(
        format!("planetesimal_capture_moon:{}:{}", planet.id, moon_id).as_str(),
        events_log,
    );

    planet
}

fn moons_to_rings(planet: &mut Planetesimal, events_log: &mut AccreteEvents) {
    let mut next_moons = Vec::new();
    for m in planet.moons.iter_mut() {
        let roche_limit = roche_limit_au(&planet.mass, &m.mass, &m.radius);
        let moon_perhelion = perihelion_distance(&m.a, &m.e);
        if moon_perhelion <= roche_limit * 2.0 {
            let ring = Ring::from_planet(roche_limit, m);
            ring.event(
                format!("moon_to_ring:{}:{}", planet.id, m.id).as_str(),
                events_log,
            );
            planet.rings.push(ring);
        } else {
            next_moons.push(m.clone());
        }
    }

    planet.moons = next_moons;
}

fn stellar_dust_limit(stellar_mass_ratio: &f64) -> f64 {
    200.0 * stellar_mass_ratio.powf(1.0 / 3.0)
}

/// "...the semimajor axes of planetary nuclei can never be greater than 50 distance units, which effectively sets an outer boundary to the problem. An inner boundary was also established, arbitrarily at 0.3 distance unit. (More than 92 percent of the total cloud mass lies between these bounds.)"
fn outermost_planet(stellar_mass_ratio: &f64) -> f64 {
    50.0 * stellar_mass_ratio.powf(0.33)
}

fn innermost_planet(stellar_mass_ratio: &f64) -> f64 {
    0.3 * stellar_mass_ratio.powf(0.33)
}

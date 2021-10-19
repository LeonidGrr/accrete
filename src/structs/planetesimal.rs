use crate::consts::*;
use crate::enviro::*;
use crate::structs::*;
use crate::utils::*;

use rand::{Rng, RngCore};
use serde::Serialize;

// http://orbitsimulator.com/formulas/
#[derive(Debug, Clone, Serialize)]
pub struct Planetesimal {
    // "In an anonymous footnote to his 1766 translation of Charles Bonnet's Contemplation de la Nature, the astronomer Johann Daniel Titius of Wittenberg noted an apparent pattern in the layout of the planets, now known as the Titius-Bode Law. If one began a numerical sequence at 0, then included 3, 6, 12, 24, 48, etc., doubling each time, and added four to each number and divided by 10, this produced a remarkably close approximation to the radii of the orbits of the known planets as measured in astronomic units."
    // axis, AU
    pub a: f64,
    // eccentricity of the orbit, unitless
    pub e: f64,
    pub distance_to_primary_star: f64,
    pub mass: f64,
    pub earth_masses: f64,
    pub is_gas_giant: bool,
    pub orbit_zone: i32,
    // equatorial radius, km
    pub radius: f64,
    pub earth_radii: f64,
    // density, g/cc
    pub density: f64,
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
    pub volatile_gas_inventory: f64,
    pub greenhouse_effect: bool,
    pub albedo: f64,
    pub is_tidally_locked: bool,
    pub surface_pressure_bar: f64,
    pub surface_temp_kelvin: f64,
    pub day_temp_kelvin: f64,
    pub night_temp_kelvin: f64,
    pub boiling_point_kelvin: f64,
    pub hydrosphere: f64,
    pub cloud_cover: f64,
    pub ice_cover: f64,
    pub moons: Vec<Planetesimal>,
    pub rings: Vec<Ring>,
    pub is_moon: bool,
    // orbit clearing less than 1.0 is threshold between planet and dearf planet
    pub orbit_clearing: f64,
    // dwarf planet may have an astroid field or other objects on its orbit
    pub is_dwarf_planet: bool,
    pub hill_sphere: f64,
    pub tectonic_activity: bool,
    pub magnetosphere: bool,
    // if planet had collision with other objects
    pub has_collision: bool,
}

impl Planetesimal {
    pub fn new(
        planetesimal_inner_bound: &f64,
        planetesimal_outer_bound: &f64,
        rng: &mut dyn RngCore,
    ) -> Self {
        let a = rng.gen_range(*planetesimal_inner_bound..*planetesimal_outer_bound);
        let e = random_eccentricity(rng);

        Planetesimal {
            a,
            e,
            distance_to_primary_star: a,
            mass: PROTOPLANET_MASS,
            earth_masses: 0.0,
            orbit_zone: 0,
            radius: 0.0,
            earth_radii: 0.0,
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
            day_temp_kelvin: 0.0,
            night_temp_kelvin: 0.0,
            surface_pressure_bar: 0.0,
            boiling_point_kelvin: 0.0,
            hydrosphere: 0.0,
            cloud_cover: 0.0,
            ice_cover: 0.0,
            moons: Vec::new(),
            rings: Vec::new(),
            length_of_year: 0.0,
            escape_velocity_km_per_sec: 0.0,
            is_tidally_locked: false,
            is_moon: false,
            is_gas_giant: false,
            is_dwarf_planet: false,
            orbit_clearing: 0.0,
            hill_sphere: 0.0,
            tectonic_activity: false,
            magnetosphere: false,
            has_collision: false,
        }
    }

    pub fn derive_planetary_environment(
        &mut self,
        stellar_luminosity: &f64,
        stellar_mass: &f64,
        main_seq_age: &f64,
        ecosphere: &(f64, f64),
        rng: &mut dyn RngCore,
    ) {
        if !self.is_moon {
            self.orbit_zone = orbital_zone(stellar_luminosity, self.a);
        }
        if self.is_gas_giant {
            self.density = empirical_density(
                &self.mass,
                &self.distance_to_primary_star,
                &ecosphere.1,
                &self.is_gas_giant,
            );
            self.radius = volume_radius(&self.mass, &self.density);
        } else {
            self.radius = kothari_radius(&self.mass, &self.is_gas_giant, &self.orbit_zone);
            self.density = volume_density(&self.mass, &self.radius);
        }
        self.orbital_period_days = period(&self.a, &self.mass, stellar_mass);
        self.day_hours = day_length(self, stellar_mass, main_seq_age);
        self.axial_tilt = inclination(&self.a, rng);
        self.escape_velocity = escape_vel(&self.mass, &self.radius);
        self.surface_accel = acceleration(&self.mass, &self.radius);
        self.rms_velocity = rms_vel(&MOLECULAR_NITROGEN, &self.a);
        self.molecule_weight = molecule_limit(&self.mass, &self.radius);

        if self.is_gas_giant {
            self.surface_grav = INCREDIBLY_LARGE_NUMBER;
            self.greenhouse_effect = false;
            self.volatile_gas_inventory = INCREDIBLY_LARGE_NUMBER;
            self.surface_pressure_bar = INCREDIBLY_LARGE_NUMBER;
            self.boiling_point_kelvin = INCREDIBLY_LARGE_NUMBER;
            self.hydrosphere = INCREDIBLY_LARGE_NUMBER;
            self.albedo = about(GAS_GIANT_ALBEDO, 0.1, rng);
            self.surface_temp_kelvin = INCREDIBLY_LARGE_NUMBER;
        } else {
            self.surface_grav = gravity(&self.surface_accel);
            self.greenhouse_effect = greenhouse(
                &self.distance_to_primary_star,
                &self.orbit_zone,
                &self.surface_pressure_bar,
                &ecosphere.1,
            );
            self.volatile_gas_inventory = vol_inventory(
                &self.mass,
                &self.escape_velocity,
                &self.rms_velocity,
                stellar_mass,
                &self.orbit_zone,
                &self.greenhouse_effect,
                rng,
            );
            self.surface_pressure_bar = pressure(
                &self.volatile_gas_inventory,
                &self.radius,
                &self.surface_grav,
            );
            if self.surface_pressure_bar == 0.0 {
                self.boiling_point_kelvin = 0.0;
            } else {
                self.boiling_point_kelvin = boiling_point_kelvin(&self.surface_pressure_bar);
                iterate_surface_temp(self, &ecosphere.1, rng);
            }
        }

        get_day_night_temp_kelvin(self);

        self.hill_sphere = hill_sphere_au(&self.a, &self.e, &self.mass, stellar_mass);
        self.earth_masses = get_earth_mass(self.mass);
        self.earth_radii = self.radius / EARTH_RADIUS_IN_KM;
        self.length_of_year = self.orbital_period_days / 365.25;
        self.escape_velocity_km_per_sec = self.escape_velocity / CM_PER_KM;
        self.is_tidally_locked = check_tidal_lock(self.day_hours, self.orbital_period_days);

        // Probability of planet have a tectonic activity:
        // 1) planet locked close to the star due to one side of planet being hotter than other
        // 2) planet mass is within 0.5-1.5 of Earth mass
        // 3) planet had collision
        if (!self.is_gas_giant)
            && (self.is_tidally_locked
                || self.earth_masses > 0.5 && self.earth_masses < 1.5
                || self.has_collision)
        {
            self.tectonic_activity = true;
        }

        // Probability of planet have a tectonic activity:
        // 1) planet is gas giant
        // 2) planet have tectonic activity
        // 3) planet have atmosphere
        if self.is_gas_giant || self.tectonic_activity || self.surface_pressure_bar > 0.0 {
            self.magnetosphere = true;
        }

        for moon in self.moons.iter_mut() {
            moon.derive_planetary_environment(
                stellar_luminosity,
                &self.mass,
                main_seq_age,
                ecosphere,
                rng,
            );
        }
    }

    pub fn random_outer_body(
        planetesimal_inner_bound: &f64,
        planetesimal_outer_bound: &f64,
        rng: &mut dyn RngCore,
    ) -> Self {
        let mut random_body =
            Planetesimal::new(planetesimal_inner_bound, planetesimal_outer_bound, rng);
        random_body.mass = rng.gen_range(PLANETESIMAL_MASS..PROTOPLANET_MASS * 1.0e5);
        random_body.orbit_zone = 3;
        random_body.radius = kothari_radius(
            &random_body.mass,
            &random_body.is_gas_giant,
            &random_body.orbit_zone,
        );

        random_body
    }

    pub fn random_planet(
        stellar_luminosity: f64,
        stellar_mass: f64,
        a: f64,
        e: f64,
        mass: f64,
        post_accretion_intensity: u32,
        rng: &mut dyn RngCore,
    ) -> Planetesimal {
        let main_seq_age = main_sequence_age(stellar_mass, stellar_luminosity);
        let stellar_radius_au = stellar_radius_au(stellar_mass);
        let stellar_surface_temp = stellar_surface_temp(stellar_radius_au, stellar_luminosity);
        let spectral_class = spectral_class(&stellar_surface_temp);
        let ecosphere = ecosphere(&stellar_luminosity, &spectral_class);

        let mut is_gas_giant = false;
        let crit_mass = critical_limit(&B, &a, &e, &stellar_luminosity);
        if mass > crit_mass {
            is_gas_giant = true;
        }

        let mut is_dwarf_planet = false;
        let orbit_clearing = clearing_neightbourhood(&mass, &a, &stellar_mass);
        if orbit_clearing < 1.0 {
            is_dwarf_planet = true;
        }

        let mut random_planet = Planetesimal {
            a,
            e,
            distance_to_primary_star: a,
            mass,
            earth_masses: 0.0,
            orbit_zone: 0,
            radius: 0.0,
            earth_radii: 0.0,
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
            day_temp_kelvin: 0.0,
            night_temp_kelvin: 0.0,
            surface_pressure_bar: 0.0,
            boiling_point_kelvin: 0.0,
            hydrosphere: 0.0,
            cloud_cover: 0.0,
            ice_cover: 0.0,
            moons: Vec::new(),
            rings: Vec::new(),
            length_of_year: 0.0,
            escape_velocity_km_per_sec: 0.0,
            is_tidally_locked: false,
            is_moon: false,
            is_gas_giant,
            is_dwarf_planet,
            orbit_clearing,
            hill_sphere: 0.0,
            tectonic_activity: false,
            magnetosphere: false,
            has_collision: false,
        };

        for _i in 0..post_accretion_intensity {
            let Planetesimal { a, e, mass, .. } = random_planet;
            let r_inner = inner_effect_limit(&a, &e, &mass);
            let r_outer = outer_effect_limit(&a, &e, &mass);
            let mut outer_body = Planetesimal::random_outer_body(&r_inner, &r_outer, rng);
            planetesimals_intersect(
                &mut outer_body,
                &mut random_planet,
                &stellar_luminosity,
                &stellar_mass,
                rng,
            );
        }

        random_planet.derive_planetary_environment(
            &stellar_luminosity,
            &stellar_mass,
            &main_seq_age,
            &ecosphere,
            rng,
        );

        random_planet
    }
}

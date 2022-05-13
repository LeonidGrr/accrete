use crate::{
    consts::{
        A_SCALE_FACTOR, PLANET_PERIOD_FACTOR, PLANET_RADIUS_SCALE_FACTOR, TRAIL_LENGTH,
        TRAIL_MINIMUM_ANGLE, UPDATE_A_LIMIT, UPDATE_A_RATE, UPDATE_E_LIMIT, UPDATE_E_RATE,
    },
    planet_model::{PlanetId, PlanetPosition},
};
use accrete::{enviro::period, Planetesimal};
use bevy::{math::vec3, prelude::*};
use bevy_polyline::prelude::*;
use ringbuffer::{ConstGenericRingBuffer, RingBufferExt, RingBufferWrite};

#[derive(Component, Clone, Default, Debug)]
pub struct Trail(ConstGenericRingBuffer<Vec3, TRAIL_LENGTH>);

#[derive(Debug, Clone, Bundle)]
pub struct Orbit {
    pub parameters: OrbitalParameters,
    pub trail: Trail,
}

impl Orbit {
    pub fn remove_orbital_lines_resources(
        polyline_handle: &Handle<Polyline>,
        polylines: &mut ResMut<Assets<Polyline>>,
    ) {
        polylines.remove(polyline_handle);
    }

    pub fn new(parameters: OrbitalParameters) -> Self {
        Orbit {
            parameters,
            trail: Trail(ConstGenericRingBuffer::<Vec3, TRAIL_LENGTH>::new()),
        }
    }
}

#[derive(Debug, Clone, Component)]
pub struct OrbitalParameters {
    pub a: f32,
    pub b: f32,
    pub e: f32,
    pub focus: f32,
    pub u: f32,
    pub t: f32,
}

impl OrbitalParameters {
    pub fn new(planet: &Planetesimal, parent_mass: f64) -> Self {
        let a = OrbitalParameters::scaled_a(planet.a);
        let e = planet.e as f32;
        let u = 1.0;
        let b = OrbitalParameters::get_semiminor_axis(a, e);
        let t = OrbitalParameters::get_orbital_period(a as f64, planet.mass, parent_mass);
        let focus = OrbitalParameters::get_focus(a, b);

        OrbitalParameters {
            a,
            u,
            e,
            b,
            focus,
            t,
        }
    }

    pub fn scaled_radius(raw_radius: f64) -> f32 {
        raw_radius as f32 * PLANET_RADIUS_SCALE_FACTOR
    }

    pub fn scaled_a(raw_a: f64) -> f32 {
        raw_a as f32 * A_SCALE_FACTOR
    }

    pub fn update_orbit_immediate(
        &mut self,
        target_a: f32,
        target_e: f64,
        mass: f64,
        parent_mass: f64,
    ) {
        self.a = target_a;
        self.e = target_e as f32;
        self.b = OrbitalParameters::get_semiminor_axis(self.a, self.e);
        self.focus = OrbitalParameters::get_focus(self.a, self.b);
        self.t = OrbitalParameters::get_orbital_period(self.a as f64, mass, parent_mass);
    }

    pub fn update_value_by_limit(
        current_value: &mut f32,
        target_value: f32,
        update_rate: f32,
        limit: f32,
    ) {
        let diff = (target_value - *current_value).abs();
        if diff < limit {
            *current_value = target_value;
        } else {
            match *current_value < target_value {
                true => {
                    *current_value += update_rate;
                }
                false => {
                    *current_value -= update_rate;
                }
            }
        }
    }

    pub fn update_orbit(&mut self, target_a: f32, target_e: f64, mass: f64, parent_mass: f64) {
        OrbitalParameters::update_value_by_limit(
            &mut self.a,
            target_a,
            UPDATE_A_RATE,
            UPDATE_A_LIMIT,
        );
        OrbitalParameters::update_value_by_limit(
            &mut self.e,
            target_e as f32,
            UPDATE_E_RATE,
            UPDATE_E_LIMIT,
        );
        self.b = OrbitalParameters::get_semiminor_axis(self.a, self.e);
        self.focus = OrbitalParameters::get_focus(self.a, self.b);
        self.t = OrbitalParameters::get_orbital_period(self.a as f64, mass, parent_mass);
    }

    pub fn get_focus(a: f32, b: f32) -> f32 {
        (a.powf(2.0) - b.powf(2.0)).sqrt()
    }

    pub fn get_orbital_period(a: f64, small_mass: f64, large_mass: f64) -> f32 {
        period(&a, &small_mass, &large_mass) as f32
    }

    pub fn get_semiminor_axis(a: f32, e: f32) -> f32 {
        a * (1.0 - e.powf(2.0)).sqrt()
    }

    pub fn get_orbital_position(&mut self, simulation_step: f32) -> Vec3 {
        let OrbitalParameters { a, t, e, focus, .. } = *self;

        // Relative time converted to radians
        let m = 2.0 * std::f32::consts::PI * simulation_step / t * PLANET_PERIOD_FACTOR;
        let cos_f = (m.cos() - e) / (1.0 - e * m.cos());
        let sin_f = ((1.0 - e.powf(2.0)).sqrt() * m.sin()) / (1.0 - e * m.cos());
        let r = a * (1.0 - e.powf(2.0)) / (1.0 + e * cos_f);
        let x = focus + r * cos_f;
        let z = r * sin_f;

        vec3(x, 0.0, z)
    }
}

pub struct OrbitsPlugin;

impl Plugin for OrbitsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_orbits_system);
    }
}

fn update_orbits_system(
    mut polylines: ResMut<Assets<Polyline>>,
    mut query: Query<(&PlanetPosition, &mut Trail, &Handle<Polyline>), With<PlanetId>>,
) {
    // for (global_transform, trail, polyline_handle) in query.iter() {
    //     let polyline = polylines
    //         .get_mut(polyline_handle)
    //         .expect("Failed to get orbital polyline resource");
    //     polyline.vertices.push(global_transform.0);
    // }

    query.for_each_mut(|(planet_position, mut trail, polyline_handle)| {
        if let Some(position) = trail.0.back() {
            let last_vec = *position - planet_position.0;
            let last_last_vec = if let Some(position) = trail.0.get(-2) {
                *position - planet_position.0
            } else {
                last_vec
            };

            let gt_min_angle = last_vec.dot(last_last_vec) > TRAIL_MINIMUM_ANGLE;
            if gt_min_angle {
                trail.0.push(planet_position.0);
                polylines.get_mut(polyline_handle).unwrap().vertices =
                    trail.0.iter().map(|v| *v).collect()
            } else {
                if polylines.get_mut(polyline_handle).unwrap().vertices.len() > 1 {
                    *trail.0.get_mut(-1).unwrap() = planet_position.0;
                    *polylines
                        .get_mut(polyline_handle)
                        .expect("Failed to get orbital polyline resource")
                        .vertices
                        .last_mut()
                        .expect("Failed to get orbital polyline last vertex") =
                        planet_position.0.into();
                }
            }
        } else {
            trail.0.push(planet_position.0);
            polylines
                .get_mut(polyline_handle)
                .expect("Failed to get orbital polyline resource")
                .vertices = trail.0.iter().map(|v| Vec3::from(*v)).collect()
        }
    });
}

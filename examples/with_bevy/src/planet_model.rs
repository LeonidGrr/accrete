use crate::consts::{
    A_SCALE_FACTOR, PLANET_PERIOD_FACTOR, PLANET_RADIUS_SCALE_FACTOR, UPDATE_A_LIMIT,
    UPDATE_A_RATE, UPDATE_E_LIMIT, UPDATE_E_RATE,
};
use crate::simulation_state::SimulationState;
use crate::surface::get_planet_color;
use accrete::enviro::period;
use accrete::{Planetesimal, PrimaryStar};
use bevy::{math::vec3, prelude::*, tasks::TaskPool};

#[derive(Debug, Clone, Bundle)]
pub struct PlanetModel {
    pub planet_id: PlanetId,
    pub position: PlanetPosition,
    pub orbit: Orbit,
}

impl PlanetModel {
    pub fn new(planetesimal: &Planetesimal, primary_star: &PrimaryStar) -> Self {
        let Planetesimal { id, a, e, .. } = planetesimal;
        let a = Orbit::scaled_a(*a);
        let planet_id = PlanetId(id.to_owned());
        let position = PlanetPosition(vec3(-(a - 0.001), 0.0, 0.0));
        let orbit = Orbit::new(a, *e as f32, planetesimal.mass, primary_star.stellar_mass);

        PlanetModel {
            planet_id,
            position,
            orbit,
        }
    }

    pub fn create_planet_resourses(
        commands: &mut Commands,
        state: &mut ResMut<SimulationState>,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        planet: &Planetesimal,
        primary_star: &PrimaryStar,
    ) {
        let mut planet_model = PlanetModel::new(planet, primary_star);
        planet_model
            .position
            .update_position(&mut planet_model.orbit, state.current_step);
        state.planets.insert(planet.id.to_owned(), planet.clone());
        let color = get_planet_color(&planet);

        commands
            .spawn()
            .insert_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius: Orbit::scaled_radius(planet.radius),
                    subdivisions: 32,
                })),
                material: materials.add(color.into()),
                transform: Transform::from_translation(planet_model.position.0),
                visibility: Visibility { is_visible: false },
                ..default()
            })
            .insert_bundle(planet_model);
    }

    pub fn update_planet_resources(
        mesh_handle: &Handle<Mesh>,
        material_handle: &Handle<StandardMaterial>,
        visibility: &mut Visibility,
        state: &mut ResMut<SimulationState>,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        planetesimal: &Planetesimal,
    ) {
        if let Some(mesh) = meshes.get_mut(mesh_handle) {
            let next_mesh = Mesh::from(shape::Icosphere {
                radius: Orbit::scaled_radius(planetesimal.radius),
                subdivisions: 32,
            });
            mesh.clone_from(&next_mesh);
        }

        if let Some(material) = materials.get_mut(material_handle) {
            let color = get_planet_color(planetesimal);
            material.clone_from(&color.into());
        }

        visibility.is_visible = true;
        state
            .planets
            .insert(planetesimal.id.to_owned(), planetesimal.clone());
    }

    pub fn remove_planet_resources(
        entity: Entity,
        id: &PlanetId,
        mesh_handle: &Handle<Mesh>,
        material_handle: &Handle<StandardMaterial>,
        commands: &mut Commands,
        state: &mut ResMut<SimulationState>,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        commands.entity(entity).despawn();
        materials.remove(material_handle);
        meshes.remove(mesh_handle);
        state.planets.remove(&id.0);
    }
}

#[derive(Debug, Clone, Component)]
pub struct PlanetId(pub String);

#[derive(Debug, Clone, Copy, Component)]
pub struct PlanetPosition(pub Vec3);

impl PlanetPosition {
    pub fn update_position(&mut self, orbit: &mut Orbit, simulation_step: f32) {
        let next_position = orbit.get_orbital_position(simulation_step);
        self.0.x = next_position.x;
        self.0.z = next_position.z;
        // TODO speed up near star
    }
}

#[derive(Debug, Clone, Component)]
pub struct Orbit {
    pub a: f32,
    pub b: f32,
    pub e: f32,
    pub focus: f32,
    pub u: f32,
    pub t: f32,
}

impl Orbit {
    pub fn new(a: f32, e: f32, mass: f64, parent_mass: f64) -> Self {
        let u = 1.0;
        let b = Orbit::get_semiminor_axis(a, e);

        Orbit {
            a,
            u,
            e,
            b,
            focus: Orbit::get_focus(a, b),
            t: Orbit::get_orbital_period(a as f64, mass, parent_mass),
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
        self.b = Orbit::get_semiminor_axis(self.a, self.e);
        self.focus = Orbit::get_focus(self.a, self.b);
        self.t = Orbit::get_orbital_period(self.a as f64, mass, parent_mass);
    }

    fn update_value_by_limit(
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
        Orbit::update_value_by_limit(&mut self.a, target_a, UPDATE_A_RATE, UPDATE_A_LIMIT);
        Orbit::update_value_by_limit(&mut self.e, target_e as f32, UPDATE_E_RATE, UPDATE_E_LIMIT);
        self.b = Orbit::get_semiminor_axis(self.a, self.e);
        self.focus = Orbit::get_focus(self.a, self.b);
        self.t = Orbit::get_orbital_period(self.a as f64, mass, parent_mass);
    }

    fn get_focus(a: f32, b: f32) -> f32 {
        (a.powf(2.0) - b.powf(2.0)).sqrt()
    }

    fn get_orbital_period(a: f64, small_mass: f64, large_mass: f64) -> f32 {
        period(&a, &small_mass, &large_mass) as f32
    }

    fn get_semiminor_axis(a: f32, e: f32) -> f32 {
        a * (1.0 - e.powf(2.0)).sqrt()
    }

    fn get_orbital_position(&mut self, simulation_step: f32) -> Vec3 {
        let Orbit { a, t, e, focus, .. } = *self;

        // Relative time converted to radians
        let m = 2.0 * std::f32::consts::PI * simulation_step / t * PLANET_PERIOD_FACTOR;
        let cos_f = (m.cos() - e) / (1.0 - e * m.cos());
        let sin_f = ((1.0 - e.powf(2.0)).sqrt() * m.sin()) / (1.0 - e * m.cos());
        let r = a * (1.0 - e.powf(2.0)) / (1.0 + e * cos_f);
        let x = focus + r * cos_f;
        let z = r * sin_f;

        // let x = focus + (a * current_t.cos() as f32);
        // let z = b * current_t.sin() as f32;

        vec3(x, 0.0, z)
    }
}

pub struct PlanetsPlugin;

impl Plugin for PlanetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_planets_position_system);
    }
}

fn update_planets_position_system(
    state: Res<SimulationState>,
    mut query: Query<(&mut PlanetPosition, &mut Orbit, &mut Transform)>,
) {
    let taskpool = TaskPool::new();
    query.par_for_each_mut(
        &taskpool,
        4,
        |(mut planet_position, mut orbit, mut transform)| {
            planet_position.update_position(&mut orbit, state.current_step);
            transform.translation.x = planet_position.0.x;
            transform.translation.z = planet_position.0.z;
        },
    );
}

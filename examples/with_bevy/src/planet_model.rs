use crate::orbit::{Orbit, OrbitalParameters};
use crate::simulation_state::SimulationState;
use crate::surface::get_planet_color;
use accrete::{Planetesimal, PrimaryStar};
use bevy::{math::vec3, prelude::*, tasks::TaskPool};

#[derive(Debug, Clone, Bundle)]
pub struct PlanetModel {
    pub planet_id: PlanetId,
    pub position: PlanetPosition,
}

impl From<&Planetesimal> for PlanetModel {
    fn from(planetesimal: &Planetesimal) -> Self {
        let Planetesimal { id, a, .. } = planetesimal;
        let a = OrbitalParameters::scaled_a(*a);
        let planet_id = PlanetId(id.to_owned());
        let position = PlanetPosition(vec3(-(a - 0.001), 0.0, 0.0));

        PlanetModel {
            planet_id,
            position,
        }
    }
}

impl PlanetModel {
    pub fn create_planet_resourses(
        commands: &mut Commands,
        state: &mut ResMut<SimulationState>,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        planet: &Planetesimal,
        primary_star: &PrimaryStar,
    ) {
        let mut planet_model = PlanetModel::from(planet);
        let mut orbital_parameters = OrbitalParameters::new(planet, primary_star.stellar_mass);
        planet_model
            .position
            .update_position(&mut orbital_parameters, state.current_step);
        state.planets.insert(planet.id.to_owned(), planet.clone());
        let color = get_planet_color(&planet);

        commands
            .spawn()
            .insert_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius: OrbitalParameters::scaled_radius(planet.radius),
                    subdivisions: 32,
                })),
                material: materials.add(color.into()),
                transform: Transform::from_translation(planet_model.position.0),
                visibility: Visibility { is_visible: false },
                ..default()
            })
            .insert_bundle(Orbit {
                parameters: orbital_parameters,
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
                radius: OrbitalParameters::scaled_radius(planetesimal.radius),
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
    pub fn update_position(
        &mut self,
        orbital_parameters: &mut OrbitalParameters,
        simulation_step: f32,
    ) {
        let next_position = orbital_parameters.get_orbital_position(simulation_step);
        self.0.x = next_position.x;
        self.0.z = next_position.z;
        // TODO speed up near star
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
    mut query: Query<(&mut PlanetPosition, &mut OrbitalParameters, &mut Transform)>,
) {
    let taskpool = TaskPool::new();
    query.par_for_each_mut(
        &taskpool,
        4,
        |(mut planet_position, mut orbital_parameters, mut transform)| {
            planet_position.update_position(&mut orbital_parameters, state.current_step);
            transform.translation.x = planet_position.0.x;
            transform.translation.z = planet_position.0.z;
        },
    );
}

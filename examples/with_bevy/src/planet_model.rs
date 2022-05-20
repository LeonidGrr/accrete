use crate::consts::TRAIL_LENGTH;
use crate::orbit::{Orbit, OrbitBundle};
use crate::simulation_state::SimulationState;
use crate::surface::get_planet_color;
use accrete::{Planetesimal, PrimaryStar};
use bevy::{math::vec3, prelude::*};
use bevy_polyline::prelude::*;

#[derive(Debug, Clone, Bundle)]
pub struct PlanetModel {
    pub planet_id: PlanetId,
    pub position: PlanetPosition,
    pub planet_data: PlanetData,
}

impl From<&Planetesimal> for PlanetModel {
    fn from(planetesimal: &Planetesimal) -> Self {
        let Planetesimal { id, a, .. } = planetesimal;
        let a = Orbit::scaled_a(*a);
        let planet_id = PlanetId(id.to_owned());
        let position = PlanetPosition(vec3(-(a - 0.001), 0.0, 0.0));

        PlanetModel {
            planet_id,
            position,
            planet_data: PlanetData(planetesimal.clone()),
        }
    }
}

impl PlanetModel {
    pub fn create_planet(
        commands: &mut Commands,
        state: &mut ResMut<SimulationState>,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        polyline_materials: &mut ResMut<Assets<PolylineMaterial>>,
        polylines: &mut ResMut<Assets<Polyline>>,
        planet: &Planetesimal,
        primary_star: &PrimaryStar,
    ) {
        let mut planet_model = PlanetModel::from(planet);
        let mut orbital_parameters = Orbit::new(planet, primary_star.stellar_mass);
        let position = planet_model
            .position
            .update_position(&mut orbital_parameters, state.current_step);
        let color = get_planet_color(planet);

        commands
            .spawn()
            .insert_bundle(PolylineBundle {
                polyline: polylines.add(Polyline {
                    vertices: Vec::with_capacity(TRAIL_LENGTH),
                }),
                material: polyline_materials.add(PolylineMaterial {
                    width: 0.1,
                    color: Color::WHITE,
                    perspective: true,
                }),
                visibility: Visibility { is_visible: false },
                ..default()
            })
            .insert_bundle(OrbitBundle::new(orbital_parameters))
            .insert_bundle(planet_model)
            .with_children(|parent| {
                parent
                    .spawn_bundle(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Icosphere {
                            radius: Orbit::scaled_radius(planet.radius),
                            subdivisions: 32,
                        })),
                        material: materials.add(color.into()),
                        transform: Transform::from_translation(position),
                        visibility: Visibility { is_visible: false },
                        ..default()
                    });
            });
    }

    pub fn update_planet_model(
        children: &Children,
        child_query: &mut Query<
            (&Handle<Mesh>, &Handle<StandardMaterial>, &mut Visibility),
            Without<PlanetId>,
        >,
        visibility: &mut Visibility,
        planet_data: &mut PlanetData,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        planetesimal: &Planetesimal,
    ) {
        for &child in children.iter() {
            if let Ok((mesh_handle, material_handle, mut mesh_visibility)) =
                child_query.get_mut(child)
            {
                if let Some(mesh) = meshes.get_mut(mesh_handle) {
                    let next_mesh = Mesh::from(shape::Icosphere {
                        radius: 0.2,
                        // radius: Orbit::scaled_radius(planetesimal.radius),
                        subdivisions: 32,
                    });
                    mesh.clone_from(&next_mesh);
                }

                if let Some(material) = materials.get_mut(material_handle) {
                    let color = get_planet_color(planetesimal);
                    material.clone_from(&color.into());
                }

                visibility.is_visible = true;
                mesh_visibility.is_visible = true;
                planet_data.0 = planetesimal.clone();
            }
        }
    }

    pub fn remove_planet_resources(
        children: &Children,
        child_query: &mut Query<
            (&Handle<Mesh>, &Handle<StandardMaterial>, &mut Visibility),
            Without<PlanetId>,
        >,
        entity: Entity,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        for &child in children.iter() {
            if let Ok((mesh_handle, material_handle, _)) = child_query.get_mut(child) {
                commands.entity(entity).despawn();
                materials.remove(material_handle);
                meshes.remove(mesh_handle);
            }
        }
    }
}

#[derive(Debug, Clone, Component)]
pub struct PlanetId(pub String);

#[derive(Debug, Clone, Copy, Component)]
pub struct PlanetPosition(pub Vec3);

#[derive(Debug, Clone, Component)]
pub struct SourcePlanet;

#[derive(Debug, Clone, Component)]
pub struct TargetPlanet;

#[derive(Debug, Clone, Component)]
pub struct PlanetData(pub Planetesimal);

impl PlanetPosition {
    pub fn update_position(
        &mut self,
        orbital_parameters: &mut Orbit,
        simulation_step: f32,
    ) -> Vec3 {
        let next_position = orbital_parameters.get_orbital_position(simulation_step);
        self.0.x = next_position.x;
        self.0.z = next_position.z;
        next_position
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
    mut query: Query<(&mut PlanetPosition, &mut Orbit, &Children)>,
    mut child_query: Query<&mut Transform>,
) {
    query.for_each_mut(|(mut planet_position, mut orbital_parameters, children)| {
        planet_position.update_position(&mut orbital_parameters, state.current_step);
        for &child in children.iter() {
            if let Ok(mut transform) = child_query.get_mut(child) {
                transform.translation.x = planet_position.0.x;
                transform.translation.z = planet_position.0.z;
            }
        }
    });
}

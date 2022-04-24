use crate::{planet_model::PlanetsPlugin, simulation_state::SimulationStatePlugin, ui::UIPlugin};
use accrete::{events::AccreteEvent, PrimaryStar};
use bevy::prelude::*;

pub fn run(log: Vec<AccreteEvent>, primary_star: PrimaryStar) {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Accrete simulation".to_string(),
            ..default()
        })
        .insert_resource(primary_star)
        .insert_resource(log)
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .add_plugin(UIPlugin)
        .add_plugin(PlanetsPlugin)
        .add_plugin(SimulationStatePlugin)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        point_light: PointLight {
            intensity: 1600.0,
            color: Color::RED,
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(50.1, 50.0, 50.1).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

use crate::{
    active_event::ActiveEvent,
    planet_model::PlanetsPlugin,
    simulation_state::{EventPlugin, SimulationState},
};
use accrete::events::AccreteEvent;
use bevy::prelude::*;

pub fn run(log: Vec<AccreteEvent>) {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Accrete simulation".to_string(),
            ..Default::default()
        })
        .insert_resource(SimulationState::new())
        .insert_resource(ActiveEvent::default())
        .insert_resource(log)
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .add_plugin(EventPlugin)
        .add_plugin(PlanetsPlugin)
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
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    });
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-50.0, 50.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

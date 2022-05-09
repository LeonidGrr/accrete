use crate::{
    dust_model::DustPlugin, planet_model::PlanetsPlugin, simulation_state::SimulationStatePlugin,
    ui::UIPlugin,
};
use accrete::{events::AccreteEvent, PrimaryStar};
use bevy::prelude::*;
use bevy_polyline::prelude::*;
// use bevy_inspector_egui::WorldInspectorPlugin;

pub fn run(log: Vec<AccreteEvent>, primary_star: PrimaryStar) {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Accrete simulation".to_string(),
            ..default()
        })
        .insert_resource(primary_star)
        .insert_resource(log)
        .add_startup_system(setup_scene)
        .add_plugins(DefaultPlugins)
        .add_plugin(PolylinePlugin)
        .add_plugin(DustPlugin)
        // .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(UIPlugin)
        .add_plugin(PlanetsPlugin)
        .add_plugin(SimulationStatePlugin)
        .run();
}

fn setup_scene(mut commands: Commands) {
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        point_light: PointLight {
            intensity: 100.0,
            color: Color::WHITE,
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

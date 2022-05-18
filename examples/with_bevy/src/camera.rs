use bevy::prelude::*;
use smooth_bevy_cameras::{
    controllers::unreal::{UnrealCameraBundle, UnrealCameraController, UnrealCameraPlugin},
    LookTransformPlugin,
};

fn setup_camera_system(mut commands: Commands) {
    commands.spawn_bundle(UnrealCameraBundle::new(
        UnrealCameraController::default(),
        PerspectiveCameraBundle::default(),
        Vec3::new(0.0, 150.0, 0.0),
        Vec3::ZERO,
    ));
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LookTransformPlugin)
            .add_plugin(UnrealCameraPlugin::default())
            .add_startup_system(setup_camera_system);
    }
}

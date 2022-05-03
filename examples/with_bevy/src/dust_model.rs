use bevy::prelude::*;
use bevy::render::{render_resource::WgpuFeatures, settings::WgpuSettings};

#[derive(Debug, Component)]
pub struct PrimaryStar;

fn setup_dust_model(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn()
        .insert(PrimaryStar)
        .insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.2 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            ..default()
        });
}

fn update_primary_star(mut primary_star_query: Query<&mut Transform, With<PrimaryStar>>) {
    let mut primary_star_transform = primary_star_query
        .get_single_mut()
        .expect("Failed to get PrimaryStar Transform");
    primary_star_transform.rotation *= Quat::from_rotation_y(-0.1);
}

pub struct DustPlugin;

impl Plugin for DustPlugin {
    fn build(&self, app: &mut App) {
        let mut options = WgpuSettings::default();
        options
            .features
            .set(WgpuFeatures::VERTEX_WRITABLE_STORAGE, true);

        app.insert_resource(options)
            .add_startup_system(setup_dust_model)
            .add_system(update_primary_star);
    }
}

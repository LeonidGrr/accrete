use bevy::{
    prelude::*,
    render::{render_resource::WgpuFeatures, settings::WgpuSettings},
};
use bevy_hanabi::*;

#[derive(Debug, Component)]
pub struct PrimaryStar;

fn create_dust_effect(
    commands: &mut Commands,
    effects: &mut ResMut<Assets<EffectAsset>>,
    radius: f32,
) {
    let mut color_gradient = Gradient::new();
    color_gradient.add_key(0.0, Vec4::splat(1.0));
    color_gradient.add_key(0.1, Vec4::new(1.0, 1.0, 0.0, 1.0));
    color_gradient.add_key(0.4, Vec4::new(1.0, 0.0, 0.0, 1.0));
    color_gradient.add_key(1.0, Vec4::splat(0.0));

    let mut size_gradient = Gradient::new();
    size_gradient.add_key(0.0, Vec2::splat(1.0));
    size_gradient.add_key(0.3, Vec2::splat(0.6));
    size_gradient.add_key(0.6, Vec2::splat(0.3));
    size_gradient.add_key(1.0, Vec2::splat(0.0));

    let effect = effects.add(
        EffectAsset {
            name: "DustEffect".to_string(),
            capacity: 1024,
            spawner: Spawner::rate(64.0.into()),
            ..default()
        }
        .init(PositionCircleModifier {
            center: Vec3::ZERO,
            axis: Vec3::Y,
            radius,
            speed: Value::Uniform((0.0, 1.0)),
            dimension: ShapeDimension::Surface,
        })
        // .render(ParticleTextureModifier {
        //     texture: texture_handle.clone(),
        // })
        .render(ColorOverLifetimeModifier {
            gradient: color_gradient,
        })
        .render(SizeOverLifetimeModifier {
            gradient: size_gradient,
        }),
    );

    commands
        .spawn()
        .insert_bundle(ParticleEffectBundle::new(effect));
}

fn setup_dust_model_system(
    // asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    // let texture_handle: Handle<Image> = asset_server.load("cloud.png");

    create_dust_effect(&mut commands, &mut effects, 50.0);
    create_dust_effect(&mut commands, &mut effects, 45.0);
    create_dust_effect(&mut commands, &mut effects, 40.0);
    create_dust_effect(&mut commands, &mut effects, 35.0);
    create_dust_effect(&mut commands, &mut effects, 30.0);
    create_dust_effect(&mut commands, &mut effects, 25.0);
    create_dust_effect(&mut commands, &mut effects, 20.0);
    create_dust_effect(&mut commands, &mut effects, 15.0);
    create_dust_effect(&mut commands, &mut effects, 10.0);

    commands
        .spawn()
        .insert(PrimaryStar)
        .insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.2 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            ..default()
        });
}

fn update_primary_star_system(mut primary_star_query: Query<&mut Transform, With<PrimaryStar>>) {
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
            .add_plugin(HanabiPlugin)
            .add_startup_system(setup_dust_model_system)
            .add_system(update_primary_star_system);
    }
}

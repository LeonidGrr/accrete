use bevy_hanabi::*;
use bevy::{prelude::*, render::{settings::WgpuSettings, render_resource::WgpuFeatures}};
use crate::planet_model::{PlanetData, PlanetPosition};

#[derive(Debug, Component)]
pub struct PrimaryStar;

#[derive(Debug, Component)]
pub struct DustModel;

fn create_dust_effect(
    commands: &mut Commands,
    effects: &mut ResMut<Assets<EffectAsset>>,
    radius: f32,
    texture_handle: &Handle<Image>,
) -> Entity {
    let mut color_gradient = Gradient::new();
    color_gradient.add_key(0.0, Vec4::splat(1.0));
    color_gradient.add_key(0.1, Vec4::new(1.0, 1.0, 0.0, 1.0));
    color_gradient.add_key(0.8, Vec4::new(1.0, 0.0, 0.0, 1.0));
    color_gradient.add_key(1.0, Vec4::splat(0.0));

    let mut size_gradient = Gradient::new();
    size_gradient.add_key(0.0, Vec2::splat(1.0));
    size_gradient.add_key(0.9, Vec2::splat(0.5));
    size_gradient.add_key(1.0, Vec2::splat(0.0));

    let effect_handle = effects.add(
        EffectAsset {
            name: "DustEffect".to_string(),
            capacity: 4096,
            spawner: Spawner::once((radius * 128.0).into(), true),
            ..default()
        }
        .init(PositionCircleModifier {
            center: Vec3::ZERO,
            axis: Vec3::Y,
            radius,
            speed: Value::Uniform((0.0, 0.01)),
            dimension: ShapeDimension::Volume,
        })
        .init(ParticleLifetimeModifier {
            lifetime: 100.0,
        })
        .render(ParticleTextureModifier {
            texture: texture_handle.clone(),
        })
        .render(ColorOverLifetimeModifier {
            gradient: color_gradient,
        })
        .render(SizeOverLifetimeModifier {
            gradient: size_gradient,
        }),
    );

    commands
        .spawn()
        .insert_bundle((DustModel, effect_handle.clone_weak()))
        .insert_bundle(ParticleEffectBundle::new(effect_handle))
        .id()
}

fn setup_dust_model_system(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    let texture_handle: Handle<Image> = asset_server.load("textures/cloud.png");
    let entity = commands
        .spawn()
        .insert(PrimaryStar)
        .insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.2 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            ..default()
        }).id();

    for i in 0..20 {
        let bound = (i * 5) as f32;
        let child_effect = create_dust_effect(&mut commands, &mut effects, bound, &texture_handle);
        commands.entity(entity).add_child(child_effect);
    }
}

fn update_primary_star_system(mut primary_star_query: Query<&mut Transform, With<PrimaryStar>>) {
    let mut primary_star_transform = primary_star_query
        .get_single_mut()
        .expect("Failed to get PrimaryStar Transform");
    primary_star_transform.rotation *= Quat::from_rotation_y(-0.1);
}

fn update_force_fields(mut effects: ResMut<Assets<EffectAsset>>, dust_query: Query<&Handle<EffectAsset>, With<DustModel>>, planets_query: Query<(&PlanetData, &PlanetPosition, &Visibility)>) {
    dust_query.for_each(|effect| {
        let effect_asset = effects.get_mut(effect).expect("Failed to get effect asset");
        let mut force_fields = vec![];
        planets_query.for_each(|(_, planet_position, visibility)| {
            if visibility.is_visible {
                force_fields.push(
                    ForceFieldParam {
                        position: planet_position.0,
                        max_radius: 1.0,
                        min_radius: 0.01,
                        mass: 10.0,
                        force_exponent: 2.0,
                        conform_to_sphere: true,
                    }
                );
            }
        });
        force_fields.truncate(16);
        let modifier = ForceFieldModifier::new(force_fields);
        effect_asset.update_layout.force_field = modifier.force_field;
    });
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
            .add_system(update_force_fields)
            .add_system(update_primary_star_system);
       
    }
}


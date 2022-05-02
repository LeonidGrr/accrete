use bevy::math::vec3;
use bevy::prelude::*;
use bevy::render::{render_resource::WgpuFeatures, settings::WgpuSettings};
use bevy_hanabi::*;

use crate::planet_model::PlanetPosition;

// fn setup(
//     mut commands: Commands,
//     mut effects: ResMut<Assets<EffectAsset>>,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {

//     let mut gradient = Gradient::new();
//     gradient.add_key(0.0, Vec4::new(0.0, 1.0, 1.0, 1.0));
//     gradient.add_key(1.0, Vec4::new(0.0, 1.0, 1.0, 0.0));

//     let spawner = Spawner::once(30.0.into(), false);

//     // Force field effects
//     let effect = effects.add(
//         EffectAsset {
//             name: "Impact".into(),
//             capacity: 32768,
//             spawner,
//             ..default()
//         }
//         .init(PositionSphereModifier {
//             radius: BALL_RADIUS,
//             speed: Value::Uniform((0.1, 0.3)),
//             dimension: ShapeDimension::Surface,
//             ..default()
//         })
//         .update(bevy_hanabi::ForceFieldModifier::new(vec![
//             ForceFieldParam {
//                 position: attractor1_position,
//                 max_radius: 1000000.0,
//                 min_radius: BALL_RADIUS * 6.0,
//                 mass: 3.0,
//                 // quadratic force: proportional to 1 / distance^2
//                 force_exponent: 2.0,
//                 conform_to_sphere: true,
//             },
//         ]))
//         .render(SizeOverLifetimeModifier {
//             gradient: Gradient::constant(Vec2::splat(0.05)),
//         })
//         .render(ColorOverLifetimeModifier { gradient }),
//     );

//     commands.spawn_bundle(ParticleEffectBundle::new(effect).with_spawner(spawner));
// }

#[derive(Debug, Component)]
pub struct PrimaryStar;

fn setup_dust_model(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::splat(1.0));
    gradient.add_key(1.0, Vec4::splat(1.0));

    let effect = effects.add(
        EffectAsset {
            capacity: 32768,
            spawner: Spawner::once(32768.0.into(), true),
            ..default()
        }
        .init(PositionCircleModifier {
            center: Vec3::ZERO,
            axis: Vec3::Y,
            radius: 1.0,
            speed: Value::Single(1.0),
            dimension: ShapeDimension::Surface,
        })
        .update(bevy_hanabi::ForceFieldModifier::new(vec![
            ForceFieldParam {
                position: vec3(10.0, 0.0, 10.0),
                max_radius: 1024.0,
                min_radius: 1.0,
                mass: 300.0,
                // quadratic force: proportional to 1 / distance^2
                force_exponent: 2.0,
                conform_to_sphere: true,
            },
        ]))
        .render(ColorOverLifetimeModifier { gradient })
        .render(SizeOverLifetimeModifier {
            gradient: Gradient::constant([0.1; 2].into()),
        }),
    );

    commands
        .spawn()
        .insert(PrimaryStar)
        .insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.2 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            ..default()
        })
        .with_children(|p| {
            p.spawn()
                .insert_bundle(ParticleEffectBundle::new(effect))
                .insert(Name::new("DustModel"));
        });
}

fn update_primary_star(mut primary_star_query: Query<&mut Transform, With<PrimaryStar>>) {
    let mut primary_star_transform = primary_star_query
        .get_single_mut()
        .expect("Failed to get PrimaryStar Transform");
    primary_star_transform.rotation *= Quat::from_rotation_y(-0.1);
}

fn update_dust_model(
    mut effect_query: Query<&mut GlobalTransform, With<ParticleEffect>>,
    planets_query: Query<(&PlanetPosition, &Visibility)>,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    let mut dust_transform = effect_query
        .get_single_mut()
        .expect("Failed to get ParticleEffect Transform");
    dust_transform.rotation *= Quat::from_rotation_y(-0.1);

    // let effect_handle = effect_query.single();
    let effect = effects.iter_mut().fold(None, |mut acc, (_, asset)| {
        acc = Some(asset);
        acc
    });
    // if let Ok(effect_handle) = effect_handle {
    //     let mut effect = effects
    //         .get_mut(effect_handle)
    //         .expect("Failed to find DustModel effects");

    // if let Some(effect) = effect {
    //     let mut next_force_fields = vec![];
    //     for (planet_position, visibility) in planets_query.iter() {
    //         if visibility.is_visible {
    //             next_force_fields.push(ForceFieldParam {
    //                 position: planet_position.0,
    //                 max_radius: 100.0,
    //                 min_radius: 1.0,
    //                 mass: 30.0,
    //                 // quadratic force: proportional to 1 / distance^2
    //                 force_exponent: 2.0,
    //                 conform_to_sphere: false,
    //             });
    //         }
    //     }

    //     let modifier = bevy_hanabi::ForceFieldModifier::new(next_force_fields);
    //     effect.update_layout.force_field = modifier.force_field;
    // }
        
    // }
}
// fn setup2(
//     mut commands: Commands,
//     mut effects: ResMut<Assets<EffectAsset>>,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     let cube = meshes.add(Mesh::from(Cube { size: 10.0 }));
//     let mat = materials.add(Color::PURPLE.into());

//     let mut gradient2 = Gradient::new();
//     gradient2.add_key(0.0, Vec4::new(0.0, 0.0, 1.0, 1.0));
//     gradient2.add_key(1.0, Vec4::splat(0.0));

//     let effect2 = effects.add(
//         EffectAsset {
//             name: "emit:once".to_string(),
//             capacity: 32768,
//             spawner: Spawner::once(1000.0.into(), true),
//             ..default()
//         }
//         .render(ColorOverLifetimeModifier {
//             gradient: gradient2,
//         }),
//     );

//     commands
//         .spawn()
//         .insert(Name::new("emit:once"))
//         .insert_bundle(ParticleEffectBundle {
//             effect: ParticleEffect::new(effect2),
//             transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
//             ..default()
//         })
//         .with_children(|p| {
//             // Reference cube to visualize the emit origin
//             p.spawn()
//                 .insert_bundle(PbrBundle {
//                     mesh: cube.clone(),
//                     material: mat.clone(),
//                     ..default()
//                 })
//                 .insert(Name::new("source"));
//         });

// let mut gradient3 = Gradient::new();
// gradient3.add_key(0.0, Vec4::new(0.0, 0.0, 1.0, 1.0));
// gradient3.add_key(1.0, Vec4::splat(0.0));

// let effect3 = effects.add(
//     EffectAsset {
//         name: "emit:burst".to_string(),
//         capacity: 32768,
//         spawner: Spawner::burst(400.0.into(), 3.0.into()),
//         ..default()
//     }
//     .init(PositionSphereModifier {
//         center: Vec3::ZERO,
//         radius: 5.,
//         dimension: ShapeDimension::Volume,
//         speed: 2.0.into(),
//     })
//     .update(AccelModifier {
//         accel: Vec3::new(0., 5., 0.),
//     })
//     .render(ColorOverLifetimeModifier {
//         gradient: gradient3,
//     }),
// );

// commands
//     .spawn()
//     .insert(Name::new("emit:burst"))
//     .insert_bundle(ParticleEffectBundle {
//         effect: ParticleEffect::new(effect3),
//         transform: Transform::from_translation(Vec3::new(30., 0., 0.)),
//         ..default()
//     })
//     .with_children(|p| {
//         // Reference cube to visualize the emit origin
//         p.spawn()
//             .insert_bundle(PbrBundle {
//                 mesh: cube.clone(),
//                 material: mat.clone(),
//                 ..default()
//             })
//             .insert(Name::new("source"));
//     });
// }

pub struct DustPlugin;

impl Plugin for DustPlugin {
    fn build(&self, app: &mut App) {
        let mut options = WgpuSettings::default();
        options
            .features
            .set(WgpuFeatures::VERTEX_WRITABLE_STORAGE, true);

        app.insert_resource(options)
            .add_plugin(HanabiPlugin)
            .add_startup_system(setup_dust_model)
            .add_system(update_primary_star)
            .add_system(update_dust_model);
    }
}

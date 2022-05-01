use bevy::prelude::*;
use bevy::render::{render_resource::WgpuFeatures, settings::WgpuSettings};
use bevy_hanabi::*;

// fn setup(
//     mut commands: Commands,
//     mut effects: ResMut<Assets<EffectAsset>>,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {


//     let attractor1_position = Vec3::new(0.01, 0.0, 0.0);


//     commands.spawn_bundle(PbrBundle {
//         mesh: meshes.add(Mesh::from(shape::UVSphere {
//             sectors: 128,
//             stacks: 4,
//             radius: BALL_RADIUS * 2.0,
//         })),
//         material: materials.add(StandardMaterial {
//             base_color: Color::YELLOW,
//             unlit: false,
//             ..default()
//         }),
//         transform: Transform::from_translation(attractor1_position),
//         ..default()
//     });



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
            speed: Value::Uniform((1.0, 25.0)),
            dimension: ShapeDimension::Surface,
        })
        // .update(bevy_hanabi::ForceFieldModifier::new(vec![
        //     ForceFieldParam {
        //         position: Vec3::ZERO,
        //         max_radius: 1024.0,
        //         min_radius: 1.0,
        //         mass: 3.0,
        //         // quadratic force: proportional to 1 / distance^2
        //         force_exponent: 2.0,
        //         conform_to_sphere: true,
        //     },
        // ]))
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

fn update_dust_model(mut query: Query<&mut Transform, With<PrimaryStar>>) {
    let mut transform = query
        .get_single_mut()
        .expect("Failed to get PrimaryStar Transform");
    transform.rotation *= Quat::from_rotation_y(-0.1);
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
            .add_system(update_dust_model);
    }
}
